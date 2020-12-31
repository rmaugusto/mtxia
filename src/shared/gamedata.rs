use std::time::Duration;

use crate::fish::Fish;

#[derive(Debug, Default)]
pub struct MapData {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
}

pub struct GameData {
    pub current_generation: i32,
    pub best_time: Option<Duration>,
    pub map_data: MapData,
    pub create_generation: bool,
    pub died_fishes: Vec<Fish>
}
