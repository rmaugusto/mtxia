use bevy::{app::startup_stage, prelude::*};

mod assets;
mod fish;
mod fps;
mod gamedata;
mod ground;

pub const CAMERA_SCALE: f32 = 1.0;
pub const ARENA_WIDTH: f32 = 640.0;
pub const ARENA_HEIGHT: f32 = 640.0;

const MARGINS: f32 = 1.125;
pub const WINDOW_WIDTH: u32 = (MARGINS * CAMERA_SCALE * ARENA_WIDTH) as u32;
pub const WINDOW_HEIGHT: u32 = (MARGINS * CAMERA_SCALE * ARENA_HEIGHT) as u32;

fn main() {
    App::build()
        .add_resource(WindowDescriptor {
            title: "MtxIa".to_string(),
            width: WINDOW_WIDTH,
            height: WINDOW_HEIGHT,
            vsync: true,
            resizable: true,
            ..Default::default()
        })
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(gamedata::GameData {})
        .add_plugins(DefaultPlugins)
        .add_startup_stage_after(startup_stage::STARTUP, "LOAD_RESOURCES")
        .add_startup_stage_after("LOAD_RESOURCES", "LOAD_GROUND")
        .add_startup_stage_after("LOAD_GROUND", "LOAD_FISH")
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(ground::GroundPlugin)
        .add_plugin(fish::FishPlugin)
        .add_startup_system(setup.system())
        .run();
}

pub fn setup(mut commands: Commands) {
    commands
        .spawn(Camera2dComponents {
            transform: Transform::from_scale(Vec3::splat(CAMERA_SCALE)),
            ..Default::default()
        })
        .spawn(UiCameraComponents::default());
}
