pub mod ground_plugin;
mod creation_system;
pub struct GroundTile;

pub struct OtherTypeIntersection;
pub struct FloorTile;
pub struct WaterTile;

pub const TILE_SIZE: f32 = 32.0;
pub const HALF_TILE_SIZE: f32 = TILE_SIZE / 2.0;