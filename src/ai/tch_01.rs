use std::collections::HashMap;

use tch::{
    nn::{self, LinearConfig, Module},
    Device, Tensor,
};

use super::AiProcessor;

fn net(vs: &nn::Path, input_neurons: usize, hidden_neurons: usize) -> impl Module {
    nn::seq()
        .add(nn::linear(
            vs,
            input_neurons as i64,
            hidden_neurons as i64,
            LinearConfig {
                bias: true,
                ..Default::default()
            },
        ))
        .add_fn(|xs| xs.relu())
        .add(nn::linear(
            vs,
            hidden_neurons as i64,
            2 as i64,
            LinearConfig {
                bias: true,
                ..Default::default()
            },
        ))
        .add_fn(|xs| xs.relu())
}

pub struct Tch01 {
    config: HashMap<String, String>,
    vs: nn::VarStore,
    input_neurons: usize,
    hidden_neurons: usize,
}

impl Tch01 {
    pub fn create(_config: HashMap<String, String>) -> Self {
        let _input_neurons: usize = _config
            .get("input_neurons")
            .unwrap_or(&String::from("1"))
            .parse()
            .expect("Error parsing");

        let _hidden_neurons: usize = _config
            .get("hidden_neurons")
            .unwrap_or(&String::from("1"))
            .parse()
            .expect("Error parsing");

        let _vs = nn::VarStore::new(Device::cuda_if_available());

        Tch01 {
            config: _config,
            vs: _vs,
            input_neurons: _input_neurons,
            hidden_neurons: _hidden_neurons,
        }
    }
}

impl AiProcessor for Tch01 {
    fn nn_forward(&mut self, input: Vec<f64>) -> Vec<bool> {
        let net = net(&self.vs.root(), self.input_neurons, self.hidden_neurons);

        let input_tensor = Tensor::of_slice(&input).to_kind(tch::Kind::Float);
        let res = net.forward(&input_tensor);

        vec![res.double_value(&[0]) > 0f64, res.double_value(&[1]) > 0f64]
    }

    fn save(&mut self, path: &std::path::Path) {
        self.vs.save(path).expect("Error saving state NN");
    }

    fn load(&mut self, path: &std::path::Path) {
        self.vs.load(path).expect("Error loading state NN");
    }

    fn get_name(&self) -> String {
        String::from("Tch01")
    }

    fn random_weights(&mut self) {}

    fn modify_random_weights(&mut self) {}
}
