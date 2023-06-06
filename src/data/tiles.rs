use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    reflect::TypeUuid
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::board::TileKind;

const FILE_PATH: &str = "data/data.tile.yaml";

#[derive(SystemParam)]
pub struct TileDataParam<'w> {
    data_assets: Res<'w, super::DataAssets>,
    tile_assets: Res<'w, Assets<TileAsset>>
}
impl<'w> TileDataParam<'w> {
    pub fn get(&self, state: &TileKind) -> Option<&TileData> {
        let tile_data = self.tile_assets.get(&self.data_assets.tile_data)?;
        tile_data.0.get(&state)
    }
}

#[derive(Debug, Default, Deserialize, TypeUuid)]
#[uuid = "a614e4e3-b20b-4ff6-99ae-1c019d9edf4c"]
pub struct TileAsset(pub HashMap<TileKind, TileData>);


#[derive(Debug, Deserialize)]
pub struct TileData {
    pub sprite: usize,
    pub color: super::RGB,
}

pub fn load_data(
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
    mut assets: ResMut<super::DataAssets>
) {
    let data: Handle<TileAsset> = asset_server.load(FILE_PATH);
    asset_list.0.push(data.clone_untyped());
    assets.tile_data = data;
}