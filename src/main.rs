use bevy::prelude::*;

mod assets;
mod camera;
mod globals;
mod graphics;
mod hex;
mod manager;
mod states;
mod tiles;
mod ui;

fn main() {
    App::new()
        .add_plugins(
            DefaultPlugins.set(
                WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (
                            globals::WINDOW_WIDTH,
                            globals::WINDOW_HEIGHT
                        ).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                }
            ).set(
                ImagePlugin::default_nearest()
            )
        )
        .insert_resource(Msaa::Off)
        .insert_resource(ClearColor(Color::BLACK))
        .add_state::<states::MainState>()
        .add_plugin(assets::AssetsPlugin)
        .add_plugin(graphics::GraphicsPlugin)
        .add_plugin(manager::ManagerPlugin)
        .add_plugin(tiles::TilesPlugin)
        .add_plugin(ui::UiPlugin)
        .add_startup_system(camera::setup)
        .run();
}
