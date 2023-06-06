use bevy::prelude::*;

use super::{Board, TileKind, Tile, Position};

pub fn spawn_map(
    mut commands: Commands,
    mut board: ResMut<Board>
) {
    for x in 0..8 {
        for y in 0..8 {
            let state = match (x + y) % 5 {
                0 => TileKind::Plains,
                1 => TileKind::Mountains,
                _ => TileKind::Forest
            };
            let entity = commands.spawn((
                    Position(IVec2::new(x, y)),
                    Tile(state)
                ))
                .id();
            board.tiles.insert(IVec2::new(x, y), entity);
        }
    }
}