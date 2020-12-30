use bevy::{
    ecs::IntoQuerySystem,
    prelude::{AppBuilder, Plugin},
};

use super::*;

pub struct StartupPlugin;

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("LOAD_RESOURCES", assets_plugin::load_resources.system());
    }
}
