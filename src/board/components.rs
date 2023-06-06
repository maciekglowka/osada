use bevy::prelude::*;

use super::enums::{SiteKind, TileKind};

#[derive(Component, Debug)]
pub struct Site{
    pub kind: SiteKind,
    pub level: usize
}

#[derive(Component, Debug)]
pub struct Position(pub IVec2);

#[derive(Component, Debug)]
pub struct Tile(pub TileKind);
