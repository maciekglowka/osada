use bevy::prelude::*;
use bevy::ecs::system::Command;

use super::enums::SiteKind;
use super::events::SiteUpdateEvent;
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
                tier: 0
            },
            Position(self.v)
        ));
        world.send_event(SiteUpdateEvent);
    }
}

pub struct UpgradeSite(pub Entity);
impl Command for UpgradeSite {
    fn write(self, world: &mut World) {
        let Some(mut site) = world.get_mut::<Site>(self.0) else { return };
        site.tier += 1;
        world.send_event(SiteUpdateEvent);
    }
}