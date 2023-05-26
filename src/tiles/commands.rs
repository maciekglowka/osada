use bevy::prelude::*;
use bevy::ecs::system::Command;

use crate::hex::Hex;
use super::models::Structure;

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