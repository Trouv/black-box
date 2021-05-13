use crate::{
    box_internal::components::{BoxState, Itemized, Progression},
    standard_box::components::{Active, BoxOutDisplay, BoxReference, BoxUiRoot, ProgressionPiece},
    AppState,
};
use bevy::prelude::*;

pub fn despawn_box_ui(mut commands: Commands, ui_query: Query<Entity, With<BoxUiRoot>>) {
    for ui_root in ui_query.iter() {
        commands.entity(ui_root).despawn_recursive();
    }
}

pub fn deactivate_box(
    mut commands: Commands,
    active_box_query: Query<Entity, (With<Active>, Or<(With<BoxState>, With<Progression>)>)>,
) {
    for entity in active_box_query.iter() {
        commands.entity(entity).remove::<Active>();
    }
}

pub fn spawn_box_ui(
    mut commands: Commands,
    active_prog_query: Query<(Entity, &Progression), (With<Active>, With<BoxState>)>,
    server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let transparent = materials.add(ColorMaterial::color(Color::NONE));
    for (box_entity, progression) in active_prog_query.iter() {
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
                        for (i, piece) in progression.get_prompt().iter().enumerate() {
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
                                    collector: box_entity,
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
                            .insert(BoxReference::new(box_entity));
                    });
            })
            .insert(BoxUiRoot);
    }
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
