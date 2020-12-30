use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ElementState,
    },
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

use crate::{fish::Fish, shared::components::MouseState};

pub fn entity_selection_system(
    mut state: Local<MouseState>,
    wnds: Res<Windows>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut q_fish: Query<(&mut Transform, &mut Fish)>,
    q_camera: Query<(&Camera, &Transform, &OrthographicProjection)>,
) {
    for ev_input in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        //TODO: RECREATE SOMETHING MORE REUSABLE
        if ev_input.state == ElementState::Pressed && (ev_input.button == MouseButton::Left) {
            let wnd = wnds.get_primary().unwrap();
            let mouse_cp = state.cursor_position.unwrap();
            let win_size = Vec2::new(wnd.width() as f32, wnd.height() as f32);
            let pos = mouse_cp - win_size / 2.0;

            for (_, cam_transform, _) in q_camera.iter() {
                let click_pos = cam_transform.compute_matrix() * pos.extend(0.0).extend(1.0);

                for (fish_transform, mut fish) in q_fish.iter_mut() {
                    fish.selected = false;

                    if click_pos.x() + 15.0 >= fish_transform.translation.x()
                        && click_pos.x() - 15.0 <= fish_transform.translation.x()
                        && click_pos.y() + 15.0 >= fish_transform.translation.y()
                        && click_pos.y() - 15.0 <= fish_transform.translation.y()
                    {
                        fish.selected = true;
                    }
                }
            }
        }
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        state.cursor_position = Some(event.position);
    }
}
