use bevy::prelude::*;
use std::collections::HashMap;

use crate::hex::DIRECTIONS;
use crate::ui::Cursor;

use super::{
    Board,
    components::{Tile, Position},
    enums::Goods
};

// pub fn upgrade_tile(
//     mut tile_query: Query<&mut Tile>,
//     board: Res<Board>,
//     ev_tile: EventReader<super::TileEvent>
// ) {
//     for ev in ev_tile.iter() {
//         if let TileEvent(TileEventKind::Upgrade(hex, structure)) = ev {
//             let Some(entity) = board.tiles.get(&hex) else { return };
//             let Ok(mut tile) = tile_query.get_mut(*entity) else { return };
//             tile.0 = structure;
//         }
//     }
// }
