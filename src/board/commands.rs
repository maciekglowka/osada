use bevy::prelude::*;
use bevy::ecs::system::Command;

use super::enums::SiteKind;
use super::components::{Position, Site};

pub struct PlaceSite{
    pub kind: SiteKind,
    pub v: IVec2
}
impl Command for PlaceSite {
    fn write(self, world: &mut World) {
        world.spawn((
            Site{
                kind: self.kind,
                level: 0
            },
            Position(self.v)
        ));
    }
}