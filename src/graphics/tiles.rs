use bevy::prelude::*;

use crate::board::{Position, Tile};
use super::{
    GraphicsAssets, TILE_SIZE, TILE_Z,
    math::to_world_v3
};

pub fn update_tile(
    mut query: Query<(&Tile, &mut TextureAtlasSprite), Changed<Tile>>,
    tile_data: crate::data::TileDataParam
) {
    for (tile, mut sprite) in query.iter_mut() {
        let Some(data) = tile_data.get(&tile.0) else { continue };
        sprite.index = data.sprite;
        sprite.color = data.color.into();
    }
}

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Tile), Added<Tile>>,
    assets: Res<GraphicsAssets>,
    tile_data: crate::data::TileDataParam
) {
    for (entity, position, tile) in query.iter() {
        let Some(data) = tile_data.get(&tile.0) else { continue };
        let mut sprite = TextureAtlasSprite::new(data.sprite);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = data.color.into();
        let v = to_world_v3(position.0, TILE_Z);
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    sprite,
                    texture_atlas: assets.tiles_texture.clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            );
    }
}