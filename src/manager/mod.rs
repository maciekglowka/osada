use bevy::prelude::*;

use crate::states::{MainState, GameState};

pub struct ManagerPlugin;

impl Plugin for ManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(game_start.in_schedule(OnEnter(MainState::Game)))
            .add_system(game_end.in_schedule(OnExit(MainState::Game)));
    }
}

fn game_start(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::Board);
}

fn game_end(
    mut next_state: ResMut<NextState<GameState>>
) {
    next_state.set(GameState::None);
}
