use bevy::prelude::*;

use crate::states::MainState;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerState>()
            .add_event::<InvalidCommandEvent>()
            .add_system(
                player_init.in_schedule(OnEnter(MainState::Game))
            );
    }
}

pub struct InvalidCommandEvent;


#[derive(Default, Resource)]
pub struct PlayerState {
    pub credits: i32,
    pub goods_range: u32
}
impl PlayerState {
    // pub fn pay(&mut self, amount: u32) -> bool {
    //     if self.credits < amount { return false }
    //     self.credits -= amount;
    //     return true
    // }
}

fn player_init(
    mut state: ResMut<PlayerState>
) {
    state.credits = 10;
    state.goods_range = 6;
}