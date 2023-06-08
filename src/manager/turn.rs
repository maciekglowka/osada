use bevy::prelude::*;

use crate::board::Site;
use crate::data::SiteDataParam;
use crate::player::PlayerState;

pub fn update_credits(
    site_query: Query<&Site>,
    site_data: SiteDataParam,
    mut player_state: ResMut<PlayerState>
) {
    let balance = crate::board::func::sites_credit_balance(
        &site_data, &site_query
    );
    player_state.credits += balance;
}

pub fn progress_turn(
    mut state: ResMut<TurnState>,
    mut ev_progress: EventReader<ProgressTurnEvent>,
    mut ev_next: EventWriter<NextTurnEvent>
) {
    for ev in ev_progress.iter() {
        state.index += ev.0;
        if state.index >= state.length {
            state.index -= state.length;
            ev_next.send(NextTurnEvent);
        }
    }
}


pub struct ProgressTurnEvent(pub usize);
pub struct NextTurnEvent;

#[derive(Default, Resource)]
pub struct TurnState {
    pub index: usize,
    pub length: usize
}