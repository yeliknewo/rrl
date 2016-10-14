use num::{Float, FromPrimitive, Num, ToPrimitive};
use num::iter::range;
use rand::{Rng, thread_rng};
use rand::distributions::range::SampleRange;
use std::vec::Drain;

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct NeuralNetwork<W: Float + SampleRange + FromPrimitive> {
    layers: Vec<NeuralLayer<W>>,
}

impl<W: Float + SampleRange + FromPrimitive> NeuralNetwork<W> {
    pub fn sigmoid_inputs(inputs: &mut Vec<W>) {
        for input in inputs {
            *input = Neuron::sigmoid(*input);
        }
    }

    pub fn new(weights_and_biases: Vec<Vec<Vec<W>>>) -> NeuralNetwork<W> {
        let mut out = NeuralNetwork {
            layers: vec![],
        };
        for layer in weights_and_biases {
            out.layers.push(NeuralLayer::new(layer));
        }
        out
    }

    pub fn new_random<S: Num + PartialOrd + Clone + ToPrimitive>(first_input_size: S,
                                                                 layer_sizes: &Vec<S>,
                                                                 min_weight: W,
                                                                 max_weight: W,
                                                                 min_bias: W,
                                                                 max_bias: W)
                                                                 -> NeuralNetwork<W> {
        let mut layers = vec![];

        let mut input_size = first_input_size;

        for layer_size in layer_sizes {
            layers.push(NeuralLayer::new_random(input_size,
                                                layer_size.clone(),
                                                min_weight,
                                                max_weight,
                                                min_bias,
                                                max_bias));
            input_size = layer_size.clone();
        }

        NeuralNetwork {
            layers: layers,
        }
    }

    pub fn fire(&self,
                inputs: &Vec<W>)
                -> Vec<W> {
        let mut inputs = inputs.to_vec();

        for layer in &self.layers {
            inputs = layer.fire(&inputs);
        }

        inputs
    }

    pub fn get_weights_and_bias(&self) -> Vec<Vec<Vec<W>>> {
        let mut output = vec![];

        for layer in &self.layers {
            output.push(layer.get_weights_and_bias());
        }

        output
    }

    pub fn set_weights_and_bias(&mut self,
                                mut weights_and_biases: Vec<Vec<Vec<W>>>) {
        assert_eq!(weights_and_biases.len(),
                   self.layers.len());

        let mut wab_iter: Drain<Vec<Vec<W>>> = weights_and_biases.drain(..);

        for layer in &mut self.layers {
            layer.set_weights_and_bias(wab_iter.next()
                .unwrap_or_else(|| panic!("Wab iter next was none")));
        }
    }
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct NeuralLayer<W: Float + SampleRange + FromPrimitive> {
    neurons: Vec<Neuron<W>>,
}

impl<W: SampleRange + Float + FromPrimitive> NeuralLayer<W> {
    fn new(weights_and_biases: Vec<Vec<W>>) -> NeuralLayer<W> {
        let mut out = NeuralLayer {
            neurons: vec![],
        };
        for neuron in weights_and_biases {
            out.neurons.push(Neuron::new(neuron));
        }
        out
    }

    fn new_random<S: Num + PartialOrd + Clone + ToPrimitive>(num_inputs: S,
                                                             num_outputs: S,
                                                             min_weight: W,
                                                             max_weight: W,
                                                             min_bias: W,
                                                             max_bias: W)
                                                             -> NeuralLayer<W> {
        let mut neurons = vec![];

        for _ in range(S::zero(),
                       num_outputs) {
            neurons.push(Neuron::new_random(num_inputs.clone(),
                                            min_weight,
                                            max_weight,
                                            min_bias,
                                            max_bias));
        }

        NeuralLayer {
            neurons: neurons,
        }
    }

    fn fire(&self,
            inputs: &Vec<W>)
            -> Vec<W> {
        let mut outputs = vec![];
        for neuron in &self.neurons {
            outputs.push(neuron.fire(&inputs));
        }

        outputs
    }

    fn get_weights_and_bias(&self) -> Vec<Vec<W>> {
        let mut output = vec![];

        for neuron in &self.neurons {
            output.push(neuron.get_weights_and_bias());
        }

        output
    }

    fn set_weights_and_bias(&mut self,
                            mut weights_and_biases: Vec<Vec<W>>) {
        assert_eq!(weights_and_biases.len(),
                   self.neurons.len());

        let mut wab_iter: Drain<Vec<W>> = weights_and_biases.drain(..);

        for neuron in &mut self.neurons {
            neuron.set_weights_and_bias(wab_iter.next()
                .unwrap_or_else(|| panic!("Wab Iter was different length than neurons")));
        }
    }
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Neuron<W: Float + SampleRange + FromPrimitive> {
    last_weight_count: usize,
    weights: Vec<W>,
    bias: W,
}

impl<W: Float + SampleRange + FromPrimitive> Neuron<W> {
    fn new(weights_and_bias: Vec<W>) -> Neuron<W> {
        let mut out = Neuron {
            last_weight_count: weights_and_bias.len() - 1,
            weights: vec![],
            bias: W::zero(),
        };
        out.set_weights_and_bias(weights_and_bias);
        out
    }

    fn new_random<S: Num + PartialOrd + Clone + ToPrimitive>(num_inputs: S,
                                                             min_weight: W,
                                                             max_weight: W,
                                                             min_bias: W,
                                                             max_bias: W)
                                                             -> Neuron<W> {
        let mut weights = vec![];

        let mut rng = thread_rng();

        // warn!("Num Inputs: {:?}", num_inputs);
        for _ in range(S::zero(),
                       num_inputs) {
            weights.push(rng.gen_range(min_weight,
                                       max_weight));
        }

        let bias = rng.gen_range(min_bias,
                                 max_bias);

        Neuron {
            last_weight_count: weights.len(),
            weights: weights,
            bias: bias,
        }
    }

    fn sigmoid(input: W) -> W {
        W::from_f64(2.0).unwrap_or_else(|| panic!("From f64 2.0 was none")) / (W::one() + (-input).exp()) - W::one()
    }

    fn fire(&self,
            inputs: &Vec<W>)
            -> W {
        assert_eq!(inputs.len(),
                   self.weights.len());
        let mut sum = self.bias;
        for index in 0..inputs.len() {
            sum = sum + *inputs.get(index).unwrap_or_else(|| panic!("Input get Index was none")) * *self.weights.get(index).unwrap_or_else(|| panic!("Weights get Index was none"));
        }

        Neuron::sigmoid(sum)
    }

    fn get_weights(&self) -> &Vec<W> {
        &self.weights
    }

    fn get_bias(&self) -> W {
        self.bias
    }

    fn get_weights_and_bias(&self) -> Vec<W> {
        let mut combo = self.get_weights().to_vec();
        combo.push(self.get_bias());
        combo
    }

    fn set_weights(&mut self,
                   weights: Vec<W>) {
        assert_eq!(weights.len(),
                   self.last_weight_count);
        self.weights = weights;
        self.last_weight_count = self.weights.len();
    }

    fn set_bias(&mut self,
                bias: W) {
        self.bias = bias;
    }

    fn set_weights_and_bias(&mut self,
                            mut weights_and_bias: Vec<W>) {
        self.set_bias(weights_and_bias.pop()
            .unwrap_or_else(|| panic!("Weights and Bias was empty")));
        self.set_weights(weights_and_bias);
    }
}
