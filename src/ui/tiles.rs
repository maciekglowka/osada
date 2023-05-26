use bevy::prelude::*;
use std::collections::HashMap;

use crate::tiles::{
    Board,
    commands::TileUpgrade,
    Position,
    Structure,
    Tile
};

use super::GameUiState;
use super::cursor::Cursor;
use super::elements::selection_menu::{SelectionMenu, SelectionMenuOption, draw_menu};
use super::events::MenuCloseEvent;

pub type MenuType = Box<dyn Structure>;

pub fn open_build_menu(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    assets: Res<super::UiAssets>,
    mut next_state: ResMut<NextState<GameUiState>>,
    cursor_query: Query<&Position, With<Cursor>>,
    board: Res<Board>,
    tile_query: Query<&Tile>
) {
    if !keys.just_pressed(KeyCode::Space) { return };

    let Ok(cursor) = cursor_query.get_single() else { return };
    let Some(tile_entity) = board.tiles.get(&cursor.0) else { return };
    let Ok(tile) = tile_query.get(*tile_entity) else { return };
    let Some(next) = tile.0.next(HashMap::new()) else { return };

    let options = vec![
        SelectionMenuOption::<MenuType>::new(
            "Jeden".to_string(), next
        )
    ];
    draw_menu(
        &mut commands,
        options,
        assets.as_ref()
    );
    next_state.set(GameUiState::BuildMenu);
}

pub fn on_close_build_menu(
    mut commands: Commands,
    mut ev_menu: EventReader<MenuCloseEvent>,
    cursor_query: Query<&Position, With<Cursor>>,
    mut menu_query: Query<&mut SelectionMenu<MenuType>>,
    mut next_state: ResMut<NextState<GameUiState>>
) {
    for ev in ev_menu.iter() {
        next_state.set(GameUiState::Cursor);
        if !ev.0 { continue };
        let Ok(mut menu) = menu_query.get_single_mut() else { continue };
        let Ok(position) = cursor_query.get_single() else { continue };
        let option = menu.get_current();
        let Some(structure) = option.value.take() else { continue };
        commands.add(TileUpgrade { hex: (*position).0, structure });
        break;
    }
}
