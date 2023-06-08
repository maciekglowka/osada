use bevy::prelude::*;
use bevy::ecs::system::Command;

use crate::board;
use crate::ui::events::ReloadUiEvent;

use super::ProgressTurnEvent;

pub struct ConstructSite {
    pub kind: board::SiteKind,
    pub v: IVec2
}
impl Command for ConstructSite {
    fn write(self, world: &mut World) {
        let board_command = board::commands::PlaceSite {
            kind: self.kind, v: self.v
        };
        board_command.write(world);
        world.send_event(ProgressTurnEvent(1));
        world.send_event(ReloadUiEvent);
    }
}