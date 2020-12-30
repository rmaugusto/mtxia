use bevy::prelude::*;

use super::*;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut AppBuilder) {

        app.add_startup_system_to_stage("CREATE_PANEL", creation_system::create_panel.system());
        app.add_system(entity_selection_system::entity_selection_system.system());
        app.add_system(update_panel::text_update_system.system());
        app.add_system(keyboard_system::keyboard_input_system.system());
        
    }
}
