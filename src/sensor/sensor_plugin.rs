use bevy::{
    ecs::IntoQuerySystem,
    prelude::{AppBuilder, Plugin},
};

use super::*;

pub struct SensorPlugin;

impl Plugin for SensorPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_to_stage(
            "UPDATE_SENSOR",
            update_coordinates_system::update_sensor_coordinates.system(),
        );
        app.add_system_to_stage(
            "DETECT_SENSOR",
            collision_system::detect_sensor_collision.system(),
        );
        app.add_system_to_stage(
            "UPDATE_RENDER_SENSOR",
            render_system::update_sensor_render_system.system(),
        );
    }
}
