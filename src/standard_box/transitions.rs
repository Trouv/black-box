use crate::{
    box_internal::{
        components::{BoxOut, BoxState, Itemized, Pressable, Progression},
        BoxData,
    },
    resources::LevelNum,
    standard_box::components::{BoxReader, ProgressionPiece},
    AppState, LEVEL_ORDER,
};
use bevy::prelude::*;
use std::convert::TryFrom;
pub fn into_black_box(mut state: ResMut<State<AppState>>) {
    state.replace(AppState::StandardBox).unwrap();
}

pub fn black_box_cleanup(mut commands: Commands, ui_query: Query<(Entity, &BoxUiRoot)>) {
    for (ui_entity, box_ui_root) in ui_query.iter() {
        commands.entity(box_ui_root.0).despawn_recursive();
        commands.entity(ui_entity).despawn_recursive();
    }
}

pub fn black_box_setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    level_num: Res<LevelNum>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let level_data = BoxData::try_from(LEVEL_ORDER[(level_num.0 - 1) % LEVEL_ORDER.len()])
        .expect(format!("Unable to load level {}", level_num.0).as_str());
    let box_ = spawn_box(&level_data, &mut commands, &server);
    spawn_box_ui(
        level_data.prompt,
        &mut commands,
        &server,
        &mut materials,
        box_,
        &level_num,
    );
}

pub fn spawn_box(
    level_data: &BoxData,
    commands: &mut Commands,
    server: &Res<AssetServer>,
) -> Entity {
    commands
        .spawn_bundle((Transform::default(), GlobalTransform::identity()))
        .with_children(|parent| {
            parent.spawn_scene(server.load("models/box.glb#Scene0"));

            let box_ = parent.parent_entity();
            for (i, button_data) in level_data.buttons.iter().enumerate() {
                parent
                    .spawn_bundle((
                        Transform::from_translation(button_data.translation.clone()),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_scene(server.load("models/button_base.glb#Scene0"));
                        parent
                            .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                            .insert(button_data.button.clone())
                            .insert(Itemized {
                                collector: box_,
                                index: i,
                            })
                            .insert(Pressable::default())
                            .with_children(|parent| {
                                parent.spawn_scene(server.load("models/button_body.glb#Scene0"));
                            });
                    });
            }
        })
        .insert(BoxState::default())
        .insert(Progression {
            prompt: level_data.prompt.clone(),
            answer: Vec::new(),
        })
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
            let font = server.load("fonts/rainyhearts.ttf");
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
                .with_children(|parent| {
                    for (i, piece) in prompt.iter().enumerate() {
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
                            .insert(ProgressionPiece)
                            .insert(Itemized {
                                collector: box_,
                                index: i,
                            })
                            .with_children(|parent| {
                                parent
                                    .spawn_bundle(TextBundle {
                                        text: Text::with_section(
                                            piece.to_string(),
                                            TextStyle {
                                                font: font.clone(),
                                                font_size: 50.,
                                                color: Color::rgb(0.1, 0.1, 0.1),
                                            },
                                            TextAlignment::default(),
                                        ),
                                        ..Default::default()
                                    })
                                    .id();
                            });
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
                                font: font.clone(),
                                font_size: 50.,
                                color: Color::rgb(0.1, 0.1, 0.1),
                            },
                            TextAlignment::default(),
                        ),
                        ..Default::default()
                    });
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
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
                                    font: font.clone(),
                                    font_size: 200.,
                                    color: Color::rgb(0.36, 0.63, 0.36),
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

pub fn level_completion(
    progress_query: Query<&Progression>,
    mut state: ResMut<State<AppState>>,
    mut level_num: ResMut<LevelNum>,
) {
    for progress in progress_query.iter() {
        if progress.answer.len() >= progress.prompt.len() {
            state.replace(AppState::StandardBoxTransition).unwrap();
            level_num.0 += 1;
        }
    }
}
