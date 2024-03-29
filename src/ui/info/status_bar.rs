use bevy::prelude::*;

use crate::manager::TurnState;
use crate::player::PlayerState;

use super::super::{FONT_SIZE, MENU_PADDING, UiAssets};

const HEIGHT: f32 = FONT_SIZE + 8.;

#[derive(Component)]
pub struct StatusBar;

pub fn draw(
    mut commands: Commands,
    assets: Res<UiAssets>,
    player_state: Res<PlayerState>,
    turn_state: Res<TurnState>
) {
    let container = commands.spawn((
            NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    position: UiRect { left: Val::Px(0.), top: Val::Px(0.), ..Default::default() },
                    size: Size::new(Val::Percent(100.), Val::Px(HEIGHT)),
                    padding: UiRect::all(MENU_PADDING),
                    ..Default::default()
                },
                background_color: Color::DARK_GRAY.into(),
                ..Default::default()
            },
            StatusBar
        ))
        .id();

    // TODO add more stats
    let info = commands.spawn(
            TextBundle {
                text: Text::from_section(
                    format!("Credits: {} | {}", player_state.credits, "*".repeat(turn_state.index + 1)),
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