use bevy::{
    ecs::{Query, Res, ResMut},
    input::Input,
    prelude::{KeyCode, Transform},
};

use crate::{
    fish::{self, moviment_system, Fish, TurnFish},
    shared::config::Config,
};

pub fn keyboard_input_system(
    keyboard_input: Res<Input<KeyCode>>,
    config: Res<Config>,
    mut fish_query: Query<(&mut Fish, &mut Transform)>,
) {
    if config.fish.keyboard {
        for (mut fish, mut t) in fish_query.iter_mut() {
            if fish.selected {
                if keyboard_input.pressed(KeyCode::Up) {
                    fish::moviment_system::increase_speed(&mut fish);
                }

                if keyboard_input.just_pressed(KeyCode::Down) {
                    fish::moviment_system::decrease_speed(&mut fish);
                }

                if keyboard_input.just_released(KeyCode::Left) {
                    fish::moviment_system::turn_fish(&TurnFish::LEFT, &mut t, &mut fish);
                }

                if keyboard_input.just_released(KeyCode::Right) {
                    fish::moviment_system::turn_fish(&TurnFish::RIGHT, &mut t, &mut fish);
                }
            }
        }
    }
}
