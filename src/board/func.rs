use bevy::{
    ecs::system::SystemParam,
    prelude::*
};
use std::collections::HashMap;

use crate::data::SiteDataParam;
use super::{Board, Position, Tile, Site, SiteKind, ALL_DIRECTIONS, Goods};

pub fn float_dist(a: IVec2, b: IVec2) -> f32 {
    // 'real' float dist of the integer vectors
    a.as_vec2().distance(b.as_vec2())
}
pub fn manhattan_dist(a: IVec2, b: IVec2) -> u32 {
    ((b.x - a.x).abs() + (b.y - a.y).abs()) as u32
}

#[derive(SystemParam)]
pub struct PossibleSitesParam<'w, 's> {
    tile_query: Query<'w, 's, &'static Tile>,
    site_query: Query<'w, 's, &'static Position, With<Site>>,
    board: Res<'w, Board>,
    data: SiteDataParam<'w>
}

pub fn possible_sites(
    param: &PossibleSitesParam,
    v: IVec2
) -> Vec<SiteKind> {
    // check that it does not interfere with other site's range
    if param.site_query.iter()
        .any(|p| float_dist(p.0, v) < 2.99) {
            return Vec::new()
        }
    // check current tile
    let Some(tile_entity) = param.board.tiles.get(&v) else { return Vec::new() };
    let Ok(tile) = param.tile_query.get(*tile_entity) else { return Vec::new() };
    // collect neighbours as a HashMap<Kind, Count>
    let neighbours = ALL_DIRECTIONS.iter()
        .flat_map(|d| param.board.tiles.get(&(v + *d)))
        .flat_map(|e| param.tile_query.get(*e))
        .fold(HashMap::new(), |mut acc, t| {
            let cur = acc.entry(t.0).or_insert(0);
            *cur += 1;
            acc
        });
    // iterate through all possible sites
    // filter based on requirements vs. available tiles in neighbours
    param.data.all()
        .iter()
        // can be placed on this tile
        .filter(|(_, v)| v.placed_on.iter().any(|t| *t == tile.0))
        // are the neighbouring tiles requirements met
        .filter_map(|(k, v)| {
            if v.tiles_required.iter()
                .all(|(k, v)| neighbours.get(k).unwrap_or(&0) >= v) {
                    Some(*k)
                } else {
                    None
                }
        })
        .collect()
}

#[derive(SystemParam)]
pub struct GoodsSupplyParam<'w, 's> {
    pub site_query: Query<'w, 's, (&'static Position, &'static Site)>,
    pub data: SiteDataParam<'w>
}
pub fn get_current_goods(
    param: &GoodsSupplyParam,
    v: IVec2,
    range: u32
) -> HashMap<Goods, u32> {
    // get goods available in range
    param.site_query.iter()
        // get sites in range
        .filter(|s| manhattan_dist(s.0.0, v) <= range && s.0.0 != v)
        // get their current output, based on tier
        .filter_map(|s| match param.data.get(&s.1.kind) {
            Some(data) => {
                Some(&data.tiers[s.1.tier].goods_produced)
            },
            None => None
        })
        // accumulate results into a joined hashmap
        .fold(HashMap::new(), |mut acc, row| {
            for (k, v) in row.iter() {
                let cur = acc.entry(*k).or_insert(0);
                *cur += v;
            }
            acc
        })
}

pub fn sites_credit_balance(
    site_data: &SiteDataParam,
    site_query: &Query<&Site>
) -> i32 {
    let mut balance = 0;
    for site in site_query.iter() {
        let Some(data) = site_data.get(&site.kind) else { continue };
        balance += &data.tiers[site.tier].credits;
    }
    balance
}