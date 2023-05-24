use bevy::prelude::*;

use crate::tiles::{Position, Tile};
use super::{
    GraphicsAssets, TILE_SIZE, TILE_Z, HEIGHT_RATIO,
    math::hex_to_v3
};

pub fn update_tile(
    tile_query: Query<(&Tile, &Children), Changed<Tile>>,
    mut sprite_query: Query<&mut TextureAtlasSprite>
) {
    for (tile, children) in tile_query.iter() {
        for child in children {
            let Ok(mut sprite) = sprite_query.get_mut(*child) else { continue };
            sprite.index = tile.0.sprite();
        }
    }
}

pub fn spawn_tile_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Tile), Added<Tile>>,
    assets: Res<GraphicsAssets>
) {
    for (entity, position, tile) in query.iter() {
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


        let mut struct_sprite = TextureAtlasSprite::new(tile.0.sprite());
        struct_sprite.custom_size = Some(Vec2::splat(TILE_SIZE));

        let structure = commands.spawn(SpriteSheetBundle {
                sprite: struct_sprite,
                texture_atlas: assets.tiles_texture.clone(),
                transform: Transform::from_translation(
                    Vec3::new(0., 0.5 * HEIGHT_RATIO * TILE_SIZE, 1.)
                ),
                ..Default::default()
            })
            .id();
        commands.entity(entity).add_child(structure);
    }
}