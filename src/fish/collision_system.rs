use std::time::Instant;

use bevy::prelude::*;

use crate::sensor::SensorSet;

use super::Fish;

const MIN_DISTANCE: f32 = 14.0;

pub fn detect_fish_collision(mut query: Query<(&mut Fish, &SensorSet)>) {
    for (mut fish, ss) in query.iter_mut() {
        let sens_opt = ss.sensors.iter().find(|s| s.distance <= MIN_DISTANCE);
        if sens_opt.is_some() {
            fish.die();
        }
    }
}
