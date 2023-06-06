use bevy::prelude::*;

use crate::board::{Position, Site};
use super::{
    GraphicsAssets, TILE_SIZE, SITE_Z,
    math::to_world_v3
};


pub fn spawn_site_renderer(
    mut commands: Commands,
    query: Query<(Entity, &Position, &Site), Added<Site>>,
    assets: Res<GraphicsAssets>,
    site_data: crate::data::SiteDataParam
) {
    for (entity, position, site) in query.iter() {
        let Some(data) = site_data.get(&site.kind) else { continue };
        let mut sprite = TextureAtlasSprite::new(data.sprite);
        sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
        sprite.color = data.color.into();
        let v = to_world_v3(position.0, SITE_Z);
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