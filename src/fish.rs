use bevy::prelude::*;
use bevy_rapier2d::physics::RigidBodyHandleComponent;
use rand::Rng;

use crate::{assets, gamedata::{GameData}, ground::WaterTile};
extern crate tiled;

const TILE_SIZE: f32 = 96.0;

pub struct FishPlugin;

pub struct AnimationEntity {
    reverse_index: bool,
}

#[derive(Default)]
pub struct FishCreation {
    count: u16,
}

impl Plugin for FishPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_system_to_stage("LOAD_FISH",create_fishes.system());
        app.add_system(animate_fish.system());
        // app.add_system(create_fishes.system());
    }
}

fn create_fishes(
    mut commands: Commands,
    assets: Res<assets::Assets>,
    mut fc: Local<FishCreation>,
    game_data: Res<GameData>,
    query: Query<(&Transform,&WaterTile)>,
) {
    // if fc.count == 0 {

        let mut rng = rand::thread_rng();

        //Scan water position
        let mut water_translator_ref = Vec::new();
        for (t, wt) in query.iter() {
            water_translator_ref.push(&t.translation);
        }

        //Ramdom fish position
        let water_idx =rng.gen_range(0, water_translator_ref.len()-1);

        let mut transform = Transform::default();
        transform.scale = Vec3::splat(1.5);
        transform.rotate(Quat::from_rotation_z(rng.gen_range(0.0, 179.0)));
        transform.translation = water_translator_ref[water_idx].clone();

        commands
            .spawn(SpriteSheetComponents {
                texture_atlas: assets.fish.clone(),
                transform: transform,
                ..Default::default()
            })
            .with(AnimationEntity {
                reverse_index: false,
            })
            .with(Timer::from_seconds(0.3, true));

        fc.count += 1;

    // }

}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,

) {

    // for w in query.iter_mut() {
    //     print!("xxx");
    // }
    //Load texture
    let texture_handle = asset_server.load("sprites/fish.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 12, 1);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);

    let mut transform = Transform::default();
    transform.scale = Vec3::splat(0.5);
    transform.rotate(Quat::from_rotation_z(90.0));

    commands
        .spawn(SpriteSheetComponents {
            texture_atlas: texture_atlas_handle.clone(),
            transform: transform,
            ..Default::default()
        })
        .with(AnimationEntity {
            reverse_index: false,
        })
        .with(Timer::from_seconds(0.3, true));
}

fn animate_fish(mut query: Query<(&mut Timer, &mut TextureAtlasSprite, &mut AnimationEntity)>) {
    for (timer, mut sprite, mut ae) in query.iter_mut() {
        if timer.finished {
            if ae.reverse_index {
                sprite.index -= 1;
            } else {
                sprite.index += 1;
            }

            if sprite.index == 0 {
                ae.reverse_index = false;
            } else if sprite.index >= 2 {
                ae.reverse_index = true;
            }
        }
    }
}
