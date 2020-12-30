use bevy::{
    input::mouse::MouseWheel,
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

use crate::shared::components::MouseState;


pub fn camera_zoom_system(
    mut state: Local<MouseState>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    mut query_camera: Query<(&mut Camera, &mut Transform, &mut OrthographicProjection)>,
) {
    for ev_wh in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        for (_, mut tc, _) in query_camera.iter_mut() {
            let scale = &mut tc.scale;
            *scale.x_mut() = (0.1 as f32).max(*scale.x_mut() - (ev_wh.y * 0.1));
            *scale.y_mut() = *scale.x_mut()
        }
    }
}
