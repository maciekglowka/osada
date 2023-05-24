use bevy::prelude::*;

use crate::states::MainState;

mod assets;
mod cursor;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(
                cursor::spawn_cursor.in_schedule(OnEnter(MainState::Game))
            )
            .add_system(
                cursor::cursor_input.in_set(OnUpdate(MainState::Game))
            );
    }
}

#[derive(Resource)]
pub struct UiAssets {
    pub cursor_texture: Handle<TextureAtlas>
}
