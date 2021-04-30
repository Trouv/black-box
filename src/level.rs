use crate::components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece};
use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::Path};

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
struct ButtonData {
    button: Button,
    translation: Vec3,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LevelData {
    prompt: Vec<BoxOut>,
    buttons: Vec<ButtonData>,
    #[serde(skip)]
    pub entities: Vec<Entity>,
    #[serde(skip)]
    pub box_: Option<Entity>,
}

impl TryFrom<&str> for LevelData {
    type Error = ron::error::Error;

    fn try_from(path: &str) -> ron::error::Result<LevelData> {
        let input_path = Path::new("assets/levels").join(path);
        let f = std::fs::File::open(&input_path)?;
        from_reader(f)
    }
}

pub const GREEN: Color = Color::rgb(0.36, 0.63, 0.36);

pub struct ColorHandles {
    pub white: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
    pub transparent: Handle<ColorMaterial>,
}

pub fn add_colors(mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
    commands.insert_resource(ColorHandles {
        white: materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.9))),
        green: materials.add(ColorMaterial::color(GREEN)),
        transparent: materials.add(ColorMaterial::color(Color::NONE)),
    });
}

impl LevelData {
    fn init_progress(
        &self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        box_: Entity,
    ) -> Entity {
        let mut pieces = Vec::<Entity>::new();
        let progression = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::FlexEnd,
                    size: Size {
                        height: Val::Percent(100.),
                        width: Val::Percent(100.),
                    },
                    ..Default::default()
                },
                material: materials.add(ColorMaterial::color(Color::NONE)),
                ..Default::default()
            })
            .insert(BoxReader::new(box_))
            .with_children(|parent| {
                for piece in self.prompt.iter() {
                    pieces.push(
                        parent
                            .spawn_bundle(NodeBundle {
                                style: Style {
                                    align_items: AlignItems::Center,
                                    justify_content: JustifyContent::Center,
                                    size: Size {
                                        height: Val::Percent(10.),
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
            })
            .insert(Progression {
                prompt: pieces.clone(),
                answer: Vec::new(),
            })
            .id();

        progression
    }

    fn init_box(
        &mut self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        buttons: (Vec<Entity>, Vec<Entity>),
    ) -> (Entity, Entity) {
        let transform = Transform::from_xyz(0., 0., 0.);

        let box_component = BlackBox::new(buttons.1);

        let box_ = commands
            .spawn_bundle((transform, GlobalTransform::identity()))
            .insert(box_component)
            .with_children(|parent| {
                parent.spawn_scene(server.load("models/box.glb#Scene0"));
            })
            .id();
        self.box_ = Some(box_);

        let display = commands
            .spawn_bundle(NodeBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(30.),
                        ..Default::default()
                    },
                    position: Rect {
                        bottom: Val::Percent(60.),
                        ..Default::default()
                    },
                    justify_content: JustifyContent::Center,
                    align_items: AlignItems::Center,
                    ..Default::default()
                },
                material: materials.add(ColorMaterial::color(Color::NONE)),
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
            })
            .id();

        for button in buttons.0 {
            commands.entity(button).insert(Parent(box_));
        }

        (box_, display)
    }

    fn init_buttons(
        &self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
    ) -> (Vec<Entity>, Vec<Entity>) {
        let mut total_entities = Vec::new();
        let mut button_entities = Vec::new();

        for button in &self.buttons {
            total_entities.push(
                commands
                    .spawn_bundle((
                        Transform::from_translation(button.translation.clone()),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_scene(server.load("models/button_base.glb#Scene0"));
                        button_entities.push(
                            parent
                                .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                                .insert(button.button.clone())
                                .with_children(|parent| {
                                    parent
                                        .spawn_scene(server.load("models/button_body.glb#Scene0"));
                                })
                                .id(),
                        )
                    })
                    .id(),
            );
        }

        (total_entities, button_entities)
    }

    fn init_level_counter(
        &self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        level_num: usize,
    ) -> Entity {
        commands
            .spawn_bundle(TextBundle {
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
                    format!("{}/{}", ((level_num - 1) % 10) + 1, LEVEL_ORDER.len()),
                    TextStyle {
                        font: server.load("fonts/rainyhearts.ttf"),
                        font_size: 50.,
                        color: Color::rgb(0.1, 0.1, 0.1),
                    },
                    TextAlignment::default(),
                ),
                ..Default::default()
            })
            .id()
    }

    pub fn init(
        &mut self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        materials: &mut ResMut<Assets<ColorMaterial>>,
        level_num: usize,
    ) -> Entity {
        let level_counter = self.init_level_counter(commands, server, level_num);
        let buttons = self.init_buttons(commands, server);
        let (box_, display) = self.init_box(commands, server, materials, buttons);
        let progress = self.init_progress(commands, server, materials, box_);

        self.entities.push(level_counter);
        self.entities.push(box_);
        self.entities.push(display);
        self.entities.push(progress);
        progress
    }
}
