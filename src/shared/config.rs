use std::{
    collections::HashMap,
    fs::{self},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct FishConfig {
    pub count: i32,
    pub sensors: i32,
    pub range: f32,
    pub speed: f32,
    pub energy: f32,
    pub render_sensor: bool,
    pub energy_inc: f32,
    pub energy_to_turn: f32,
    pub energy_to_walk: f32,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub time_speed: f32,
    pub keyboard: bool,
    pub headless: bool,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ModeEnum {
    LEARN,
    RUN,
    DISABLED,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum InputLayerEnum {
    SENSORS,
    SPEED,
    ENERGY,
    FITNESS,
    AGE,
    DISTANCE,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct MethodAiConfig {
    pub name: String,
    pub config: HashMap<String, String>,
}
#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AiConfig {
    pub state_path: String,
    pub mode: ModeEnum,
    pub input_vars: Vec<InputLayerEnum>,
    pub method: MethodAiConfig,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct Config {
    pub fish: FishConfig,
    pub general: GeneralConfig,
    pub ai: AiConfig,
}

pub fn load_config() -> Config {
    let contents =
        fs::read_to_string("config/mtxia.yaml").expect("File not found config/mtxia.yaml");
    return serde_yaml::from_str(&contents).expect("Error parsing yaml");
}
