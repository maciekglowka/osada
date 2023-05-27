use bevy::prelude::*;
use bevy::ecs::system::Command;

use crate::hex::Hex;
use super::components::{Position, Tile};
use super::models::Structure;


pub struct InsertTile {
    pub hex: Hex,
}
impl Command for InsertTile {
    fn write(self, world: &mut World) {
        let structure = if let Some(mut board) = world.get_resource_mut::<super::Board>() {
            if board.tiles.contains_key(&self.hex) { return };
            let Some(s) = board.queue.pop_front() else { return };
            s
        } else {
            return;
        };
        let entity = world.spawn((
                Position(self.hex),
                Tile(structure)
            ))
            .id();
        let Some(mut board) = world.get_resource_mut::<super::Board>() else { return };
        board.tiles.insert(self.hex, entity);
    }
}

pub struct TileUpgrade {
    pub hex: Hex,
    pub structure: Box<dyn Structure>
}
impl Command for TileUpgrade {
    fn write(self, world: &mut World) {
        let Some(board) = world.get_resource::<super::Board>() else { return };
        let Some(entity) = board.tiles.get(&self.hex) else { return };
        let Some(mut tile) = world.get_mut::<super::Tile>(*entity) else { return };
        tile.0 = self.structure;
    }
}