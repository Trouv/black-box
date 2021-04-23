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
        let font = server.load_untyped("fonts/rainyhearts.ttf");

        let prog_reader = world
            .entry_mut(box_)
            .unwrap()
            .get_component_mut::<BlackBox>()
            .unwrap()
            .output_channel
            .register_reader();

        let progression = world.push((
            Progression::default(),
            UiTransform::new(
                "progression_1".to_string(),
                Anchor::TopMiddle,
                Anchor::TopMiddle,
                0.,
                0.,
                0.,
                PIXEL_X * 100.,
                PIXEL_Y * 16.,
            ),
            BoxReader::new(box_, prog_reader),
        ));

        let mut pieces = Vec::<Entity>::new();
        for (i, piece) in self.prompt.iter().enumerate() {
            pieces.push(world.push((
                UiTransform::new(
                    format!("prog_piece_{}", i),
                    Anchor::Middle,
                    Anchor::Middle,
                    PIXEL_X * ((-6. * (self.prompt.len() as f32 - 1.)) + (12. * i as f32)),
                    0.,
                    0.,
                    12. * PIXEL_X,
                    16. * PIXEL_Y,
                ),
                UiText::new(
                    Some(font.clone()),
                    piece.to_string(),
                    [0.1, 0.1, 0.1, 1.0],
                    PIXEL_X * 10.,
                    LineMode::Single,
                    Anchor::Middle,
                ),
                UiImage::SolidColor([0.9, 0.9, 0.9, 1.]),
                Parent(progression),
                ProgressionPiece(piece.clone()),
            )));
        }

        world
            .entry_mut(progression)
            .unwrap()
            .get_component_mut::<Progression>()
            .unwrap()
            .prompt = pieces.clone();

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

        let box_ = world.push((transform, box_, gltf_handle));

        let display = world.push((
            UiTransform::new(
                "display".to_string(),
                Anchor::Middle,
                Anchor::Middle,
                0.,
                0.15,
                0.,
                100.,
                100.,
            )
            .into_percent(),
            UiText::new(
                Some(font),
                "".to_string(),
                GREEN,
                PIXEL_Y * 60.,
                LineMode::Single,
                Anchor::Middle,
            ),
            BoxReader::new(box_, reader_id),
        ));

        for button in buttons_clone {
            world.entry(button).unwrap().add_component(Parent(box_));
        }

        (box_, display)
    }

    fn init_buttons(&self, commands: &mut Commands, server: &Res<AssetServer>) -> Vec<Entity> {
        let mut button_entities = Vec::new();

        let gltf_handle = server.load_untyped("models/button.glb");

        for button in &self.buttons {
            button_entities.push(world.push((
                gltf_handle.clone(),
                button.transform.clone(),
                button.button.clone(),
            )));
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
