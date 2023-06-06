use bevy::prelude::*;

mod assets;
pub mod math;
mod sites;
mod tiles;

pub const TILE_SIZE: f32 = 32.;
pub const SITE_Z: f32 = 100.;
pub const TILE_Z: f32 = 50.;
pub const OVERLAY_Z: f32 = 200.;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(assets::load_assets)
            .add_system(sites::spawn_site_renderer)
            .add_system(tiles::spawn_tile_renderer)
            .add_system(tiles::update_tile);
    }
}

#[derive(Resource)]
pub struct GraphicsAssets {
    pub tiles_texture: Handle<TextureAtlas>
}
