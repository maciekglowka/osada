use bevy::prelude::*;
use std::collections::HashMap;

use crate::hex::DIRECTIONS;
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
    // TODO move input handling to a separate module
    if !keys.just_pressed(KeyCode::Space) { return };

    let Ok(cursor) = cursor_query.get_single() else { return };
    let position = cursor.0;
    let Some(entity) = board.tiles.get(&position) else { return };

    // gather goods from all surrounding tiles
    let goods = DIRECTIONS.iter()
        .flat_map(|d| board.tiles.get(&(position + *d)))
        .flat_map(|e| tile_query.get(*e))
        .map(|t| t.0.produce())
        .fold(HashMap::new(), |mut acc, g| {
            for (goods, quantity) in g.iter() {
                let cur = acc.entry(*goods).or_insert(0);
                *cur += quantity;
            }
            acc
        });

    let Ok(mut tile) = tile_query.get_mut(*entity) else { return };

    let next = tile.0.next(goods);
    if let Some(next) = next {
        tile.0 = next;
    }
}
