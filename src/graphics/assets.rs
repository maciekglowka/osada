use bevy::prelude::*;

use super::GraphicsAssets;

const ATLAS_PATH: &str = "ascii.png";
const HEX_PATH: &str = "hex.png";

pub fn load_assets(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlasses: ResMut<Assets<TextureAtlas>>,
    mut asset_list: ResMut<crate::assets::AssetList>
) {
    let sprite_texture = asset_server.load(ATLAS_PATH);
    asset_list.0.push(sprite_texture.clone_untyped());
    let atlas = TextureAtlas::from_grid(
        sprite_texture,
        Vec2::splat(10.),
        16,
        16,
        None,
        None
    );
    let sprite_handle = texture_atlasses.add(atlas);

    let hex_texture = asset_server.load(HEX_PATH);
    asset_list.0.push(hex_texture.clone_untyped());
    let hex_atlas = TextureAtlas::from_grid(
        hex_texture,
        Vec2::splat(32.),
        1,
        1,
        None,
        None
    );
    let hex_handle = texture_atlasses.add(hex_atlas);

    commands.insert_resource(
        GraphicsAssets { 
            sprite_texture: sprite_handle,
            hex_texture: hex_handle
         }
    );
}