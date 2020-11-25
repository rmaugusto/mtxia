use bevy::{
    input::{
        mouse::{MouseButtonInput, MouseMotion, MouseWheel},
        ElementState,
    },
    prelude::*,
    render::camera::Camera,
    render::camera::OrthographicProjection,
};
extern crate tiled;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tiled::{parse_with_path, LayerData};

use crate::{
    assets,
};

const TILE_SIZE: f32 = 32.0;
const WATER_ID:u32 = 41;

pub struct GroundPlugin;
pub struct WaterTile;

#[derive(Default)]
struct State {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    mouse_motion_event_reader: EventReader<MouseMotion>,
    mouse_wheel_event_reader: EventReader<MouseWheel>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
    cursor_position: Option<Vec2>,
    moving: bool,
}

impl Plugin for GroundPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("LOAD_GROUND",setup_plugin.system());
        app.add_system(ground_mouse_moviment.system());
    }
}


fn setup_plugin(
    mut commands: Commands,
    windows: Res<Windows>,
    assets: Res<assets::Assets>,
) {

    let win = windows.get_primary().unwrap();

    //Load Map tiled
    let file = File::open(&Path::new("assets/map/map.tmx")).unwrap();
    let reader = BufReader::new(file);
    let map = parse_with_path(reader, Path::new("./assets/map/map.tmx")).unwrap();

    //Read only first layer
    let layer0 = map.layers.get(0).unwrap();

    let map_tiles = match &layer0.tiles {
        LayerData::Finite(vec_titles) => vec_titles,
        LayerData::Infinite(_) => panic!("Invalid title type"),
    };

    //Create elements according to map
    for (y, t) in map_tiles.iter().enumerate() {
        for (x, t2) in t.iter().enumerate() {

            let tile_position = Vec3::new(
                x as f32 * TILE_SIZE,
                ((win.height() / 2) as f32 - 16.0) - (y as f32 * TILE_SIZE),
                0.0,
            );

            if t2.gid - 1 == WATER_ID {
                commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: assets.ground.clone(),
                    sprite: TextureAtlasSprite::new(t2.gid - 1),
                    transform: Transform::from_translation(tile_position),
                    ..Default::default()
                })
                .with(WaterTile);
            }else{
                commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: assets.ground.clone(),
                    sprite: TextureAtlasSprite::new(t2.gid - 1),
                    transform: Transform::from_translation(tile_position),
                    ..Default::default()
                });

            }

        }
    }

}

fn ground_mouse_moviment(
    mut state: Local<State>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    mouse_motion_events: Res<Events<MouseMotion>>,
    mouse_wheel_events: Res<Events<MouseWheel>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut query_camera: Query<(&mut Camera, &mut Transform, &mut OrthographicProjection)>,
) {
    for ev_input in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
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
            for (_, mut tc, _) in query_camera.iter_mut() {
                let x_scale = tc.scale.x();
                let y_scale = tc.scale.y();

                let translation = &mut tc.translation;

                *translation.y_mut() += ev_mov.delta.y() * y_scale;
                *translation.x_mut() -= ev_mov.delta.x() * x_scale;
            }
        }
    }

    for ev_wh in state.mouse_wheel_event_reader.iter(&mouse_wheel_events) {
        for (_, mut tc, _) in query_camera.iter_mut() {
            let scale = &mut tc.scale;
            *scale.x_mut() = (0.1 as f32).max(*scale.x_mut() - (ev_wh.y * 0.1));
            *scale.y_mut() = *scale.x_mut()
        }
    }

    for event in state.cursor_moved_event_reader.iter(&cursor_moved_events) {
        state.cursor_position = Some(event.position);
    }
}
