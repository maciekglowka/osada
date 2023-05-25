use bevy::prelude::*;

use super::UiAssets;

const CURSOR_PATH: &str = "ui/cursor.png";
const FONT_PATH: &str = "ui/04B_03.ttf";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let cursor_texture = asset_server.load(CURSOR_PATH);
    asset_list.0.push(cursor_texture.clone_untyped());
    let cursor_atlas = TextureAtlas::from_grid(
        cursor_texture,
        Vec2::splat(32.),
        1,
        1,
        None,
        None
    );
    let cursor_handle = texture_atlasses.add(cursor_atlas);

    let font = asset_server.load(FONT_PATH);
    asset_list.0.push(font.clone_untyped());

    commands.insert_resource(
        UiAssets { 
            cursor_texture: cursor_handle,
            font
         }
    );
}