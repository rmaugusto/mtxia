use bevy::app;
use bevy::asset::AssetServer;
use bevy::diagnostic::{Diagnostics, FrameTimeDiagnosticsPlugin};
use bevy::ecs::{Commands, Query, Res};
use bevy::render::color::Color;
use bevy::text::TextStyle;
use bevy::ui::entity::TextComponents;
use bevy::ui::prelude::AlignSelf;
use bevy::ui::widget::Text;
use bevy::ui::Style;

use crate::{shared::components::CameraLockedObject, startup::InternalAssets};

pub(crate) struct FpsText;

pub(crate) fn text_update_system(
    diagnostics: Res<Diagnostics>,
    mut query: Query<(&mut Text, &FpsText)>,
) {
    for (mut text, _tag) in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.value = format!("FPS: {:.2}", average);
            }
        }
    }
}

pub(crate) fn create_fps_text(mut commands: Commands, assets: Res<InternalAssets>) {
    commands
        .spawn(TextComponents {
            style: Style {
                align_self: AlignSelf::FlexEnd,
                ..Default::default()
            },
            transform: Default::default(),
            text: Text {
                value: "FPS:".to_string(),
                font: assets.text_font.clone_weak(),
                style: TextStyle {
                    font_size: 14.0,
                    color: Color::WHITE,
                },
            },
            ..Default::default()
        })
        .with(CameraLockedObject)
        .with(FpsText);
}
