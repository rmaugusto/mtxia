use bevy::prelude::*;

use crate::{shared::components::CameraLockedObject, startup::InternalAssets};

use super::TextRef;

extern crate tiled;

const PANEL_WIDTH: f32 = 180.00;
const PANEL_HEIGHT: f32 = 400.00;

fn create_text_positions(win_width: f32) -> Vec<Rect<Val>> {
    (0..40)
        .map(|i| Rect {
            top: Val::Px(i as f32 * 15.0 + 10.0),
            left: Val::Px(win_width - PANEL_WIDTH + 10.0),
            ..Default::default()
        })
        .collect()
}

pub(crate) fn create_panel(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    assets: Res<InternalAssets>,
    windows: Res<Windows>,
) {
    let win = windows.get_primary().unwrap();

    commands
        .spawn(NodeComponents {
            style: Style {
                size: Size::new(Val::Px(PANEL_WIDTH), Val::Px(PANEL_HEIGHT)),
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Px(0.0),
                    right: Val::Px(0.0),
                    ..Default::default()
                },
                ..Default::default()
            },
            material: materials.add(Color::rgb(0.25, 0.25, 0.25).into()),
            ..Default::default()
        })
        .with(CameraLockedObject);

    create_text_positions(win.width() as f32)
        .iter()
        .enumerate()
        .for_each(|(idx, p)| {
            commands
                .spawn(TextComponents {
                    style: Style {
                        position_type: PositionType::Absolute,
                        position: *p,
                        ..Default::default()
                    },
                    transform: Default::default(),
                    text: Text {
                        value: "".to_string(),
                        font: assets.text_font.clone_weak(),
                        style: TextStyle {
                            font_size: 14.0,
                            color: Color::WHITE,
                        },
                    },
                    ..Default::default()
                })
                .with(TextRef { index: idx })
                .with(CameraLockedObject);
        });
}
