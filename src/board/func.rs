use bevy::{
    ecs::system::SystemParam,
    prelude::*
};
use std::collections::HashMap;

use crate::data::SiteDataParam;
use super::{Board, Position, Tile, Site, SiteKind, ALL_DIRECTIONS};

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
    // check that the tile is unoccupied
    if param.site_query.iter()
        .any(|p| p.0 == v) {
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
        .filter(|(_, v)| v.placed_on.iter().any(|t| *t == tile.0))
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