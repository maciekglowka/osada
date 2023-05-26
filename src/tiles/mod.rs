use bevy::prelude::*;
use std::collections::HashMap;

use crate::hex::Hex;
use crate::states::MainState;

pub mod commands;
mod components;
mod enums;
mod models;
mod systems;

pub use components::{Position, Tile};
pub use models::Structure;

pub struct TilesPlugin;
impl Plugin for TilesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            // .add_event::<TileEvent>()
            .add_system(spawn_tiles.in_schedule(OnEnter(MainState::Game)));
            // .add_system(systems::upgrade_tile.run_if(OnUpdate(MainState::Game)));
    }
}

// pub struct TileEvent(pub TileEventKind);
// pub enum TileEventKind {
//     Upgrade(Hex, Box<dyn models::Structure>)
// }

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

pub fn upgrade_tile(
    position: Position,
    structure: Box<dyn models::Structure>,
    tile_query: &mut Query<&mut Tile>,
    board: &Board
) {
    // // TODO move input handling to a separate module
    // if !keys.just_pressed(KeyCode::Space) { return };

    // let Ok(cursor) = cursor_query.get_single() else { return };
    // let position = cursor.0;
    // let Some(entity) = board.tiles.get(&position) else { return };

    // // gather goods from all surrounding tiles
    // let goods = DIRECTIONS.iter()
    //     .flat_map(|d| board.tiles.get(&(position + *d)))
    //     .flat_map(|e| tile_query.get(*e))
    //     .map(|t| t.0.produce())
    //     .fold(HashMap::new(), |mut acc, g| {
    //         for (goods, quantity) in g.iter() {
    //             let cur = acc.entry(*goods).or_insert(0);
    //             *cur += quantity;
    //         }
    //         acc
    //     });
    let Some(entity) = board.tiles.get(&position.0) else { return };
    let Ok(mut tile) = tile_query.get_mut(*entity) else { return };
    tile.0 = structure;

    // let next = tile.0.next(goods);
    // if let Some(next) = next {
    //     tile.0 = next;
    // }
}
