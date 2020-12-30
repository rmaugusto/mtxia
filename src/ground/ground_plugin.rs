use bevy::prelude::*;

use super::*;

pub struct GroundPlugin;

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("LOAD_GROUND", creation_system::create_ground.system());
    }
}
