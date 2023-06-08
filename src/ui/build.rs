use bevy::prelude::*;

use crate::board::{
    Board,
    Position,
    SiteKind,
    Tile,
    func::{PossibleSitesParam, possible_sites}
};
use crate::manager::commands::ConstructSite;

use super::{FONT_SIZE, MENU_PADDING, UiAssets, GameUiState};
use super::cursor::Cursor;
use super::elements::select_menu::{SelectMenu, SelectMenuOption, draw_menu};
use super::events::MenuCloseEvent;

pub fn open_menu(
    mut commands: Commands,
    keys: Res<Input<KeyCode>>,
    assets: Res<super::UiAssets>,
    cursor_query: Query<&Position, With<Cursor>>,
    param: PossibleSitesParam,
    mut next_state: ResMut<NextState<GameUiState>>,
) {
    if !keys.just_pressed(KeyCode::Space) { return };
    let Ok(cursor) = cursor_query.get_single() else { return };

    let possible = possible_sites(&param, cursor.0);
    let options: Vec<_> = possible.iter()
        .map(|k| SelectMenuOption::<SiteKind>::new(k.name().into(), *k))
        .collect();
    if options.len() == 0 { return }
    draw_menu(
        &mut commands,
        options,
        assets.as_ref()
    );
    next_state.set(GameUiState::BuildMenu);
}

pub fn on_close_menu(
    mut commands: Commands,
    mut ev_menu: EventReader<MenuCloseEvent>,
    cursor_query: Query<&Position, With<Cursor>>,
    menu_query: Query<&SelectMenu<SiteKind>>,
    mut next_state: ResMut<NextState<GameUiState>>
) {
    for ev in ev_menu.iter() {
        next_state.set(GameUiState::Cursor);
        if !ev.0 { continue };
        let Ok(menu) = menu_query.get_single() else { continue };
        let Ok(position) = cursor_query.get_single() else { continue };
        let option = menu.get_current();
        commands.add(ConstructSite { v: (*position).0, kind: option.value });
        break;
    }
}

