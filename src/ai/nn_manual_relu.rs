use std::fs;

use rand::{seq::IteratorRandom, thread_rng, Rng};

use serde::{Deserialize, Serialize};

use super::{AiAction, AiProcessor};

const BIAS: usize = 1;
const HIDDEN_LAYERS: usize = 1;
const INPUT_NEURONS: usize = 13;
const HIDDEN_NEUROS: usize = 13;
const OUTPUT_NEUROS: usize = 2;
const INITIAL_WEIGHT_RATE: f64 = 1f64;

#[derive(Serialize, Deserialize)]
struct Neuron {
    weight: Vec<f64>,
    output: f64,
}

impl Neuron {
    fn random_number() -> f64 {
        let mut rng = rand::thread_rng();
        rng.gen_range(0f64, 20_001f64) / 10f64 - 1_000f64
    }

    fn create(weight_size: usize) -> Self {
        Neuron {
            weight: (0..weight_size).map(|_| 0f64).collect(),
            output: 1f64,
        }
    }

    pub fn random_weights(&mut self) {
        self.weight.iter_mut().for_each(|w| {
            *w = Neuron::random_number();
        });
    }

    pub fn multiply_random_weights(&mut self) {
        let mut rng = rand::thread_rng();
        self.weight.iter_mut().for_each(|w| {
            let rnd_number = rng.gen_range(0f64, 10_001f64) / 10_000f64 + 0.5f64;
            *w = *w * rnd_number;
        });
    }

    pub fn add_random_weights(&mut self) {
        self.weight.iter_mut().for_each(|w| {
            let rnd_number = Neuron::random_number() / 100f64;
            *w = *w + rnd_number;
        });
    }
}

#[derive(Serialize, Deserialize)]
struct Layer {
    neurons: Vec<Neuron>,
}

impl Layer {
    fn create(neurons_size: usize, weight_size: usize) -> Self {
        Layer {
            neurons: (0..neurons_size)
                .map(|_| Neuron::create(weight_size))
                .collect(),
        }
    }
}
#[derive(Serialize, Deserialize)]
struct NeuralNetwork {
    input: Layer,
    hidden_layers: Vec<Layer>,
    output: Layer,
}

impl NeuralNetwork {
    fn create(
        input_size: usize,
        hidden_layers: usize,
        hidden_size: usize,
        output_size: usize,
    ) -> Self {
        NeuralNetwork {
            input: Layer::create(input_size, input_size),
            hidden_layers: (0..hidden_layers)
                .map(|_| Layer::create(hidden_size, hidden_size))
                .collect(),
            output: Layer::create(output_size, hidden_size),
        }
    }
}

fn relu(x: f64) -> f64 {
    if x < 0f64 {
        return 0f64;
    } else {
        if x < 10000f64 {
            return x;
        } else {
            return 10000f64;
        }
    }
}
pub struct NN_Manual_Relu {
    nn: NeuralNetwork,
}

impl<'a> NN_Manual_Relu {
    pub fn create() -> Self {
        let mut _nn = NeuralNetwork::create(
            INPUT_NEURONS + BIAS,
            HIDDEN_LAYERS,
            HIDDEN_NEUROS + BIAS,
            OUTPUT_NEUROS,
        );

        NN_Manual_Relu { nn: _nn }
    }
}

impl AiProcessor for NN_Manual_Relu {
    fn forward(&mut self, input: Vec<f64>) -> Vec<bool> {
        //     //Input less BIAS
        if self.nn.input.neurons.len() - BIAS != input.len() {
            panic!("Size of input must equals size of Neurons - BIAS")
        }

        if self.nn.hidden_layers.len() == 0 {
            panic!("Hidden layer must be more then zero")
        }

        input
            .iter()
            .enumerate()
            .for_each(|(i, n)| self.nn.input.neurons[i].output = input[i]);

        let mut sum = 0f64;
        //Calculate from input to first hidden layers
        for i in 0..self.nn.hidden_layers[0].neurons.len() - BIAS {
            sum = 0f64;
            for j in 0..self.nn.input.neurons.len() - BIAS {
                sum +=
                    self.nn.input.neurons[j].output * self.nn.hidden_layers[0].neurons[i].weight[j];
            }
            self.nn.hidden_layers[0].neurons[i].output = relu(sum);
        }

        //Calculate intermediate hidden layers
        for k in 1..self.nn.hidden_layers.len() {
            for i in 0..self.nn.hidden_layers[k].neurons.len() - BIAS {
                sum = 0f64;
                for j in 0..self.nn.hidden_layers[k - 1].neurons.len() - BIAS {
                    sum += self.nn.hidden_layers[k - 1].neurons[j].output
                        * self.nn.hidden_layers[k].neurons[i].weight[j];
                }
                self.nn.hidden_layers[0].neurons[i].output = relu(sum);
            }
        }

        //Calculate last hidden layer to output layet
        let last_hidden_layer = self.nn.hidden_layers.last().unwrap();
        for i in 0..self.nn.output.neurons.len() {
            sum = 0f64;
            for j in 0..last_hidden_layer.neurons.len() {
                sum += last_hidden_layer.neurons[j].output * self.nn.output.neurons[i].weight[j];
            }
            self.nn.output.neurons[i].output = relu(sum);
        }

        //Group output
        return self
            .nn
            .output
            .neurons
            .iter()
            .map(|n| n.output > 0f64)
            .collect();
    }

    fn save(&mut self, path: &std::path::Path) {
        let as_str = serde_yaml::to_string(&self.nn).expect("Error serializing");
        fs::write(path, &as_str).expect("Unable to write file");
    }

    fn load(&mut self, path: &std::path::Path) {
        let contents = fs::read_to_string(path).expect("File not found ");
        self.nn = serde_yaml::from_str(&contents).expect("Error parsing yaml");
    }

    fn get_name(&self) -> String {
        "NN_Manual_Relu".into()
    }

    fn random_weights(&mut self) {
        self.nn
            .input
            .neurons
            .iter_mut()
            .for_each(|n| n.random_weights());

        self.nn
            .hidden_layers
            .iter_mut()
            .for_each(|l| l.neurons.iter_mut().for_each(|n| n.random_weights()));

        self.nn
            .output
            .neurons
            .iter_mut()
            .for_each(|n| n.random_weights());
    }

    fn modify_random_weights(&mut self) {
        let mut rng_layers = thread_rng();
        let mut rng_neurons = thread_rng();

        //Take n_neurons in layer to randomize completely
        let n_neurons = rng_neurons.gen_range(0, &self.nn.input.neurons.len());
        self.nn
            .input
            .neurons
            .iter_mut()
            .choose_multiple(&mut rng_layers, n_neurons)
            .iter_mut()
            .for_each(|n| n.random_weights());

        // Apply to all hidden layers
        self.nn.hidden_layers.iter_mut().for_each(|l| {
            //Take n_neurons in layer to multiply random weights
            let n_neurons = rng_neurons.gen_range(0, &l.neurons.len());
            l.neurons
                .iter_mut()
                .choose_multiple(&mut rng_layers, n_neurons)
                .iter_mut()
                .for_each(|n| n.multiply_random_weights())
        });

        //Take n_neurons in layer to add random weights
        let n_neurons = rng_neurons.gen_range(0, &self.nn.output.neurons.len());
        self.nn
            .output
            .neurons
            .iter_mut()
            .choose_multiple(&mut rng_layers, n_neurons)
            .iter_mut()
            .for_each(|n| n.add_random_weights());
    }
}
