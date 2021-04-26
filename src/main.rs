use bevy::prelude::*;
use std::env;

pub mod actions;
mod black_state;
pub mod components;
pub mod level;
pub mod systems;

use black_state::AppState::BlackBox;

fn main() {
    //let app_root = application_root_dir()?;

    //let resources = app_root.join("assets");
    //let display_config = app_root.join("config/display_config.ron");
    //let key_bindings_path = app_root.join("config/input.ron");

    //let mut dispatcher = DispatcherBuilder::default();

    //dispatcher
    //.add_bundle(LoaderBundle)
    //.add_bundle(GltfBundle)
    //.add_bundle(TransformBundle)
    //.add_bundle(AnimationBundle::<i32, Transform>::default())
    //.add_bundle(InputBundle::new().with_bindings_from_file(&key_bindings_path)?)
    //.add_bundle(UiBundle::<String>::new())
    //.add_bundle(
    //RenderingBundle::<DefaultBackend>::new()
    //.with_plugin(
    //RenderToWindow::from_config_path(display_config)?.with_clear(ClearColor {
    //float32: [0.36, 0.36, 0.63, 1.0],
    //}),
    //)
    //.with_plugin(RenderUi::default())
    //.with_plugin(RenderPbr3D::default()),
    //)
    //.add_system(systems::push_button_system)
    //.add_system(systems::render_button_system)
    //.add_system(systems::update_box_state_system)
    //.add_system(systems::render_display_system)
    //.add_system(systems::update_box_progress_system)
    //.add_system(systems::render_progression_system);
    //

    App::build()
        .add_plugins(DefaultPlugins)
        .add_state(BlackBox)
        .add_event::<components::OutputEvent>()
        .add_system_set(
            SystemSet::on_enter(BlackBox).with_system(black_state::black_box_setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(BlackBox)
                .with_system(black_state::level_completion.system())
                .with_system(systems::push_button.system())
                .with_system(systems::render_display.system())
                //.with_system(systems::render_progression.system())
                .with_system(systems::update_box_progress.system())
                .with_system(systems::update_box_state.system()),
        )
        .add_system_set(
            SystemSet::on_exit(BlackBox).with_system(black_state::black_box_cleanup.system()),
        )
        .run();

    //let args: Vec<String> = env::args().collect();
    //let level = if args.len() >= 2 {
    //args[1].parse::<usize>()
    //} else {
    //1
    //};
    //Ok(())
}
