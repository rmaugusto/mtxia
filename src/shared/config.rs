use std::fs::{self};

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
    pub keyboard: bool
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct GeneralConfig {
    pub time_speed: f32,
    pub state_path: String,
    pub headless: bool
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum ModeEnum {
    LEARN,
    RUN,
    DISABLED
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub struct AiConfig {
    pub mode: ModeEnum
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
