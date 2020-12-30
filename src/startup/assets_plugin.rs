use bevy::{asset, prelude::*};

use super::InternalAssets;

const TILE_SIZE: f32 = 32.0;

pub(crate) fn load_resources(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<asset::Assets<TextureAtlas>>,
) {
    //Load all textures
    let ground_handle: Handle<Texture> = asset_server.load("sprites/nature.png");
    let fish_handle: Handle<Texture> = asset_server.load("sprites/fish.png");

    //Add to atlas
    let ground_atlas =
        TextureAtlas::from_grid(ground_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 20, 9);
    let fish_atlas = TextureAtlas::from_grid(fish_handle, Vec2::new(TILE_SIZE, TILE_SIZE), 12, 1);

    //Insert to a custom Resource
    commands.insert_resource(InternalAssets {
        ground: texture_atlases.add(ground_atlas),
        fish: texture_atlases.add(fish_atlas),
        text_font: asset_server.load("fonts/FiraMono-Medium.ttf")
    });


}
