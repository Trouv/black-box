use crate::{
    box_internal::{
        actions::BoxOut,
        components::{BoxState, Itemized, Pressable, Progression},
        BoxData,
    },
    standard_box::components::{BoxOutDisplay, BoxReference, BoxUiRoot, ProgressionPiece},
    AppState, LEVEL_ORDER,
};
use bevy::prelude::*;
use heron::prelude::*;
use std::convert::TryFrom;

pub fn into_black_box(mut state: ResMut<State<AppState>>) {
    state
        .overwrite_set(AppState::StandardBox)
        .expect("Current state is StandardBox state unexpectedly.");
}

pub fn black_box_cleanup(mut commands: Commands, ui_query: Query<Entity, With<BoxUiRoot>>) {
    for ui_root in ui_query.iter() {
        commands.entity(ui_root).despawn_recursive();
    }
}

pub fn black_box_setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    let level_data =
        BoxData::try_from(LEVEL_ORDER[0]).unwrap_or_else(|_| panic!("Unable to load level {}", 1));
    let box_ = spawn_box(
        &level_data,
        Transform::from_xyz(0., 0.5, 0.),
        &mut commands,
        &server,
        &mut meshes,
        &mut standard_materials,
    );
    spawn_box_ui(
        level_data.prompt,
        &mut commands,
        &server,
        &mut color_materials,
        box_,
    );
}

pub fn spawn_box(
    level_data: &BoxData,
    base_transform: Transform,
    commands: &mut Commands,
    server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            transform: base_transform,
            ..Default::default()
        })
        .insert(BodyType::Static)
        .insert(Body::Cuboid {
            half_extends: Vec3::new(0.5, 0.5, 0.5),
        })
        .with_children(|parent| {
            parent
                .spawn_bundle((
                    Transform::from_xyz(0., 0.625, 0.),
                    GlobalTransform::identity(),
                ))
                .with_children(|parent| {
                    parent.spawn_scene(server.load("models/box.glb#Scene0"));

                    let box_ = parent.parent_entity();
                    for (i, button_data) in level_data.buttons.iter().enumerate() {
                        parent
                            .spawn_bundle((
                                Transform::from_translation(button_data.translation),
                                GlobalTransform::identity(),
                            ))
                            .with_children(|parent| {
                                parent.spawn_scene(server.load("models/button_base.glb#Scene0"));
                                parent
                                    .spawn_bundle((
                                        Transform::default(),
                                        GlobalTransform::identity(),
                                    ))
                                    .insert(button_data.button.clone())
                                    .insert(Itemized {
                                        collector: box_,
                                        index: i,
                                    })
                                    .insert(Pressable::default())
                                    .with_children(|parent| {
                                        parent.spawn_scene(
                                            server.load("models/button_body.glb#Scene0"),
                                        );
                                    });
                            });
                    }
                })
                .insert(BoxState::default())
                .insert(Progression::new(level_data.prompt.clone()));
        })
        .id()
}

pub fn spawn_box_ui(
    prompt: Vec<BoxOut>,
    commands: &mut Commands,
    server: &Res<AssetServer>,
    materials: &mut ResMut<Assets<ColorMaterial>>,
    box_: Entity,
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
                });

            parent
                .spawn_bundle(NodeBundle {
                    style: Style {
                        size: Size {
                            width: Val::Percent(100.),
                            height: Val::Percent(30.),
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
                        .insert(BoxOutDisplay)
                        .insert(BoxReference::new(box_));
                });
        })
        .insert(BoxUiRoot)
        .id()
}

pub fn level_completion(
    progress_query: Query<&Progression, Changed<Progression>>,
    mut state: ResMut<State<AppState>>,
) {
    for progress in progress_query.iter() {
        if progress.progress() >= progress.total() {
            state
                .overwrite_set(AppState::Roaming)
                .expect("Current state is Roaming unexpectedly.");
        }
    }
}
