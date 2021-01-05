use bevy::{
    core::Time,
    ecs::{Query, Res},
    math::{Quat, Vec3},
    prelude::Transform,
};

use crate::shared::config::Config;

use super::{Fish, TurnFish};

const TURN_ANGLE: f32 = 0.261799; //15 degrees

pub(crate) fn move_fish_forward(
    time: Res<Time>,
    config: Res<Config>,
    mut query: Query<(&mut Transform, &mut Fish)>,
) {
    for (mut t, mut fish) in query.iter_mut() {
        //Make sure is not dead
        if fish.alive {
            let (axis, ang) = t.rotation.to_axis_angle();

            let ang = match axis.z() > 0.0 {
                true => -ang,
                false => ang,
            };

            let delta_seconds = f32::min(0.1, time.delta_seconds) * config.general.time_speed;
            let distance = delta_seconds * fish.speed;
            let speed = fish.speed;
            t.translation += Vec3::new(distance * ang.sin(), distance * ang.cos(), 0.0);

            fish.distance += distance;

            //Swimming uses energy
            fish.consume_energy(delta_seconds * speed * config.fish.energy_to_walk);
        }
    }
}

pub fn decrease_speed(fish: &mut Fish) {
    fish.speed -= 10f32;
}

pub fn increase_speed(fish: &mut Fish) {
    fish.speed += 10f32;
}

pub fn turn_fish(
    config: &Config,
    direction: &TurnFish,
    fish_transform: &mut Transform,
    fish: &mut Fish,
) {
    //Make sure is not dead
    if fish.alive {
        match direction {
            TurnFish::RIGHT => fish_transform.rotate(Quat::from_rotation_z(-TURN_ANGLE)),
            TurnFish::LEFT => fish_transform.rotate(Quat::from_rotation_z(TURN_ANGLE)),
        }

        //Turn spends much energy
        fish.consume_energy(config.fish.energy_to_turn);
    }
}
