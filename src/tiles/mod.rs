use bevy::prelude::*;

use crate::hex::Hex;
use crate::states::MainState;

mod components;

pub use components::{Position, Tile};

pub struct TilesPlugin;
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_tiles.in_schedule(OnEnter(MainState::Game)));
    }
}

fn spawn_tiles(mut commands: Commands) {
    info!("Spawning tiles");
    insert_tile(&mut commands, 0, 0);
    insert_tile(&mut commands, 1, 0);
    insert_tile(&mut commands, 1, 1);
    insert_tile(&mut commands, 2, 0);
    insert_tile(&mut commands, 2, -1);
    insert_tile(&mut commands, 2, 1);
    insert_tile(&mut commands, 1, -1);
    insert_tile(&mut commands, 1, -2);
    insert_tile(&mut commands, -1, 0);
    insert_tile(&mut commands, -1, -1);
}

fn insert_tile(
    commands: &mut Commands,
    q: i32,
    r: i32
) {
    commands.spawn((
        Position(Hex::new(q, r)),
        Tile
    ));
}