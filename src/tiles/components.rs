use bevy::prelude::*;

use crate::hex::Hex;

use super::models::Structure;

#[derive(Component)]
pub struct Position(pub Hex);

#[derive(Component)]
pub struct Tile(pub Box<dyn Structure>);
