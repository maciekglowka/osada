use bevy::prelude::*;
use std::collections::HashMap;

use crate::board::{Tile, Site, Position};
use crate::data::SiteDataParam;

use super::super::{FONT_SIZE, MENU_PADDING, UiAssets};
use super::super::cursor::Cursor;

const HEIGHT: f32 = FONT_SIZE + 8.;

#[derive(Component)]
pub struct ContextBar;

pub fn draw(
    mut commands: Commands,
    assets: Res<UiAssets>,
    cursor_query: Query<&Position, With<Cursor>>,
    site_query: Query<(&Position, &Site)>,
    tile_query: Query<(&Position, &Tile)>,
    site_data: SiteDataParam
) {
    let Ok(cursor) = cursor_query.get_single() else { return };

    let tile = tile_query.iter()
        .find(|(p, _)| p.0 == cursor.0);
    let tile_info = match tile {
        None => "Unknown land",
        Some((_, tile)) => tile.0.name()
    };

    let site = site_query.iter()
        .find(|(p, _)| p.0 == cursor.0);
    let site_info = match site {
        None => String::new(),
        Some((_, site)) => {
            let state = site_data.get(&site.kind);
            let goods = match state {
                None => String::new(),
                Some(state) => {
                    let next_info = match state.tiers.get(site.tier + 1) {
                        None => String::new(),
                        Some(next) => format!("Next tier: {:?}", next.goods_required)
                    };
                    format!(
                        "Produces: {:?}, Credits: {} {}",
                        state.tiers[site.tier].goods_produced,
                        state.tiers[site.tier].credits,
                        next_info
                    )
                }
            };
            format!("{}, Tier: {} {}", site.kind.name(), site.tier, goods)
        }
    };

    let container = commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { left: Val::Px(0.), bottom: Val::Px(0.), ..Default::default() },
                    size: Size::new(Val::Percent(100.), Val::Px(HEIGHT)),
                    padding: UiRect::all(MENU_PADDING),
                    ..Default::default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..Default::default()
            },
            ContextBar
        ))
        .id();
    let info = commands.spawn(
        TextBundle {
            text: Text::from_section(
                format!("{} {}", tile_info, site_info),
                TextStyle { 
                    font: assets.font.clone(),
                    font_size: FONT_SIZE,
                    color: Color::WHITE
                }
            ),
            ..Default::default()
        }
    )
    .id();
commands.entity(container).add_child(info);
}