use crate::shared::config::Config;

use super::{AiProcessor, nf_relu_01::NfRelu01, tch_01::Tch01};

pub fn create_brain(config: &Config) -> Box<dyn AiProcessor + Sync + Send + 'static> {
    let mut comp_conf = config.ai.method.config.clone();

    let mut input_neurons = 0;

    if config
        .ai
        .input_vars
        .contains(&crate::shared::config::InputLayerEnum::SENSORS)
    {
        input_neurons += config.fish.sensors;
    }

    if config
        .ai
        .input_vars
        .contains(&crate::shared::config::InputLayerEnum::SPEED)
    {
        input_neurons += 1;
    }

    if config
        .ai
        .input_vars
        .contains(&crate::shared::config::InputLayerEnum::ENERGY)
    {
        input_neurons += 1;
    }

    if config
        .ai
        .input_vars
        .contains(&crate::shared::config::InputLayerEnum::FITNESS)
    {
        input_neurons += 1;
    }

    if config
        .ai
        .input_vars
        .contains(&crate::shared::config::InputLayerEnum::AGE)
    {
        input_neurons += 1;
    }

    comp_conf.insert(String::from("input_neurons"), input_neurons.to_string());

    if config.ai.method.name == "NfRelu01" {
        return Box::new(NfRelu01::create(comp_conf));
    }else if config.ai.method.name == "Tch01" {
        return Box::new(Tch01::create(comp_conf));
    } else {
        panic!("Ai method not implemented !");
    }
}
