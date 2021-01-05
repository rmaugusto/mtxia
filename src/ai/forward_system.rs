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
            let mut input_data: Vec<f64> = Vec::new();

            if config
                .ai
                .input_vars
                .contains(&crate::shared::config::InputLayerEnum::SENSORS)
            {
                let distances: Vec<f64> = ss.sensors.iter().map(|s| s.distance as f64).collect();
                input_data.extend(distances);
            }

            if config
                .ai
                .input_vars
                .contains(&crate::shared::config::InputLayerEnum::SPEED)
            {
                input_data.push(fish.speed as f64);
            }

            if config
                .ai
                .input_vars
                .contains(&crate::shared::config::InputLayerEnum::ENERGY)
            {
                input_data.push(fish.energy as f64);
            }

            if config
                .ai
                .input_vars
                .contains(&crate::shared::config::InputLayerEnum::FITNESS)
            {
                input_data.push(fish.fitness() as f64);
            }

            if config
                .ai
                .input_vars
                .contains(&crate::shared::config::InputLayerEnum::AGE)
            {
                input_data.push(fish.age() as f64);
            }

            if config
                .ai
                .input_vars
                .contains(&crate::shared::config::InputLayerEnum::DISTANCE)
            {
                input_data.push(fish.distance as f64);
            }

            let output = fish.brain.nn_forward(input_data);

            if output[0] {
                fish::moviment_system::turn_fish(&config, &fish::TurnFish::LEFT, &mut t, &mut fish);
            }

            if output[1] {
                fish::moviment_system::turn_fish(
                    &config,
                    &fish::TurnFish::RIGHT,
                    &mut t,
                    &mut fish,
                );
            }
        }
    }
}
