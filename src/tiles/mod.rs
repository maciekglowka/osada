use bevy::prelude::*;
use std::collections::{HashMap, VecDeque};

use crate::hex::{DIRECTIONS, Hex};
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
            .add_system(spawn_tiles.in_schedule(OnEnter(MainState::Game)));
    }
}


#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<Hex, Entity>,
    pub queue: VecDeque<Box<dyn Structure>>
}

fn spawn_tiles(mut board: ResMut<Board>) {
    // insert_tile(&mut commands, board.as_mut(), 0, 0, None);
    // insert_tile(&mut commands, board.as_mut(), 1, 0, Some(Box::new(models::Forest)));
    // insert_tile(&mut commands, board.as_mut(), 1, 1, None);
    // insert_tile(&mut commands, board.as_mut(), 2, 0, Some(Box::new(models::Forest)));
    // insert_tile(&mut commands, board.as_mut(), 2, -1, None);
    // insert_tile(&mut commands, board.as_mut(), 2, 1, None);
    // insert_tile(&mut commands, board.as_mut(), 1, -1, None);
    // insert_tile(&mut commands, board.as_mut(), 1, -2, None);
    // insert_tile(&mut commands, board.as_mut(), -1, 0, None);
    // insert_tile(&mut commands, board.as_mut(), -1, -1, None);
    board.queue.push_back(Box::new(models::Empty));
    board.queue.push_back(Box::new(models::Forest));
    board.queue.push_back(Box::new(models::Forest));
    board.queue.push_back(Box::new(models::Empty));
    board.queue.push_back(Box::new(models::Forest));
}

// fn insert_tile(
//     commands: &mut Commands,
//     board: &mut Board,
//     q: i32,
//     r: i32,
//     model: Option<Box<dyn models::Structure>>
// ) {
//     let structure = match model {
//         Some(m) => m,
//         None => Box::new(models::Empty) as Box<dyn models::Structure>
//     };
//     let hex = Hex::new(q, r);
//     let entity = commands.spawn((
//             Position(hex),
//             Tile(structure)
//         ))
//         .id();
//     board.tiles.insert(hex, entity);
// }

pub fn check_goods(
    hex: Hex,
    tile_query: &Query<&Tile>,
    board: &Board
) -> HashMap<enums::Goods, u32> {
    // gather goods from all the surrounding tiles
    DIRECTIONS.iter()
        .flat_map(|d| board.tiles.get(&(hex + *d)))
        .flat_map(|e| tile_query.get(*e))
        .map(|t| t.0.produce())
        .fold(HashMap::new(), |mut acc, g| {
            for (goods, quantity) in g.iter() {
                let cur = acc.entry(*goods).or_insert(0);
                *cur += quantity;
            }
            acc
        })
}
