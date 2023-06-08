use bevy::prelude::*;
use rand::prelude::*;

use super::{Board, TileKind, Tile, Position, Site};
use super::commands::UpgradeSite;

pub fn update_sites(
    mut commands: Commands,
    site_query: Query<(Entity, &Position, &Site)>,
    param: super::func::GoodsSupplyParam
) {
    for (entity, position, site) in site_query.iter() {
        let Some(data) = param.data.get(&site.kind) else { continue };
        let Some(next_tier) = data.tiers.get(site.tier + 1) else { continue };
        let goods = super::func::get_current_goods(&param, position.0, 6);

        if next_tier.goods_required.iter()
            // check if every goods requirement is met
            .any(|(g, v)| goods.get(g).unwrap_or(&0) < v) {
                continue;
            }
        commands.add(UpgradeSite(entity))
    }
}

pub fn spawn_map(
    mut commands: Commands,
    mut board: ResMut<Board>
) {
    let mut rng = thread_rng();
    for x in 0..8 {
        for y in 0..8 {
            let state = match rng.gen_range(0..=2) {
                0 => TileKind::Plains,
                1 => TileKind::Mountains,
                _ => TileKind::Forest
            };
            let entity = commands.spawn((
                    Position(IVec2::new(x, y)),
                    Tile(state)
                ))
                .id();
            board.tiles.insert(IVec2::new(x, y), entity);
        }
    }
}