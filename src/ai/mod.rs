use std::path::Path;

pub mod ai_plugin;
pub mod ai_processor_factory;
mod creation_system;
mod forward_system;
pub mod nn_manual_relu;

pub const BEST_RESULTS_SAVED_COUNT: i32 = 5;

pub enum AiAction {
    MOVE_LEFT,
    MOVE_RIGHT
}
pub trait AiProcessor {
    fn forward(&mut self, input: Vec<f64>) -> Vec<bool>;
    fn save(&mut self, path: &Path);
    fn load(&mut self, path: &Path);
    fn get_name(&self) -> String;
    fn random_weights(&mut self);
    fn modify_random_weights(&mut self);
}

pub struct FishAi {
}
