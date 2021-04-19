use crate::{
    black_state::{CAM_RES_X, CAM_RES_Y},
    components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece},
};
use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

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
    type Error = amethyst::Error;
    fn try_from(path: &str) -> amethyst::Result<LevelData> {
        let input_path = application_root_dir()?.join("assets/levels").join(path);
        let f = std::fs::File::open(&input_path)?;
        Ok(from_reader(f)?)
    }
}

pub const GREEN: Color = Color::rgb(0.36, 0.63, 0.36);

impl LevelData {
    fn init_progress(
        &self,
        world: &mut World,
        resources: &Resources,
        box_: Entity,
        dimensions: &ScreenDimensions,
    ) -> (Entity, Vec<Entity>) {
        let pixel_x = dimensions.width() / CAM_RES_X;
        let pixel_y = dimensions.height() / CAM_RES_Y;

        let font: Handle<FontAsset> = resources
            .get::<DefaultLoader>()
            .unwrap()
            .load("fonts/rainyhearts.ttf");

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
                pixel_x * 100.,
                pixel_y * 16.,
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
                    pixel_x * ((-6. * (self.prompt.len() as f32 - 1.)) + (12. * i as f32)),
                    0.,
                    0.,
                    12. * pixel_x,
                    16. * pixel_y,
                ),
                UiText::new(
                    Some(font.clone()),
                    piece.to_string(),
                    [0.1, 0.1, 0.1, 1.0],
                    pixel_x * 10.,
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
        world: &mut World,
        resources: &Resources,
        buttons: Vec<Entity>,
        dimensions: &ScreenDimensions,
    ) -> (Entity, Entity) {
        let pixel_y = dimensions.height() / CAM_RES_Y;

        let mut transform = Transform::default();
        transform.set_translation_xyz(0., 0., 0.);

        let font: Handle<FontAsset> = resources
            .get::<DefaultLoader>()
            .unwrap()
            .load("fonts/rainyhearts.ttf");

        let buttons_clone = buttons.clone();

        let mut box_ = BlackBox::new(buttons);
        let reader_id = box_.output_channel.register_reader();
        let gltf_handle: Handle<Prefab> = resources
            .get::<DefaultLoader>()
            .unwrap()
            .load("models/box.glb");

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
                pixel_y * 60.,
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

    fn init_buttons(&self, world: &mut World, resources: &Resources) -> Vec<Entity> {
        let mut button_entities = Vec::new();

        let gltf_handle: Handle<Prefab> = resources
            .get::<DefaultLoader>()
            .unwrap()
            .load("models/button.glb");

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
        world: &mut World,
        resources: &Resources,
        dimensions: &ScreenDimensions,
        level_num: usize,
    ) -> Entity {
        let pixel_x = dimensions.width() / CAM_RES_X;
        let pixel_y = dimensions.height() / CAM_RES_Y;

        let font: Handle<FontAsset> = resources
            .get::<DefaultLoader>()
            .unwrap()
            .load("fonts/rainyhearts.ttf");

        world.push((
            UiTransform::new(
                "level_counter".to_string(),
                Anchor::TopRight,
                Anchor::TopRight,
                pixel_x * -3.,
                0.,
                0.,
                20. * pixel_x,
                16. * pixel_y,
            ),
            UiText::new(
                Some(font),
                format!("{}/{}", ((level_num - 1) % 10) + 1, LEVEL_ORDER.len()),
                [0.1, 0.1, 0.1, 1.],
                pixel_x * 10.,
                LineMode::Single,
                Anchor::MiddleRight,
            ),
        ))
    }

    pub fn init(&mut self, world: &mut World, resources: &Resources, level_num: usize) -> Entity {
        let dimensions = resources.get::<ScreenDimensions>().unwrap().clone();

        let level_counter = self.init_level_counter(world, resources, &dimensions, level_num);
        let buttons = self.init_buttons(world, resources);
        let (box_, display) = self.init_box(world, resources, buttons.clone(), &dimensions);
        let (progress, pieces) = self.init_progress(world, resources, box_, &dimensions);

        self.entities.push(level_counter);
        self.entities.extend(buttons);
        self.entities.push(box_);
        self.entities.push(display);
        self.entities.push(progress);
        self.entities.extend(pieces);
        progress
    }
}
