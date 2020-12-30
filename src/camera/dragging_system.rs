use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion},
        ElementState,
    },
    prelude::*,
    render::camera::{Camera, OrthographicProjection},
};

use crate::{
    fish::Fish,
    ground::{GroundTile, WaterTile},
    shared::components::{CameraLockedObject, MouseState},
};

pub fn camera_dragging_system(
    mut state: Local<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    // mut query_camera: Query<(&mut Camera, &mut Transform, &mut OrthographicProjection)>,
    // mut q_locked_objs: Query<Without<CameraLockedObject, (Entity, &mut Transform)>>,
    mut q_unlocked_objs: Query<Without<CameraLockedObject, (Entity, &mut Transform)>>,
    // mut q_locked_objs: Query<With<CameraLockedObject, (Entity, &mut Transform)>>
    // mut q_health: Query<Without<HealthVisual, (Entity, &Health, &UnitSize)>>,
) {
    for ev_input in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        if ev_input.state == ElementState::Pressed && (ev_input.button == MouseButton::Left) {}

        if ev_input.state == ElementState::Pressed
            && (ev_input.button == MouseButton::Right || ev_input.button == MouseButton::Middle)
        {
            state.moving = true;
        } else if ev_input.state == ElementState::Released
            && (ev_input.button == MouseButton::Right || ev_input.button == MouseButton::Middle)
        {
            state.moving = false;
        }
    }

    if state.moving {
        for ev_mov in state.mouse_motion_event_reader.iter(&mouse_motion_events) {
            for (e, mut obj_transform) in q_unlocked_objs.iter_mut() {
                let obj_trans = &mut obj_transform.translation;
                *obj_trans.y_mut() -= ev_mov.delta.y();
                *obj_trans.x_mut() += ev_mov.delta.x();

            }
        }
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        state.cursor_position = Some(event.position);
    }
}
