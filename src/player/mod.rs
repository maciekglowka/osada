use bevy::prelude::*;

use crate::states::MainState;

pub mod commands;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .add_event::<InvalidCommandEvent>()
            .add_event::<NextTurnEvent>()
            .add_system(
                player_init.in_schedule(OnEnter(MainState::Game))
            );
    }
}

pub struct InvalidCommandEvent;
pub struct NextTurnEvent;


#[derive(Default, Resource)]
pub struct PlayerState {
    pub credits: u32
}
impl PlayerState {
    pub fn pay(&mut self, amount: u32) -> bool {
        if self.credits < amount { return false }
        self.credits -= amount;
        return true
    }
}

fn player_init(
    mut state: ResMut<PlayerState>
) {
    state.credits = 10;
}