use crate::{
    black_state::{CAM_RES_X, CAM_RES_Y},
    components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece},
};
use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::{Parent, Transform},
    ecs::{Entity, World, WorldExt},
    prelude::Builder,
    renderer::{ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiImage, UiText, UiTransform},
    utils::application_root_dir,
    window::ScreenDimensions,
};
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

impl LevelData {
    fn init_progress(
        &self,
        world: &mut World,
        box_: Entity,
        dimensions: &ScreenDimensions,
    ) -> (Entity, Vec<Entity>) {
        let pixel_x = dimensions.width() / CAM_RES_X;
        let pixel_y = dimensions.height() / CAM_RES_Y;

        let font: FontHandle = world.read_resource::<Loader>().load(
            "fonts/rainyhearts.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let prog_reader = world
            .write_storage::<BlackBox>()
            .get_mut(box_)
            .unwrap()
            .output_channel
            .register_reader();

        let progression = world
            .create_entity()
            .with(Progression::default())
            .with(UiTransform::new(
                "progression_1".to_string(),
                Anchor::TopMiddle,
                Anchor::TopMiddle,
                0.,
                0.,
                0.,
                pixel_x * 100.,
                pixel_y * 16.,
            ))
            .with(BoxReader::new(box_, prog_reader))
            .build();

        let mut pieces = Vec::<Entity>::new();
        for (i, piece) in self.prompt.iter().enumerate() {
            pieces.push(
                world
                    .create_entity()
                    .with(UiTransform::new(
                        format!("prog_piece_{}", i),
                        Anchor::Middle,
                        Anchor::Middle,
                        pixel_x * ((-6. * (self.prompt.len() as f32 - 1.)) + (12. * i as f32)),
                        0.,
                        0.,
                        12. * pixel_x,
                        16. * pixel_y,
                    ))
                    .with(UiText::new(
                        font.clone(),
                        piece.to_string(),
                        [0.3, 0.3, 0.3, 1.],
                        pixel_x * 10.,
                        LineMode::Single,
                        Anchor::Middle,
                    ))
                    .with(UiImage::SolidColor([0.9, 0.9, 0.9, 1.]))
                    .with(Parent::new(progression))
                    .with(ProgressionPiece(piece.clone()))
                    .build(),
            );
        }

        let mut prog_storage = world.write_storage::<Progression>();

        prog_storage.get_mut(progression).unwrap().prompt = pieces.clone();

        (progression, pieces)
    }

    fn init_box(
        &self,
        world: &mut World,
        box_sprite: SpriteRender,
        buttons: Vec<Entity>,
        dimensions: &ScreenDimensions,
    ) -> (Entity, Entity) {
        let pixel_x = dimensions.width() / CAM_RES_X;
        let pixel_y = dimensions.height() / CAM_RES_Y;

        let mut transform = Transform::default();
        transform.set_translation_xyz(213., 50., -1.);

        let font: FontHandle = world.read_resource::<Loader>().load(
            "fonts/rainyhearts.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        let buttons_clone = buttons.clone();

        let ui_transform = UiTransform::new(
            "box".to_string(),
            Anchor::BottomMiddle,
            Anchor::BottomMiddle,
            0.,
            0.,
            0.,
            pixel_x * 100.,
            pixel_y * 100.,
        );

        let mut box_ = BlackBox::new(buttons);
        let reader_id = box_.output_channel.register_reader();

        let box_ = world
            .create_entity()
            .with(box_sprite)
            .with(transform)
            .with(box_)
            .with(ui_transform)
            .build();

        let display = world
            .create_entity()
            .with(UiTransform::new(
                "display".to_string(),
                Anchor::Middle,
                Anchor::Middle,
                0.,
                pixel_y * 36.,
                0.,
                pixel_x * 30.,
                pixel_y * 10.,
            ))
            .with(UiText::new(
                font,
                "".to_string(),
                [0.5, 1.0, 0.5, 1.0],
                pixel_y * 13.,
                LineMode::Single,
                Anchor::Middle,
            ))
            .with(BoxReader::new(box_, reader_id))
            .with(Parent::new(box_))
            .build();

        let mut parent_storage = world.write_storage::<Parent>();

        for button in buttons_clone {
            parent_storage.insert(button, Parent::new(box_)).unwrap();
        }

        (box_, display)
    }

    fn init_buttons(&self, world: &mut World, button_sprite: SpriteRender) -> Vec<Entity> {
        let mut button_entities = Vec::new();

        for button in &self.buttons {
            button_entities.push(
                world
                    .create_entity()
                    .with(button_sprite.clone())
                    .with(button.transform.clone())
                    .with(button.button.clone())
                    .build(),
            )
        }

        button_entities
    }

    fn init_level_counter(
        &self,
        world: &mut World,
        dimensions: &ScreenDimensions,
        level_num: usize,
    ) -> Entity {
        let pixel_x = dimensions.width() / CAM_RES_X;
        let pixel_y = dimensions.height() / CAM_RES_Y;

        let font: FontHandle = world.read_resource::<Loader>().load(
            "fonts/rainyhearts.ttf",
            TtfFormat,
            (),
            &world.read_resource(),
        );

        world
            .create_entity()
            .with(UiTransform::new(
                "level_counter".to_string(),
                Anchor::TopRight,
                Anchor::TopRight,
                pixel_x * -3.,
                0.,
                0.,
                20. * pixel_x,
                16. * pixel_y,
            ))
            .with(UiText::new(
                font,
                format!("{}/{}", ((level_num - 1) % 10) + 1, LEVEL_ORDER.len()),
                [0.1, 0.1, 0.1, 1.],
                pixel_x * 10.,
                LineMode::Single,
                Anchor::MiddleRight,
            ))
            .build()
    }

    pub fn init(&mut self, world: &mut World, level_num: usize) -> Entity {
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        let button_sprites = load_button_sprites(world);
        let box_sprite = load_box_sprite(world);

        let level_counter = self.init_level_counter(world, &dimensions, level_num);
        let buttons = self.init_buttons(world, button_sprites);
        let (box_, display) = self.init_box(world, box_sprite, buttons.clone(), &dimensions);
        let (progress, pieces) = self.init_progress(world, box_, &dimensions);

        self.entities.push(level_counter);
        self.entities.extend(buttons);
        self.entities.push(box_);
        self.entities.push(display);
        self.entities.push(progress);
        self.entities.extend(pieces);
        progress
    }
}

fn load_button_sprites(world: &mut World) -> SpriteRender {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/button.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/button.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: 0,
    }
}

fn load_box_sprite(world: &mut World) -> SpriteRender {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/box.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/box.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: 0,
    }
}
