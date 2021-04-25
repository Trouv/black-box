use crate::components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece};
use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, io, path::Path};

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
    transform: Transform,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LevelData {
    prompt: Vec<BoxOut>,
    buttons: Vec<ButtonData>,
    #[serde(skip)]
    pub entities: Vec<Entity>,
}

impl TryFrom<&str> for LevelData {
    type Error = io::Error;
    fn try_from(path: &str) -> io::Result<LevelData> {
        let input_path = Path::new("assets/levels").join(path);
        let f = std::fs::File::open(&input_path)?;
        Ok(from_reader(f)?)
    }
}

pub const GREEN: Color = Color::rgb(0.36, 0.63, 0.36);
const PIXEL_X: u32 = 1;
const PIXEL_Y: u32 = 1;

impl LevelData {
    fn init_progress(
        &self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        box_: Entity,
    ) -> (Entity, Vec<Entity>) {
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
                for (i, piece) in self.prompt.iter().enumerate() {
                    pieces.push(
                        parent
                            .spawn_bundle(NodeBundle {
                                material: ColorMaterial::color(Color::rgb(0.9, 0.9, 0.9)),
                                ..Default::default()
                            })
                            .insert(ProgressionPiece(piece.clone()))
                            .with_children(|parent| {
                                parent.spawn_bundle(TextBundle {
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

        (progression, pieces)
    }

    fn init_box(
        &self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        buttons: Vec<Entity>,
    ) -> (Entity, Entity) {
        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);

        let font = server.load_untyped("fonts/rainyhearts.ttf");

        let buttons_clone = buttons.clone();

        let mut box_ = BlackBox::new(buttons);
        let reader_id = box_.output_channel.register_reader();
        let gltf_handle = server.load_untyped("models/box.glb");

        let box_ = commands
            .spawn()
            .insert(transform)
            .insert(box_)
            .insert(gltf_handle)
            .id();

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
                        font,
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

        let gltf_handle = server.load_untyped("models/button.glb");

        for button in &self.buttons {
            button_entities.push(
                commands
                    .spawn()
                    .insert(gltf_handle.clone())
                    .insert(button.transform.clone())
                    .insert(button.button.clone())
                    .id(),
            );
        }

        button_entities
    }

    fn init_level_counter(
        &self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        level_num: usize,
    ) -> Entity {
        let font = server.load_untyped("fonts/rainyhearts.ttf");

        world.push((
            UiTransform::new(
                "level_counter".to_string(),
                Anchor::TopRight,
                Anchor::TopRight,
                PIXEL_X * -3.,
                0.,
                0.,
                20. * PIXEL_X,
                16. * PIXEL_Y,
            ),
            UiText::new(
                Some(font),
                format!("{}/{}", ((level_num - 1) % 10) + 1, LEVEL_ORDER.len()),
                [0.1, 0.1, 0.1, 1.],
                PIXEL_X * 10.,
                LineMode::Single,
                Anchor::MiddleRight,
            ),
        ))
    }

    pub fn init(
        &mut self,
        commands: &mut Commands,
        server: &Res<AssetServer>,
        level_num: usize,
    ) -> Entity {
        let level_counter = self.init_level_counter(&mut commands, &server, level_num);
        let buttons = self.init_buttons(&mut commands, &server);
        let (box_, display) = self.init_box(&mut commands, &server, buttons.clone());
        let (progress, pieces) = self.init_progress(&mut commands, &server, box_);

        self.entities.push(level_counter);
        self.entities.extend(buttons);
        self.entities.push(box_);
        self.entities.push(display);
        self.entities.push(progress);
        self.entities.extend(pieces);
        progress
    }
}
