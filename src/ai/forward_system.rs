use bevy::{
    ecs::{Query, Res},
    prelude::Transform,
};

use crate::{
    fish::{self, Fish},
    sensor::SensorSet,
    shared::config::{Config, ModeEnum},
};

use super::FishAi;

pub(crate) fn forward_system(
    config: Res<Config>,
    mut q_fishes: Query<(&mut Fish, &SensorSet, &FishAi, &mut Transform)>,
) {
    if config.ai.mode != ModeEnum::DISABLED {
        for (mut fish, ss, _, mut t) in q_fishes.iter_mut() {
            let mut other_input_data = vec![fish.energy as f64, config.fish.speed as f64];

            let distances: Vec<f64> = ss.sensors.iter().map(|s| s.distance as f64).collect();

            other_input_data.extend(distances);

            let output = fish.ai_processor.forward(other_input_data);

            if output[0] {
                fish::moviment_system::turn_fish(&fish::TurnFish::LEFT, &mut t, &mut fish);
            }

            if output[1] {
                fish::moviment_system::turn_fish(&fish::TurnFish::RIGHT, &mut t, &mut fish);
            }
        }
    }
}
