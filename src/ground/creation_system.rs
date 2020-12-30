use bevy::prelude::*;

use std::fs::File;
use std::io::BufReader;
use std::path::Path;

use tiled::{parse_with_path, LayerData};

use crate::{shared::gamedata::GameData, startup::InternalAssets};

use super::{FloorTile, GroundTile, OtherTypeIntersection, WaterTile, HALF_TILE_SIZE, TILE_SIZE};

const WATER_ID: u32 = 41;
const FLOOR_ID: u32 = 40;

pub(crate) fn create_ground(
    mut commands: Commands,
    windows: Res<Windows>,
    mut gd: ResMut<GameData>,
    assets: Res<InternalAssets>,
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
    for y in 0..map_tiles.len() {
        let lt1 = &map_tiles[y];
        for x in 0..lt1.len() {
            let cur_tile = map_tiles[y][x];
            let cur_tile_gid = cur_tile.gid - 1;

            let tile_position = Vec3::new(
                x as f32 * TILE_SIZE,
                ((win.height() / 2) as f32 - HALF_TILE_SIZE) - (y as f32 * TILE_SIZE),
                0.0,
            );

            if y == 0 && x == 0 {
                gd.map_data.x1 = tile_position.x();
                gd.map_data.y1 = tile_position.y();
            }

            if y == map_tiles.len()-1 && x == lt1.len()-1 {
                gd.map_data.x2 = tile_position.x()+TILE_SIZE;
                gd.map_data.y2 = tile_position.y()+TILE_SIZE;
            }

            let sprite = commands
                .spawn(SpriteSheetComponents {
                    texture_atlas: assets.ground.clone(),
                    sprite: TextureAtlasSprite::new(cur_tile_gid),
                    transform: Transform::from_translation(tile_position),
                    ..Default::default()
                })
                .with(GroundTile);

            if cur_tile_gid == WATER_ID {
                sprite.with(WaterTile);
            } else if cur_tile_gid == FLOOR_ID {
                sprite.with(FloorTile);
            } else {
                panic!("GID not implemented")
            }

            let up_tile = get_matrix_element(map_tiles, y as i32 - 1, x as i32);
            let down_tile = get_matrix_element(map_tiles, y as i32 + 1, x as i32);
            let left_tile = get_matrix_element(map_tiles, y as i32, x as i32 - 1);
            let right_tile = get_matrix_element(map_tiles, y as i32, x as i32 + 1);

            if up_tile.is_some() && up_tile.unwrap().gid != cur_tile.gid
                || down_tile.is_some() && down_tile.unwrap().gid != cur_tile.gid
                || left_tile.is_some() && left_tile.unwrap().gid != cur_tile.gid
                || right_tile.is_some() && right_tile.unwrap().gid != cur_tile.gid
            {
                sprite.with(OtherTypeIntersection);
            }
        }
    }

}

fn get_matrix_element<T>(matrix: &Vec<Vec<T>>, x: i32, y: i32) -> Option<&T> {
    if x < 0 || matrix.get(x as usize).is_none() {
        return None;
    }

    if y < 0 || matrix[x as usize].get(y as usize).is_none() {
        return None;
    }

    Some(&matrix[x as usize][y as usize])
}
