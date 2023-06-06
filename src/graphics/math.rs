use bevy::prelude::*;

use super::TILE_SIZE;

pub fn to_world_v2(v: IVec2) -> Vec2 {
    TILE_SIZE * Vec2::new(v.x as f32, v.y as f32)
}
pub fn to_world_v3(v: IVec2, z: f32) -> Vec3 {
    to_world_v2(v).extend(z)
}