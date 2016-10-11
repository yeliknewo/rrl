use rand::{Rng, thread_rng};
use std::vec::Drain;

pub type LayerSizeType = u16;

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct NeuralNetwork {
    layers: Vec<NeuralLayer>,
}

impl NeuralNetwork {
    pub fn sigmoid_inputs(inputs: &mut Vec<f64>) {
        for input in inputs {
            *input = Neuron::sigmoid(*input);
        }
    }

    pub fn new(weights_and_biases: Vec<Vec<Vec<f64>>>) -> NeuralNetwork {
        let mut out = NeuralNetwork { layers: vec![] };
        for layer in weights_and_biases {
            out.layers.push(NeuralLayer::new(layer));
        }
        out
    }

    pub fn new_random(first_input_size: LayerSizeType,
                      layer_sizes: &Vec<LayerSizeType>,
                      min_weight: f64,
                      max_weight: f64,
                      min_bias: f64,
                      max_bias: f64)
                      -> NeuralNetwork {
        let mut layers = vec![];

        let mut input_size = first_input_size;

        for layer_size in layer_sizes {
            layers.push(NeuralLayer::new_random(input_size,
                                                *layer_size,
                                                min_weight,
                                                max_weight,
                                                min_bias,
                                                max_bias));
            input_size = *layer_size;
        }

        NeuralNetwork { layers: layers }
    }

    pub fn fire(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut inputs = inputs.to_vec();

        for layer in &self.layers {
            inputs = layer.fire(&inputs);
        }

        inputs
    }

    pub fn get_weights_and_bias(&self) -> Vec<Vec<Vec<f64>>> {
        let mut output = vec![];

        for layer in &self.layers {
            output.push(layer.get_weights_and_bias());
        }

        output
    }

    pub fn set_weights_and_bias(&mut self, mut weights_and_biases: Vec<Vec<Vec<f64>>>) {
        assert_eq!(weights_and_biases.len(), self.layers.len());

        let mut wab_iter: Drain<Vec<Vec<f64>>> = weights_and_biases.drain(..);

        for layer in &mut self.layers {
            layer.set_weights_and_bias(wab_iter.next()
                .unwrap_or_else(|| panic!("Wab iter next was none")));
        }
    }
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct NeuralLayer {
    neurons: Vec<Neuron>,
}

impl NeuralLayer {
    fn new(weights_and_biases: Vec<Vec<f64>>) -> NeuralLayer {
        let mut out = NeuralLayer { neurons: vec![] };
        for neuron in weights_and_biases {
            out.neurons.push(Neuron::new(neuron));
        }
        out
    }

    fn new_random(num_inputs: LayerSizeType,
                  num_outputs: LayerSizeType,
                  min_weight: f64,
                  max_weight: f64,
                  min_bias: f64,
                  max_bias: f64)
                  -> NeuralLayer {
        let mut neurons = vec![];

        for _ in 0..num_outputs {
            neurons.push(Neuron::new_random(num_inputs, min_weight, max_weight, min_bias, max_bias));
        }

        NeuralLayer { neurons: neurons }
    }

    fn fire(&self, inputs: &Vec<f64>) -> Vec<f64> {
        let mut outputs = vec![];
        for neuron in &self.neurons {
            outputs.push(neuron.fire(&inputs));
        }

        outputs
    }

    fn get_weights_and_bias(&self) -> Vec<Vec<f64>> {
        let mut output = vec![];

        for neuron in &self.neurons {
            output.push(neuron.get_weights_and_bias());
        }

        output
    }

    fn set_weights_and_bias(&mut self, mut weights_and_biases: Vec<Vec<f64>>) {
        assert_eq!(weights_and_biases.len(), self.neurons.len());

        let mut wab_iter: Drain<Vec<f64>> = weights_and_biases.drain(..);

        for neuron in &mut self.neurons {
            neuron.set_weights_and_bias(wab_iter.next()
                .unwrap_or_else(|| panic!("Wab Iter was different length than neurons")));
        }
    }
}

#[derive(Debug, Clone, RustcEncodable, RustcDecodable)]
pub struct Neuron {
    last_weight_count: usize,
    weights: Vec<f64>,
    bias: f64,
}

impl Neuron {
    fn new(weights_and_bias: Vec<f64>) -> Neuron {
        let mut out = Neuron {
            last_weight_count: weights_and_bias.len() - 1,
            weights: vec![],
            bias: 0.0,
        };
        out.set_weights_and_bias(weights_and_bias);
        out
    }

    fn new_random(num_inputs: LayerSizeType,
                  min_weight: f64,
                  max_weight: f64,
                  min_bias: f64,
                  max_bias: f64)
                  -> Neuron {
        let mut weights = vec![];

        let mut rng = thread_rng();

        // warn!("Num Inputs: {:?}", num_inputs);
        for _ in 0..num_inputs {
            weights.push(rng.gen_range(min_weight, max_weight));
        }

        let bias = rng.gen_range(min_bias, max_bias);

        Neuron {
            last_weight_count: weights.len(),
            weights: weights,
            bias: bias,
        }
    }

    fn sigmoid(input: f64) -> f64 {
        1.0 / (1.0 + (-input).exp())
    }

    fn fire(&self, inputs: &Vec<f64>) -> f64 {
        assert_eq!(inputs.len(), self.weights.len());
        let mut sum = self.bias;
        for index in 0..inputs.len() {
            sum += inputs.get(index).unwrap_or_else(|| panic!("Input get Index was none")) *
                   *self.weights.get(index).unwrap_or_else(|| panic!("Weights get Index was none"));
        }

        Neuron::sigmoid(sum)
    }

    fn get_weights(&self) -> &Vec<f64> {
        &self.weights
    }

    fn get_bias(&self) -> f64 {
        self.bias
    }

    fn get_weights_and_bias(&self) -> Vec<f64> {
        let mut combo = self.get_weights().to_vec();
        combo.push(self.get_bias());
        combo
    }

    fn set_weights(&mut self, weights: Vec<f64>) {
        assert_eq!(weights.len(), self.last_weight_count);
        self.weights = weights;
        self.last_weight_count = self.weights.len();
    }

    fn set_bias(&mut self, bias: f64) {
        self.bias = bias;
    }

    fn set_weights_and_bias(&mut self, mut weights_and_bias: Vec<f64>) {
        self.set_bias(weights_and_bias.pop()
            .unwrap_or_else(|| panic!("Weights and Bias was empty")));
        self.set_weights(weights_and_bias);
    }
}
