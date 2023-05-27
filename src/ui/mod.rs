use bevy::prelude::*;

use crate::states::MainState;

mod assets;
mod cursor;
mod elements;
mod events;
mod tiles;

pub use cursor::Cursor;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameUiState>()
        .add_event::<events::MenuCloseEvent>()
        .add_startup_system(assets::load_assets)
            .add_systems(
                (cursor::spawn_cursor, game_start)
                .in_schedule(OnEnter(MainState::Game))
            )
            .add_systems(
                (clear::<cursor::Cursor>, game_end)
                .in_schedule(OnExit(MainState::Game))
            )
            .add_systems(
                (cursor::cursor_input, tiles::add_or_upgrade)
                .in_set(OnUpdate(GameUiState::Cursor))
            )
            .add_systems(
                (
                    elements::selection_menu::update_menu::<tiles::MenuType>,
                    elements::selection_menu::close_menu::<tiles::MenuType>,
                    tiles::on_close_build_menu
                )
                .in_set(OnUpdate(GameUiState::BuildMenu))
            )
            .add_system(
                clear::<elements::selection_menu::SelectionMenu<tiles::MenuType>>
                    .in_schedule(OnExit(GameUiState::BuildMenu))
            );
    }
}

#[derive(Clone, Debug, Default, Hash, Eq, States, PartialEq)]
pub enum GameUiState {
    #[default]
    None,
    Cursor,
    BuildMenu
}

#[derive(Resource)]
pub struct UiAssets {
    pub cursor_texture: Handle<TextureAtlas>,
    pub font: Handle<Font>
}

fn game_start(
    mut next_state: ResMut<NextState<GameUiState>>
) {
    next_state.set(GameUiState::Cursor);
}

fn game_end(
    mut next_state: ResMut<NextState<GameUiState>>
) {
    next_state.set(GameUiState::None);
}

fn clear<T: Component> (
    mut commands: Commands,
    query: Query<Entity, With<T>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}