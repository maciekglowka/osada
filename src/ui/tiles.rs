use bevy::prelude::*;

use crate::tiles::{
    Board,
    check_goods,
    commands::{InsertTile, TileUpgrade},
    Position,
    Structure,
    Tile
};

use super::GameUiState;
use super::cursor::Cursor;
use super::elements::selection_menu::{SelectionMenu, SelectionMenuOption, draw_menu};
use super::events::MenuCloseEvent;

pub type MenuType = Box<dyn Structure>;

pub fn add_or_upgrade(
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
    let Some(tile_entity) = board.tiles.get(&cursor.0) else { 
        // no tile at this position
        // try insert a new one from the board queue
        commands.add(InsertTile { hex: cursor.0 });
        return
     };

    let Ok(tile) = tile_query.get(*tile_entity) else { return };

    let goods = check_goods(cursor.0, &tile_query, board.as_ref());
    let next = tile.0.get_next(&goods);
    if next.len() == 0 { return };
    
    let options = next.into_iter()
        .map(|a| SelectionMenuOption::<MenuType>::new(
            a.name().to_string(), a
        ))
        .collect();

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
