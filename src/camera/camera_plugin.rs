use bevy::{diagnostic::FrameTimeDiagnosticsPlugin, ecs::IntoQuerySystem, prelude::{AppBuilder, Plugin, stage}};

use super::*;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(FrameTimeDiagnosticsPlugin::default());
        app.add_startup_system(fps_system::create_fps_text.system());
        app.add_system(fps_system::text_update_system.system());
        app.add_system(dragging_system::camera_dragging_system.system());
        app.add_system(zoom_system::camera_zoom_system.system());
    }
}
