use bevy::prelude::*;
use std::collections::HashMap;

use crate::hex::Hex;
use crate::states::MainState;

mod components;
mod enums;
mod models;
mod systems;

pub use components::{Position, Tile};

pub struct TilesPlugin;
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_system(spawn_tiles.in_schedule(OnEnter(MainState::Game)))
            .add_system(systems::upgrade_tile.in_set(OnUpdate(MainState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Hex, Entity>
}

fn spawn_tiles(mut commands: Commands, mut board: ResMut<Board>) {
    insert_tile(&mut commands, board.as_mut(), 0, 0, None);
    insert_tile(&mut commands, board.as_mut(), 1, 0, Some(Box::new(models::Forest)));
    insert_tile(&mut commands, board.as_mut(), 1, 1, None);
    insert_tile(&mut commands, board.as_mut(), 2, 0, Some(Box::new(models::Forest)));
    insert_tile(&mut commands, board.as_mut(), 2, -1, None);
    insert_tile(&mut commands, board.as_mut(), 2, 1, None);
    insert_tile(&mut commands, board.as_mut(), 1, -1, None);
    insert_tile(&mut commands, board.as_mut(), 1, -2, None);
    insert_tile(&mut commands, board.as_mut(), -1, 0, None);
    insert_tile(&mut commands, board.as_mut(), -1, -1, None);
}

fn insert_tile(
    commands: &mut Commands,
    board: &mut Board,
    q: i32,
    r: i32,
    model: Option<Box<dyn models::Structure>>
) {
    let structure = match model {
        Some(m) => m,
        None => Box::new(models::Empty) as Box<dyn models::Structure>
    };
    let hex = Hex::new(q, r);
    let entity = commands.spawn((
            Position(hex),
            Tile(structure)
        ))
        .id();
    board.tiles.insert(hex, entity);
}