use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
    ui::{
        Anchor, FontHandle, LineMode, Stretch, TtfFormat, UiButtonBuilder, UiImage, UiText,
        UiTransform,
    },
    window::ScreenDimensions,
};

use crate::components;
use amethyst::core::math::base::Vector3;

use log::info;

/// A dummy game state that shows 3 sprites.
pub struct BlackState;

const WIDTH: f32 = 100.;
const HEIGHT: f32 = 150.;

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

        // Place the camera
        init_camera(world);

        // Load our sprites and display them
        let button_sprites = load_button_sprites(world);
        let box_sprite = load_box_sprite(world);
        init_buttons(world, button_sprites);
        init_box(world, box_sprite);

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
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

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
fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH * 0.5, HEIGHT * 0.5, 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
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
fn init_buttons(world: &mut World, button_sprite: SpriteRender) {
    let button_count = 3;
    for i in 0..button_count {
        // Center our sprites around the center of the window
        let x = (i as f32 + 1.) * (WIDTH / (1. + button_count as f32));
        let y = 65.;
        let mut transform = Transform::default();
        transform.set_translation_xyz(x, y, 0.);

        let pressed = i % 2 == 0;

        // Create an entity for each sprite and attach the `SpriteRender` as
        // well as the transform. If you want to add behaviour to your sprites,
        // you'll want to add a custom `Component` that will identify them, and a
        // `System` that will iterate over them. See https://book.amethyst.rs/stable/concepts/system.html
        world
            .create_entity()
            .with(button_sprite.clone())
            .with(transform)
            .with(components::Button {
                pressed,
                num: i as usize,
            })
            .build();
    }
}

fn init_box(world: &mut World, box_sprite: SpriteRender) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH / 2., 50., -1.);

    let scale = WIDTH / 100.;

    transform.set_scale(Vector3::new(scale, scale, 1.));

    world
        .create_entity()
        .with(box_sprite.clone())
        .with(transform)
        .build();
}

/// Creates a simple UI background and a UI text label
/// This is the pure code only way to create UI with amethyst.
pub fn create_ui_example(world: &mut World) {
    // this creates the simple gray background UI element.
    let ui_background = world
        .create_entity()
        .with(UiImage::SolidColor([0.6, 0.1, 0.2, 1.0]))
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            30.0,
            -30.,
            0.,
            250.,
            50.,
        ))
        .build();

    // This simply loads a font from the asset folder and puts it in the world as a resource,
    // we also get a ref to the font that we then can pass to the text label we crate later.
    let font: FontHandle = world.read_resource::<Loader>().load(
        "fonts/Bangers-Regular.ttf",
        TtfFormat,
        (),
        &world.read_resource(),
    );

    // This creates the actual label and places it on the screen.
    // Take note of the z position given, this ensures the label gets rendered above the background UI element.
    world
        .create_entity()
        .with(UiTransform::new(
            "".to_string(),
            Anchor::TopLeft,
            Anchor::TopLeft,
            40.0,
            -40.,
            1.,
            200.,
            50.,
        ))
        .with(UiText::new(
            font,
            "Hello, Amethyst UI!".to_string(),
            [1., 1., 1., 1.],
            30.,
            LineMode::Single,
            Anchor::TopLeft,
        ))
        .build();
}
