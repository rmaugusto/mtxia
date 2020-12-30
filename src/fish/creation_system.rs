use bevy::prelude::*;
use rand::Rng;

use crate::{
    ai::ai_processor_factory::create_ai_processor,
    ground::WaterTile,
    sensor::SensorSet,
    shared::{config::Config, gamedata::GameData},
    startup::InternalAssets,
};

use super::{AnimationEntity, AnimationTimer, Fish};
extern crate tiled;

pub(crate) fn create_fishes(
    mut commands: Commands,
    assets: Res<InternalAssets>,
    mut config: ResMut<Config>,
    mut gd: ResMut<GameData>,
    q_water: Query<(&Transform, &WaterTile)>,
    q_fish: Query<(&Fish)>,
) {

    if !gd.create_generation {
        return;
    }

    let mut rng = rand::thread_rng();

    //Scan water position
    let water_translator_ref: Vec<Vec3> = q_water.iter().map(|(t, _)| t.translation).collect();

    let mut to_create = 0;

    //Create initial config fishes
    if q_fish.iter().len() == 0 {
        to_create = config.fish.count as usize;
        gd.current_generation += 1;
        config.fish.energy += config.fish.energy_inc;
        println!("Creating new generation: {}", gd.current_generation);
        gd.create_generation = false;

    }

    // Random angle
    let angle: f32 = rng.gen_range(0.0, 180f32.to_radians());

    // Random initial position
    let cur_gen: i32 = gd.current_generation / 10;
    let water_idx = (cur_gen % (water_translator_ref.len() as i32 - 1)) as usize;
    // Random initial position
    let water_idx = rng.gen_range(0, water_translator_ref.len());

    (0..to_create).for_each(|i| {


        let mut transform = Transform::default();
        transform.scale = Vec3::splat(1.2);
        transform.rotate(Quat::from_rotation_z(angle));
        transform.translation = water_translator_ref[water_idx].clone();

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: assets.fish.clone(),
                transform: transform,
                ..Default::default()
            })
            .with(Fish::new(
                i as i32,
                config.fish.speed,
                config.fish.energy,
                create_ai_processor(),
            ))
            .with(AnimationEntity {
                reverse_index: false,
            })
            .with(AnimationTimer {
                timer: Timer::from_seconds(0.1, true),
            })
            .with(SensorSet::new(config.fish.sensors, config.fish.range));
    });
}
