use bevy::prelude::*;

use super::*;

pub struct AiPlugin;

impl Plugin for AiPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(creation_system::create_ai.system());
        app.add_system_to_stage("MOVE_FISH", forward_system::forward_system.system());
    }
}


