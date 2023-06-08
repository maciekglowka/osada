use bevy::prelude::*;

use crate::board::{
    // commands::PlaceSite,
    Position,
    // SiteKind
};
use crate::graphics::{math::to_world_v3, TILE_SIZE, OVERLAY_Z};

use super::{UiAssets, events::CursorUpdateEvent};

#[derive(Component)]
pub struct Cursor;

pub fn cursor_input(
    // mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    mut query: Query<(&mut Position, &mut Transform), With<Cursor>>,
    mut ev_cursor: EventWriter<CursorUpdateEvent>
) {
    for (key, dir) in DIR_KEY_MAPPING {
        if !keys.pressed(key) { continue; }
        let Ok((mut position, mut transform)) = query.get_single_mut() else { return };
        position.0 += dir;
        transform.translation = to_world_v3(position.0, OVERLAY_Z);
        ev_cursor.send(CursorUpdateEvent);
    }
}

pub fn spawn_cursor(
    mut commands: Commands,
    assets: Res<UiAssets>
) {
    let mut sprite = TextureAtlasSprite::new(0);
    sprite.custom_size = Some(Vec2::splat(TILE_SIZE));
    let v = IVec2::ZERO;
    let v3 = to_world_v3(v, OVERLAY_Z); 
    commands.spawn((
            Cursor,
            Position(v),
            SpriteSheetBundle {
                sprite,
                texture_atlas: assets.cursor_texture.clone(),
                transform: Transform::from_translation(v3),
                ..Default::default()
            }
        ));
}

const DIR_KEY_MAPPING: [(KeyCode, IVec2); 4] = [
    (KeyCode::W, IVec2::new(0, 1)), (KeyCode::S, IVec2::new(0, -1)),
    (KeyCode::A, IVec2::new(-1, 0)), (KeyCode::D, IVec2::new(1, 0))
];