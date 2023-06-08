use bevy::{
    ecs::system::SystemParam,
    prelude::*,
    reflect::TypeUuid
};
use serde::Deserialize;
use std::collections::HashMap;

use crate::board::{
    Goods,
    SiteKind, TileKind
};

const FILE_PATH: &str = "data/data.site.yaml";

#[derive(SystemParam)]
pub struct SiteDataParam<'w> {
    data_assets: Res<'w, super::DataAssets>,
    site_assets: Res<'w, Assets<SiteAsset>>
}
impl<'w> SiteDataParam<'w> {
    pub fn get(&self, kind: &SiteKind) -> Option<&SiteData> {
        let data = self.site_assets.get(&self.data_assets.site_data)?;
        data.0.get(&kind)
    }
    pub fn all(&self) -> &HashMap<SiteKind, SiteData> {
        &self.site_assets.get(&self.data_assets.site_data).unwrap().0
    }
}

#[derive(Debug, Default, Deserialize, TypeUuid)]
#[uuid = "b0915040-db63-4c6b-8d5b-0d2791518e81"]
pub struct SiteAsset(pub HashMap<SiteKind, SiteData>);


#[derive(Debug, Deserialize)]
pub struct SiteState{
    pub goods_required: HashMap<Goods, u32>,
    pub goods_produced: HashMap<Goods, u32>,
    pub credits: i32
}


#[derive(Debug, Deserialize)]
pub struct SiteData {
    pub sprite: usize,
    pub color: super::RGB,
    pub tiles_required: HashMap<TileKind, u32>,
    pub placed_on: Vec<TileKind>,
    pub tiers: Vec<SiteState>
}

pub fn load_data(
    asset_server: Res<AssetServer>,
    mut asset_list: ResMut<crate::assets::AssetList>,
    mut assets: ResMut<super::DataAssets>
) {
    let data: Handle<SiteAsset> = asset_server.load(FILE_PATH);
    asset_list.0.push(data.clone_untyped());
    assets.site_data = data;
}