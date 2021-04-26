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
    pub level_num: usize,
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
                    size: Size::new(Val::Percent(100.), Val::Percent(10.)), //Progression::default(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(BoxReader::new(box_))
            .with_children(|parent| {
                for piece in self.prompt.iter() {
                    pieces.push(
                        parent
                            .spawn_bundle(NodeBundle {
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
                                                font_size: 30.,
                                                color: Color::rgb(0.1, 0.1, 0.1),
                                            },
                                            TextAlignment {
                                                vertical: VerticalAlign::Center,
                                                horizontal: HorizontalAlign::Center,
                                            },
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
        buttons: Vec<Entity>,
    ) -> (Entity, Entity) {
        let transform = Transform::from_xyz(0., 0., 0.);

        let buttons_clone = buttons.clone();

        let box_component = BlackBox::new(buttons);

        let box_ = commands
            .spawn()
            .insert(transform)
            .insert(box_component)
            .with_children(|parent| {
                parent.spawn_scene(server.load("models/box.glb#Scene0"));
            })
            .id();
        self.box_ = Some(box_);

        let display = commands
            .spawn_bundle(TextBundle {
                style: Style {
                    position_type: PositionType::Absolute,
                    size: Size {
                        width: Val::Percent(100.),
                        height: Val::Percent(50.),
                    },
                    position: Rect {
                        top: Val::Percent(10.),
                        ..Default::default()
                    },
                    ..Default::default()
                },
                text: Text::with_section(
                    "display".to_string(),
                    TextStyle {
                        font: server.load("fonts/rainyhearts.ttf"),
                        font_size: 60.,
                        color: GREEN,
                    },
                    TextAlignment {
                        vertical: VerticalAlign::Center,
                        horizontal: HorizontalAlign::Center,
                    },
                ),
                ..Default::default()
            })
            .insert(BoxReader::new(box_))
            .id();

        for button in buttons_clone {
            commands.entity(button).insert(Parent(box_));
        }

        (box_, display)
    }

    fn init_buttons(&self, commands: &mut Commands, server: &Res<AssetServer>) -> Vec<Entity> {
        let mut button_entities = Vec::new();

        for button in &self.buttons {
            button_entities.push(
                commands
                    .spawn()
                    .insert(Transform::from_translation(button.translation.clone()))
                    .insert(button.button.clone())
                    .with_children(|parent| {
                        parent.spawn_scene(server.load("models/button.glb#Scene0"));
                    })
                    .id(),
            );
        }

        button_entities
    }

    fn init_level_counter(&self, commands: &mut Commands, server: &Res<AssetServer>) -> Entity {
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
                    format!("{}/{}", ((self.level_num - 1) % 10) + 1, LEVEL_ORDER.len()),
                    TextStyle {
                        font: server.load("fonts/rainyhearts.ttf"),
                        font_size: 30.,
                        color: Color::rgb(0.1, 0.1, 0.1),
                    },
                    TextAlignment {
                        horizontal: HorizontalAlign::Right,
                        vertical: VerticalAlign::Top,
                    },
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
        self.level_num = level_num;

        let level_counter = self.init_level_counter(commands, server);
        let buttons = self.init_buttons(commands, server);
        let (box_, display) = self.init_box(commands, server, buttons.clone());
        let progress = self.init_progress(commands, server, materials, box_);

        self.entities.push(level_counter);
        self.entities.push(box_);
        self.entities.push(display);
        self.entities.push(progress);
        progress
    }
}
