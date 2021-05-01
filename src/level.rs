use crate::components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece};
use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::Path};

pub const GREEN: Color = Color::rgb(0.36, 0.63, 0.36);

pub struct ColorHandles {
    pub white: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub transparent: Handle<ColorMaterial>,
}

pub const LEVEL_ORDER: [&str; 10] = [
    "pin_pad.ron",
    "counter.ron",
    "mod_counter.ron",
    "dec_inc.ron",
    "inc_dec.ron",
    "two_toggles.ron",
    "toggle_neg_pos.ron",
    "toggle_negout_pos.ron",
    "toggle_rot.ron",
    "binary.ron",
];

pub struct LevelNum(pub usize);

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ButtonData {
    button: Button,
    translation: Vec3,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LevelData {
    pub prompt: Vec<BoxOut>,
    pub buttons: Vec<ButtonData>,
}

impl TryFrom<&str> for LevelData {
    type Error = ron::error::Error;

    fn try_from(path: &str) -> ron::error::Result<LevelData> {
        let input_path = Path::new("assets/levels").join(path);
        let f = std::fs::File::open(&input_path)?;
        from_reader(f)
    }
}

pub fn add_colors(mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
    commands.insert_resource(ColorHandles {
        white: materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.9))),
        green: materials.add(ColorMaterial::color(GREEN)),
        transparent: materials.add(ColorMaterial::color(Color::NONE)),
    });
}

pub fn spawn_box(
    buttons: Vec<ButtonData>,
    commands: &mut Commands,
    server: &Res<AssetServer>,
) -> Entity {
    let mut button_entities: Vec<Entity> = Vec::new();
    commands
        .spawn_bundle((Transform::default(), GlobalTransform::identity()))
        .with_children(|parent| {
            parent.spawn_scene(server.load("models/box.glb#Scene0"));

            for button_data in buttons {
                parent
                    .spawn_bundle((
                        Transform::from_translation(button_data.translation.clone()),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_scene(server.load("models/button_base.glb#Scene0"));
                        button_entities.push(
                            parent
                                .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                                .insert(button_data.button.clone())
                                .with_children(|parent| {
                                    parent
                                        .spawn_scene(server.load("models/button_body.glb#Scene0"));
                                })
                                .id(),
                        )
                    });
            }
        })
        .insert(BlackBox::new(button_entities))
        .id()
}

pub struct BoxUiRoot(pub Entity);

pub fn spawn_box_ui(
    prompt: Vec<BoxOut>,
    commands: &mut Commands,
    server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    box_: Entity,
    level_num: &Res<LevelNum>,
) -> Entity {
    let transparent = materials.add(ColorMaterial::color(Color::NONE));
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::ColumnReverse,
                size: Size {
                    height: Val::Percent(100.),
                    width: Val::Percent(100.),
                },
                ..Default::default()
            },
            material: transparent.clone(),
            ..Default::default()
        })
        .with_children(|parent| {
            let mut pieces = Vec::<Entity>::new();
            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::FlexEnd,
                        size: Size {
                            height: Val::Percent(10.),
                            width: Val::Percent(100.),
                        },
                        ..Default::default()
                    },
                    material: transparent.clone(),
                    ..Default::default()
                })
                .insert(BoxReader::new(box_))
                .with_children(|parent| {
                    for piece in prompt.iter() {
                        pieces.push(
                            parent
                                .spawn_bundle(NodeBundle {
                                    style: Style {
                                        align_items: AlignItems::Center,
                                        justify_content: JustifyContent::Center,
                                        size: Size {
                                            height: Val::Percent(100.),
                                            width: Val::Percent(10. / 16. * 9.),
                                        },
                                        ..Default::default()
                                    },
                                    material: materials.add(Color::rgb(0.9, 0.9, 0.9).into()),
                                    ..Default::default()
                                })
                                .insert(ProgressionPiece(piece.clone()))
                                .with_children(|parent| {
                                    parent
                                        .spawn_bundle(TextBundle {
                                            text: Text::with_section(
                                                piece.to_string(),
                                                TextStyle {
                                                    font: server.load("fonts/rainyhearts.ttf"),
                                                    font_size: 50.,
                                                    color: Color::rgb(0.1, 0.1, 0.1),
                                                },
                                                TextAlignment::default(),
                                            ),
                                            ..Default::default()
                                        })
                                        .id();
                                })
                                .id(),
                        );
                    }
                    parent.spawn_bundle(TextBundle {
                        style: Style {
                            position_type: PositionType::Absolute,
                            position: Rect {
                                top: Val::Px(10.),
                                right: Val::Px(10.),
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                        text: Text::with_section(
                            format!("{}/{}", ((level_num.0 - 1) % 10) + 1, LEVEL_ORDER.len()),
                            TextStyle {
                                font: server.load("fonts/rainyhearts.ttf"),
                                font_size: 50.,
                                color: Color::rgb(0.1, 0.1, 0.1),
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                })
                .insert(Progression {
                    prompt: pieces.clone(),
                    answer: Vec::new(),
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        position_type: PositionType::Absolute,
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(30.),
                            ..Default::default()
                        },
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..Default::default()
                    },
                    material: transparent.clone(),
                    ..Default::default()
                })
                .with_children(|parent| {
                    parent
                        .spawn_bundle(TextBundle {
                            text: Text::with_section(
                                "".to_string(),
                                TextStyle {
                                    font: server.load("fonts/rainyhearts.ttf"),
                                    font_size: 200.,
                                    color: GREEN,
                                },
                                TextAlignment::default(),
                            ),
                            ..Default::default()
                        })
                        .insert(BoxReader::new(box_));
                });
        })
        .insert(BoxUiRoot(box_))
        .id()
}
