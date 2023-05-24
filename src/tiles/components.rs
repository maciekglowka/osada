use bevy::prelude::*;

use crate::hex::Hex;

#[derive(Component)]
pub struct Position(pub Hex);

#[derive(Component)]
pub struct Tile;