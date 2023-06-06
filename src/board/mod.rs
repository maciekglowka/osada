use bevy::prelude::*;
use std::collections::{HashMap, HashSet};

use crate::states::MainState;

pub mod commands;
mod components;
mod enums;
pub mod func;
mod systems;

pub use components::{Position, Tile, Site};
pub use enums::{SiteKind, TileKind, Goods};

pub struct BoardPlugin;
impl Plugin for BoardPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Board>()
            .add_system(systems::spawn_map.in_schedule(OnEnter(MainState::Game)));
    }
}

#[derive(Default, Resource)]
pub struct Board {
    pub tiles: HashMap<IVec2, Entity>,
    pub updated: HashSet<Entity>
}


pub const ORTHO_DIRECTIONS: [IVec2; 4] = [
    IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y
];

pub const ALL_DIRECTIONS: [IVec2; 8] = [
    IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y,
    IVec2::new(1, 1), IVec2::new(-1, 1), IVec2::new(1, -1), IVec2::new(-1, -1),
];