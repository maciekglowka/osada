use bevy::prelude::*;

use crate::tiles::TileEvent;
use crate::states::{MainState, GameState};

mod assets;
mod cursor;
mod elements;
mod tiles;

pub use cursor::Cursor;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(
                cursor::spawn_cursor.in_schedule(OnEnter(MainState::Game))
            )
            .add_systems(
                (cursor::cursor_input, tiles::open_build_menu)
                .in_set(OnUpdate(GameState::Board))
            )
            .add_systems(
                (
                    elements::selection_menu::update_menu::<TileEvent>,
                    elements::selection_menu::close_menu::<TileEvent>
                )
                .in_set(OnUpdate(GameState::BuildMenu))
            );
    }
}

#[derive(Resource)]
pub struct UiAssets {
    pub cursor_texture: Handle<TextureAtlas>,
    pub font: Handle<Font>
}
