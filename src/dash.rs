use crate::font::FontHandle;
use bevy::prelude::*;
use bevy::{diagnostic::Diagnostics, diagnostic::FrameTimeDiagnosticsPlugin};

#[derive(Component)]
pub struct FpsText;

pub fn dash_fps_start_system(mut commands: Commands, font: Res<FontHandle>) {
    let text_style = TextStyle {
        font: font.medium.clone(),
        font_size: 16.0,
        color: Color::BLACK,
    };
    // let text_section = TextSection {
    //     value: "".to_string(),
    //     style: text_style.clone(),
    // };
    // let sections = vec![text_section.clone()];

    let get_style = |top: f32| -> Style {
        return Style {
            align_self: AlignSelf::FlexEnd,
            position_type: PositionType::Absolute,
            position: UiRect {
                top: Val::Px(top),
                left: Val::Px(2.0),
                ..default()
            },
            ..default()
        };
    };

    commands
        .spawn(TextBundle {
            style: get_style(60.),
            text: Text {
                sections: vec![TextSection {
                    value: "".to_string(),
                    style: text_style.clone(),
                }],
                ..default()
            },
            ..default()
        })
        .insert(FpsText);
}

pub fn dash_fps_system(diagnostics: Res<Diagnostics>, mut query: Query<&mut Text, With<FpsText>>) {
    for mut text in query.iter_mut() {
        if let Some(fps) = diagnostics.get(FrameTimeDiagnosticsPlugin::FPS) {
            if let Some(average) = fps.average() {
                text.sections[0].value = format!("fps {:.0}", average);
            }
        }
    }
}
