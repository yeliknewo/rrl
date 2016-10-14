use network::NeuralNetwork;
use num::{Float, FromPrimitive};
use rand::{Rng, thread_rng};
use rand::distributions::range::SampleRange;
use std::collections::HashMap;
use std::vec::Drain as VecDrain;

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct EvolutionaryTrainer<S: Ord + Clone, W: Float + SampleRange + FromPrimitive> {
    next_generation: HashMap<usize, NeuralNetwork<W>>,
    generation: Vec<Species<S, W>>,
}

impl<S: Ord + Clone, W: SampleRange + Float + FromPrimitive> EvolutionaryTrainer<S, W> {
    pub fn new(first_generation: HashMap<usize, NeuralNetwork<W>>) -> EvolutionaryTrainer<S, W> {
        EvolutionaryTrainer {
            next_generation: first_generation,
            generation: vec![],
        }
    }

    pub fn train<F: Fn() -> W>(&mut self,
                               mut rewards: HashMap<usize, S>,
                               mutation_mult_picker: &F,
                               mutation_add_picker: &F) {
        assert_eq!(self.next_generation.len(),
                   rewards.len());

        let drop_count = 4;

        for reward in rewards.drain() {
            // warn!("Reward Index: {:?}", reward.0);
            self.generation.push(Species::new(reward.1,
                                              self.next_generation
                                                  .remove(&reward.0)
                                                  .unwrap_or_else(|| panic!("Next Gen remove reward.o was none"))));
        }

        self.generation.sort_by_key(|species| species.get_fitness());

        let first = self.generation.pop().unwrap_or_else(|| panic!("Generation pop was none"));
        let second = self.generation.pop().unwrap_or_else(|| panic!("Generation pop was none"));

        for _ in 0..drop_count {
            let size = self.generation.len() - 1;
            self.generation.remove(size);
        }

        for _ in 0..(2 + drop_count) {
            self.add_to_next_gen(first.cross(&second,
                                             mutation_mult_picker,
                                             mutation_add_picker));
        }

        let mut delay = vec![];

        for species in self.generation.drain(..) {
            delay.push(first.cross(&species,
                                   mutation_mult_picker,
                                   mutation_add_picker));
        }

        for net in delay.drain(..) {
            self.add_to_next_gen(net);
        }
    }

    fn add_to_next_gen(&mut self,
                       net: NeuralNetwork<W>) {
        let index = self.next_generation.len();
        self.next_generation.insert(index,
                                    net);
    }

    pub fn get_next_generation(&self) -> &HashMap<usize, NeuralNetwork<W>> {
        &self.next_generation
    }
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
struct Species<S: Clone, W: SampleRange + Float + FromPrimitive> {
    fitness: S,
    network: NeuralNetwork<W>,
}

impl<S: Clone, W: SampleRange + Float + FromPrimitive> Species<S, W> {
    fn new(fitness: S,
           network: NeuralNetwork<W>)
           -> Species<S, W> {
        Species {
            fitness: fitness,
            network: network,
        }
    }

    fn get_fitness(&self) -> S {
        self.fitness.clone()
    }

    fn get_network(&self) -> &NeuralNetwork<W> {
        &self.network
    }

    fn cross<F: Fn() -> W>(&self,
                           other: &Species<S, W>,
                           mutation_mult_picker: F,
                           mutation_add_picker: F)
                           -> NeuralNetwork<W> {
        let mut net_weights_1 = self.get_network().get_weights_and_bias();
        let mut net_weights_2 = other.get_network().get_weights_and_bias();
        assert_eq!(net_weights_1.len(),
                   net_weights_2.len());

        let net_iter_1: VecDrain<Vec<Vec<W>>> = net_weights_1.drain(..);
        let mut net_iter_2: VecDrain<Vec<Vec<W>>> = net_weights_2.drain(..);

        let mut child_net = vec![];

        let mut rng = thread_rng();

        for mut layer_weights_1 in net_iter_1 {
            let mut layer_weights_2 = net_iter_2.next()
                .unwrap_or_else(|| panic!("Net 2 iter next was none"));
            assert_eq!(layer_weights_1.len(),
                       layer_weights_2.len());

            let mut child_layer = vec![];

            let layer_iter_1: VecDrain<Vec<W>> = layer_weights_1.drain(..);
            let mut layer_iter_2: VecDrain<Vec<W>> = layer_weights_2.drain(..);

            for mut neuron_weights_1 in layer_iter_1 {
                let mut neuron_weights_2 = layer_iter_2.next()
                    .unwrap_or_else(|| panic!("Layer iter 2 next was none"));
                assert_eq!(neuron_weights_1.len(),
                           neuron_weights_2.len());

                let mut child_neuron = vec![];

                let neuron_iter_1: VecDrain<W> = neuron_weights_1.drain(..);
                let mut neuron_iter_2: VecDrain<W> = neuron_weights_2.drain(..);

                for weight_1 in neuron_iter_1 {
                    let weight_2 = neuron_iter_2.next()
                        .unwrap_or_else(|| panic!("Neuron iter 2 next was none"));

                    let mutation_mult = mutation_mult_picker();

                    let mutation_add = mutation_add_picker();

                    child_neuron.push(*rng.choose(&[weight_1, weight_2])
                        .unwrap_or_else(|| panic!("Not fucking possible")) * mutation_mult + mutation_add);
                }

                child_layer.push(child_neuron);
            }
            child_net.push(child_layer);
        }
        NeuralNetwork::new(child_net)
    }
}
