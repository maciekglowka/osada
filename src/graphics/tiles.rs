use bevy::prelude::*;

use crate::tiles::{Position, Tile};
use super::{
    GraphicsAssets, TILE_SIZE, TILE_Z,
    math::hex_to_v3
};

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    assets: Res<GraphicsAssets>
) {
    for (entity, position) in query.iter() {
        let mut sprite = TextureAtlasSprite::new(0);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = Color::WHITE;
        let v = hex_to_v3(position.0, TILE_Z);
        commands.entity(entity)
            .insert(
                SpriteSheetBundle {
                    sprite,
                    texture_atlas: assets.hex_texture.clone(),
                    transform: Transform::from_translation(v),
                    ..Default::default()
                }
            );
        info!("Spawned at: {:?}", v);
    }
}