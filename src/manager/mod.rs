use bevy::prelude::*;

use crate::states::MainState;

pub mod commands;
mod turn;

pub use turn::{ProgressTurnEvent, NextTurnEvent, TurnState};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<NextTurnEvent>()
            .add_event::<ProgressTurnEvent>()
            .init_resource::<turn::TurnState>()
            .add_system(game_start.in_schedule(OnEnter(MainState::Game)))
            .add_system(turn::progress_turn.in_set(OnUpdate(MainState::Game)))
            .add_system(turn::update_credits.run_if(on_event::<NextTurnEvent>()));
    }
}

fn game_start(
    mut state: ResMut<turn::TurnState>
) {
    state.index = 0;
    state.length = 4;
}

