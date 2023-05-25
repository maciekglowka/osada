use bevy::prelude::*;

use crate::states::GameState;
use crate::tiles::{TileEvent, TileEventKind};

use super::elements::selection_menu::{SelectionMenuOption, draw_menu};

pub fn open_build_menu(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    assets: Res<super::UiAssets>,
    mut next_state: ResMut<NextState<GameState>>
) {
    if !keys.just_pressed(KeyCode::Space) { return };

    let options = vec![
        SelectionMenuOption::<TileEvent>::new(
            "Jeden".to_string(), TileEvent(TileEventKind::Upgrade)
        ),
        SelectionMenuOption::<TileEvent>::new(
            "Dwa".to_string(), TileEvent(TileEventKind::Upgrade)
        ),
    ];
    draw_menu(
        &mut commands,
        options,
        assets.as_ref()
    );
    next_state.set(GameState::BuildMenu);
}