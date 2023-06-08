use bevy::prelude::*;
use bevy::time::common_conditions::on_timer;
use core::time::Duration;

use crate::board::SiteKind;
use crate::states::MainState;

mod assets;
mod build;
mod cursor;
mod elements;
pub mod events;
mod info;

pub use cursor::Cursor;

const FONT_SIZE: f32 = 24.;
const MENU_PADDING: Val = Val::Px(8.);

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameUiState>()
        .add_event::<events::CursorUpdateEvent>()
        .add_event::<events::MenuCloseEvent>()
        .add_event::<events::ReloadUiEvent>()
        .configure_sets(
            (ReloadSet::CleanUp, ReloadSet::Draw).chain()
        )
        .configure_set(ReloadSet::CleanUp.run_if(on_event::<events::ReloadUiEvent>()))
        .configure_set(ReloadSet::Draw.run_if(on_event::<events::ReloadUiEvent>()))
        .add_startup_system(assets::load_assets)
            .add_systems(
                (cursor::spawn_cursor, game_start)
                .in_schedule(OnEnter(MainState::Game))
            )
            .add_systems(
                (clear::<cursor::Cursor>, game_end)
                .in_schedule(OnExit(MainState::Game))
            )
            .add_system(resource_update::<crate::player::PlayerState>)
            .add_system(
                cursor::cursor_input
                    .in_set(OnUpdate(GameUiState::Cursor))
                    .run_if(on_timer(Duration::from_secs_f32(0.1)))
            )
            .add_system(
                build::open_menu.in_set(OnUpdate(GameUiState::Cursor))
            )
            .add_systems(
                (
                    elements::select_menu::update_menu::<SiteKind>,
                    elements::select_menu::close_menu::<SiteKind>,
                    build::on_close_menu
                )
                .in_set(OnUpdate(GameUiState::BuildMenu))
            )
            .add_system(
                clear::<elements::select_menu::SelectMenu<SiteKind>>
                    .in_schedule(OnExit(GameUiState::BuildMenu))
            )
            .add_system(
                clear::<info::status_bar::StatusBar>.in_set(ReloadSet::CleanUp)
            )
            .add_system(
                info::status_bar::draw.in_set(ReloadSet::Draw)
            )
            .add_system(
                clear::<info::context_bar::ContextBar>
                    .run_if(on_event::<events::CursorUpdateEvent>())
                    .before(info::context_bar::draw)
            )
            .add_system(
                info::context_bar::draw.run_if(on_event::<events::CursorUpdateEvent>())
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

#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum ReloadSet {
    CleanUp,
    Draw
}

#[derive(Resource)]
pub struct UiAssets {
    pub cursor_texture: Handle<TextureAtlas>,
    pub font: Handle<Font>
}

fn game_start(
    mut next_state: ResMut<NextState<GameUiState>>,
    mut ev_reload: EventWriter<events::ReloadUiEvent>
) {
    next_state.set(GameUiState::Cursor);
    ev_reload.send(events::ReloadUiEvent);
}

fn game_end(
    mut next_state: ResMut<NextState<GameUiState>>
) {
    next_state.set(GameUiState::None);
}

fn resource_update<T: Resource>(
    res: Res<T>,
    mut ev_reload: EventWriter<events::ReloadUiEvent>
) {
    if res.is_changed() {
        ev_reload.send(events::ReloadUiEvent);
    }
}

fn clear<T: Component> (
    mut commands: Commands,
    query: Query<Entity, With<T>>
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
