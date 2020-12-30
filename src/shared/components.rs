use bevy::{input::mouse::*, math::Vec2, prelude::EventReader, window::CursorMoved};

use crate::ai::{AiProcessor};


#[derive(Default)]
pub struct MouseState {
    pub mouse_button_event_reader: EventReader<MouseButtonInput>,
    pub mouse_motion_event_reader: EventReader<MouseMotion>,
    pub mouse_wheel_event_reader: EventReader<MouseWheel>,
    pub cursor_moved_event_reader: EventReader<CursorMoved>,
    pub cursor_position: Option<Vec2>,
    pub moving: bool,
}


pub struct CameraLockedObject;

