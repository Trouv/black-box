use amethyst::{
    core::transform::TransformBundle,
    gltf::GltfSceneLoaderSystemDesc,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderShaded3D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    ui::{RenderUi, UiBundle},
    utils::application_root_dir,
};

use std::env;

pub mod actions;
mod black_state;
pub mod components;
pub mod level;
pub mod systems;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("assets");
    let display_config = app_root.join("config/display_config.ron");
    let key_bindings_path = app_root.join("config/input.ron");

    let game_data = GameDataBuilder::default()
        .with_system_desc(
            GltfSceneLoaderSystemDesc::default(),
            "gltf_loader",
            &[], // This is important so that entity instantiation is performed in a single frame.
        )
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            InputBundle::<StringBindings>::new().with_bindings_from_file(&key_bindings_path)?,
        )?
        .with_bundle(UiBundle::<StringBindings>::new())?
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)?
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderUi::default())
                .with_plugin(RenderShaded3D::default()),
        )?
        .with(systems::ButtonPush, "button_push", &["input_system"])
        .with(systems::ButtonRender, "button_render", &[])
        .with(systems::BoxStateSystem, "box_state_system", &[])
        .with(systems::DisplayRenderSystem, "display_render_system", &[])
        .with(systems::BoxProgressSystem, "box_progress_system", &[])
        .with(
            systems::RenderProgressionSystem,
            "render_progression_system",
            &[],
        );

    let args: Vec<String> = env::args().collect();
    let level = if args.len() >= 2 {
        args[1].parse::<usize>()?
    } else {
        1
    };

    let mut game = Application::new(resources, black_state::BlackState::from(level), game_data)?;
    game.run();

    Ok(())
}
