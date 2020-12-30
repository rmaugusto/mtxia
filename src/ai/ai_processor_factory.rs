use super::{nn_manual_relu::NN_Manual_Relu, AiProcessor};

pub fn create_ai_processor() -> Box<dyn AiProcessor + Sync + Send + 'static> {
    return Box::new(NN_Manual_Relu::create()) as Box<dyn AiProcessor + Sync + Send + 'static>;
}
