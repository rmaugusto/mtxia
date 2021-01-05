use std::time::Instant;

use bevy::{core::Timer, ecs::Entity};

use crate::{ai::AiProcessor, shared::config::Config};

mod animation_system;
mod collision_system;
mod creation_system;
mod death_system;
pub mod fish_plugin;
pub mod moviment_system;
pub mod save_summary;

pub enum TurnFish {
    RIGHT,
    LEFT,
}
pub struct Fish {
    pub index: i32,
    pub speed: f32,
    pub energy: f32,
    pub selected: bool,
    pub alive: bool,
    pub create_at: Instant,
    pub distance: f32,
    pub died_at: Option<Instant>,
    pub brain: Box<dyn AiProcessor + Sync + Send>,
    max_energy: f32,
}

impl Fish {
    pub fn age(&self) -> f32 {
        if self.alive {
            self.create_at.elapsed().as_secs_f32()
        } else {
            self.create_at.elapsed().as_secs_f32() - self.died_at.unwrap().elapsed().as_secs_f32()
        }
    }

    pub fn fitness(&self) -> f32 {
        (self.distance * (self.max_energy - self.energy)) / 1000f32
    }

    pub fn die(&mut self) {
        self.alive = false;
        self.died_at = Some(Instant::now());
    }

    pub fn consume_energy(&mut self, used: f32) {
        self.energy -= used;

        if self.energy < 0f32 {
            self.energy = 0f32;
        }
    }
}

pub struct FishAlive {}

pub struct AnimationEntity {
    reverse_index: bool,
}

pub struct AnimationTimer {
    timer: Timer,
}

impl Fish {
    pub fn new(
        _index: i32,
        _speed: f32,
        _max_energy: f32,
        _brain: Box<dyn AiProcessor + Sync + Send>,
    ) -> Fish {
        Fish {
            index: _index,
            speed: _speed,
            energy: _max_energy,
            max_energy: _max_energy,
            selected: false,
            create_at: Instant::now(),
            brain: _brain,
            died_at: None,
            distance: 0f32,
            alive: true,
        }
    }
}
