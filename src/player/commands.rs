use bevy::prelude::*;
use bevy::ecs::system::Command;

use crate::board;
use super::{InvalidCommandEvent, PlayerState, NextTurnEvent};

pub struct FundSite {
    pub kind: board::SiteKind,
    pub v: IVec2
}
impl Command for FundSite {
    fn write(self, world: &mut World) {
        if let Some(mut player_state) = world.get_resource_mut::<PlayerState>() {
            if !player_state.pay(2) { 
                world.send_event(InvalidCommandEvent);
                return
            };
        }
        let board_command = board::commands::PlaceSite {
            kind: self.kind, v: self.v
        };
        board_command.write(world);
        world.send_event(NextTurnEvent);
    }
}