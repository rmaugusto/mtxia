use std::time::Instant;

use bevy::{core::Timer, ecs::Entity};

use crate::ai::AiProcessor;

mod animation_system;
mod collision_system;
mod creation_system;
mod death_system;
pub mod fish_plugin;
pub mod moviment_system;

pub enum TurnFish {
    RIGHT,
    LEFT,
}
pub struct Fish {
    pub index: i32,
    pub speed: f32,
    pub energy: f32,
    pub selected: bool,
    pub create_at: Instant,
    pub died_at: Option<Instant>,
    pub performed: f32,
    pub ai_processor: Box<dyn AiProcessor + Sync + Send + 'static>,
}

pub struct AnimationEntity {
    reverse_index: bool,
}

// pub struct FishDiedEvent {
//     entity: Entity
// }

pub struct AnimationTimer {
    timer: Timer,
}

impl Fish {
    pub fn new(
        _index: i32,
        _speed: f32,
        _energy: f32,
        _ai_processor: Box<dyn AiProcessor + Sync + Send + 'static>,
    ) -> Fish {
        Fish {
            index: _index,
            speed: _speed,
            energy: _energy,
            selected: false,
            create_at: Instant::now(),
            ai_processor: _ai_processor,
            died_at: None,
            performed: 0f32,
        }
    }
}

pub fn kill_fish() {}
