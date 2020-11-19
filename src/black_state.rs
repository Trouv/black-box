use amethyst::{
    assets::{AssetStorage, Loader},
    core::{
        math::base::Vector3,
        transform::{Parent, Transform},
    },
    ecs::{Entity, World, WorldExt},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{Anchor, FontHandle, LineMode, TtfFormat, UiImage, UiText, UiTransform},
    window::ScreenDimensions,
};

use crate::components;

/// A dummy game state that shows 3 sprites.
pub struct BlackState;

impl SimpleState for BlackState {
    // Here, we define hooks that will be called throughout the lifecycle of our game state.
    //
    // In this example, `on_start` is used for initializing entities
    // and `handle_state` for managing the state transitions.
    //
    // For more state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle

    /// The state is initialized with:
    /// - a camera centered in the middle of the screen.
    /// - 3 sprites places around the center.
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Get the screen dimensions so we can initialize the camera and
        // place our sprites correctly later. We'll clone this since we'll
        // pass the world mutably to the following functions.
        let dimensions = (*world.read_resource::<ScreenDimensions>()).clone();

        // Place the camera
        init_camera(world, &dimensions);

        // Load our sprites and display them
        let button_sprites = load_button_sprites(world);
        let box_sprite = load_box_sprite(world);
        let buttons = init_buttons(world, button_sprites);
        let box_ = init_box(world, box_sprite, buttons, &dimensions);
        init_progress(world, box_, &dimensions);

        //create_ui_example(world);
    }

    /// The following events are handled:
    /// - The game state is quit when either the close button is clicked or when the escape key is pressed.
    /// - Any other keypress is simply logged to the console.
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            //if let Some(event) = get_key(&event) {
            ////info!("handling key event: {:?}", event);
            //}

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

/// Creates a camera entity in the `world`.
///
/// The `dimensions` are used to center the camera in the middle
/// of the screen, as well as make it cover the entire screen.
fn init_camera(world: &mut World, dimensions: &ScreenDimensions) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() * 0.5, dimensions.height() * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(dimensions.width(), dimensions.height()))
        .with(transform)
        .build();
}

/// Loads and splits the `logo.png` image asset into 3 sprites,
/// which will then be assigned to entities for rendering them.
///
/// The provided `world` is used to retrieve the resource loader.
fn load_button_sprites(world: &mut World) -> SpriteRender {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
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

    // Load the spritesheet definition file, which contains metadata on our
    // spritesheet texture.
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

    // Create our sprite renders. Each will have a handle to the texture
    // that it renders from. The handle is safe to clone, since it just
    // references the asset.
    SpriteRender {
        sprite_sheet: sheet_handle.clone(),
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
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0,
    }
}

/// Creates an entity in the `world` for each of the provided `sprites`.
/// They are individually placed around the center of the screen.
fn init_buttons(world: &mut World, button_sprite: SpriteRender) -> Vec<Entity> {
    let button_count = 3;

    let mut transforms = Vec::<Transform>::new();

    for i in 0..button_count {
        // Center our sprites around the center of the window
        let x = (i as f32 - 1.) * (100. / (1. + button_count as f32));
        let y = 0.;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 1.);
        transforms.push(transform)
    }
    fn inc(s: components::BoxState) -> components::BoxResult {
        let mut s = s.clone();
        s[0] += 1.;
        (s, None)
    }

    fn dec(s: components::BoxState) -> components::BoxResult {
        let mut s = s.clone();
        s[0] -= 1.;
        (s, None)
    }

    fn out(s: components::BoxState) -> components::BoxResult {
        (s, Some(components::BoxOut::Int(s[0] as i32)))
    }

    vec![
        world
            .create_entity()
            .with(button_sprite.clone())
            .with(transforms.remove(0))
            .with(components::Button::new(Some(dec)))
            .build(),
        world
            .create_entity()
            .with(button_sprite.clone())
            .with(transforms.remove(0))
            .with(components::Button::new(Some(out)))
            .build(),
        world
            .create_entity()
            .with(button_sprite.clone())
            .with(transforms.remove(0))
            .with(components::Button::new(Some(inc)))
            .build(),
    ]
}

fn init_box(
    world: &mut World,
    box_sprite: SpriteRender,
    buttons: Vec<Entity>,
    dimensions: &ScreenDimensions,
) -> Entity {
    let pixel = dimensions.width() / 100.;

    let mut transform = Transform::default();
    transform.set_translation_xyz(dimensions.width() / 2., dimensions.width() / 2., -1.);
    transform.set_scale(Vector3::new(pixel, pixel, 1.));

    //let scale = WIDTH / 100.;

    //transform.set_scale(Vector3::new(scale, scale, 1.));
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
        pixel * 100.,
        pixel * 100.,
    );

    let mut box_ = components::BlackBox::new(buttons);
    let reader_id = box_.output_channel.register_reader();

    let box_ = world
        .create_entity()
        .with(box_sprite.clone())
        .with(transform)
        .with(box_)
        .with(ui_transform)
        .build();

    world
        .create_entity()
        .with(UiTransform::new(
            "display".to_string(),
            Anchor::Middle,
            Anchor::Middle,
            0.,
            pixel * 36.,
            0.,
            pixel * 30.,
            pixel * 10.,
        ))
        .with(UiText::new(
            font,
            "test".to_string(),
            [0.5, 1.0, 0.5, 1.0],
            pixel * 13.9,
            LineMode::Single,
            Anchor::Middle,
        ))
        .with(components::BoxReader::new(box_, reader_id))
        .with(Parent::new(box_))
        //.with(components::BoxDisplay)
        .build();

    let mut parent_storage = world.write_storage::<Parent>();

    for button in buttons_clone {
        parent_storage.insert(button, Parent::new(box_)).unwrap();
    }

    box_
}

fn init_progress(world: &mut World, box_: Entity, dimensions: &ScreenDimensions) {
    let pixel = dimensions.width() / 100.;

    let prompt = vec![
        components::BoxOut::Int(0),
        components::BoxOut::Int(2),
        components::BoxOut::Int(1),
        components::BoxOut::Int(0),
        components::BoxOut::Int(2),
        components::BoxOut::Int(3),
        components::BoxOut::Int(-1),
        components::BoxOut::Int(0),
    ];

    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/rainyhearts.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    let prog_reader = world
        .write_storage::<components::BlackBox>()
        .get_mut(box_)
        .unwrap()
        .output_channel
        .register_reader();

    let progression = world
        .create_entity()
        .with(components::Progression::default())
        .with(UiTransform::new(
            "progression_1".to_string(),
            Anchor::TopMiddle,
            Anchor::TopMiddle,
            0.,
            0.,
            0.,
            pixel * 100.,
            pixel * 8.,
        ))
        .with(components::BoxReader::new(box_, prog_reader))
        .build();

    let mut pieces = Vec::<Entity>::new();
    for (i, piece) in prompt.iter().enumerate() {
        pieces.push(
            world
                .create_entity()
                .with(UiTransform::new(
                    format!("prog_piece_{}", i),
                    Anchor::Middle,
                    Anchor::Middle,
                    pixel * ((-4. * (prompt.len() as f32 - 1.)) + (8. * i as f32)),
                    0.,
                    0.,
                    8. * pixel,
                    8. * pixel,
                ))
                .with(UiText::new(
                    font.clone(),
                    piece.to_string(),
                    [0.3, 0.3, 0.3, 1.],
                    pixel * 6.9,
                    LineMode::Single,
                    Anchor::Middle,
                ))
                .with(UiImage::SolidColor([0.9, 0.9, 0.9, 1.]))
                .with(Parent::new(progression))
                .with(components::ProgressionPiece(piece.clone()))
                .build(),
        );
    }

    let mut prog_storage = world.write_storage::<components::Progression>();

    prog_storage.get_mut(progression).unwrap().prompt = pieces;
}
