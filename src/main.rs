extern crate tch;
mod ai;
mod camera;
mod fish;
mod ground;
mod interaction;
mod sensor;
mod shared;
mod startup;

pub const CAMERA_SCALE: f32 = 1.0;
pub const ARENA_WIDTH: f32 = 800.0;
pub const ARENA_HEIGHT: f32 = 600.0;

const MARGINS: f32 = 1.125;
pub const WINDOW_WIDTH: u32 = (MARGINS * CAMERA_SCALE * ARENA_WIDTH) as u32;
pub const WINDOW_HEIGHT: u32 = (MARGINS * CAMERA_SCALE * ARENA_HEIGHT) as u32;

use std::time::Duration;

use bevy::{app::{ScheduleRunnerSettings, startup_stage}, prelude::*};
use shared::components::CameraLockedObject;

fn main() {
    let config = shared::config::load_config();

    let mut app = App::build();

    if config.general.headless {


        app
            .add_resource(ScheduleRunnerSettings::run_loop(Duration::from_secs_f64(
                1.0 / 60.0,
            )))
            .add_plugins(MinimalPlugins)
            ;

    } else {
        app
            .add_plugins(DefaultPlugins)
            .add_resource(WindowDescriptor {
                title: "MtxIa".to_string(),
                width: WINDOW_WIDTH,
                height: WINDOW_HEIGHT,
                vsync: true,
                resizable: true,
                ..Default::default()
            });
    }

    app
        //Create resources
        .add_resource(ClearColor(Color::rgb(0.0, 0.0, 0.0)))
        .add_resource(shared::gamedata::GameData {
            create_generation: true,
            current_generation: Default::default(),
            best_time: Default::default(),
            map_data: Default::default(),
            died_fishes: Vec::new()
        })
        .add_resource(config)
        //Create Stages
        .add_startup_stage_after(startup_stage::PRE_STARTUP, "LOAD_RESOURCES")
        .add_startup_stage_after(startup_stage::STARTUP, "LOAD_GROUND")
        .add_startup_stage_after("LOAD_GROUND", "CREATE_FISH")
        .add_startup_stage_after("CREATE_FISH", "CREATE_SENSOR")
        .add_startup_stage_after("CREATE_SENSOR", "CREATE_PANEL")
        .add_stage_after(stage::UPDATE, "MOVE_FISH")
        .add_stage_after("MOVE_FISH", "UPDATE_SENSOR")
        .add_stage_after("UPDATE_SENSOR", "DETECT_SENSOR")
        .add_stage_after("DETECT_SENSOR", "UPDATE_RENDER_SENSOR")
        .add_stage_after("DETECT_SENSOR", "COLLISION_SYSTEM")
        //Create events
        // .add_event::<FishDiedEvent>()
        //Create Pugins
        .add_plugin(startup::startup_plugin::StartupPlugin)
        .add_plugin(ground::ground_plugin::GroundPlugin)
        .add_plugin(fish::fish_plugin::FishPlugin)
        .add_plugin(sensor::sensor_plugin::SensorPlugin)
        .add_plugin(camera::camera_plugin::CameraPlugin)
        .add_plugin(interaction::interaction_plugin::InteractionPlugin)
        .add_plugin(ai::ai_plugin::AiPlugin)
        .add_startup_system(setup.system())
        .run();
}

pub fn setup(mut commands: Commands) {
    let transform = Transform::from_scale(Vec3::splat(CAMERA_SCALE));

    commands
        .spawn(Camera2dComponents {
            transform: transform,
            ..Default::default()
        })
        .with(CameraLockedObject)
        .spawn(UiCameraComponents::default())
        .with(CameraLockedObject);
}
