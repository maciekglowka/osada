use bevy::prelude::*;
use std::collections::HashMap;

use crate::ui::Cursor;

use super::{
    Board,
    components::{Tile, Position},
    enums::Goods
};

pub fn upgrade_tile(
    keys: Res<Input<KeyCode>>,
    cursor_query: Query<&Position, With<Cursor>>,
    mut tile_query: Query<&mut Tile>,
    board: Res<Board>
) {
    if !keys.just_pressed(KeyCode::Space) { return };

    let Ok(cursor) = cursor_query.get_single() else { return };
    let Some(entity) = board.tiles.get(&cursor.0) else { return };
    let Ok(mut tile) = tile_query.get_mut(*entity) else { return };

    let next = tile.0.next(
        HashMap::from_iter([(Goods::Wood, 4)])
    );
    if let Some(next) = next {
        tile.0 = next;
    }
}
