use bevy::prelude::*;
use std::{env, num::ParseIntError};

pub mod actions;
mod black_state;
pub mod components;
pub mod level;
pub mod systems;

use black_state::AppState;

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabels {
    Input,
}

fn main() -> Result<(), ParseIntError> {
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
    let args: Vec<String> = env::args().collect();
    let level_num = level::LevelNum(if args.len() >= 2 {
        args[1].parse::<usize>()?
    } else {
        1
    });

    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(level_num)
        .add_state(AppState::BlackBox)
        .add_event::<components::OutputEvent>()
        .add_startup_system(black_state::camera_setup.system())
        .add_system_set(
            SystemSet::on_enter(AppState::BlackBox)
                .with_system(black_state::black_box_setup.system())
                .with_system(level::add_colors.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::BlackBox)
                .label(SystemLabels::Input)
                .with_system(systems::push_button.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::BlackBox)
                .after(SystemLabels::Input)
                .with_system(systems::update_box_progress.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::BlackBox)
                .with_system(black_state::level_completion.system())
                .with_system(systems::render_display.system())
                .with_system(systems::render_progression.system())
                .with_system(systems::update_box_state.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::BlackBox)
                .with_system(black_state::black_box_cleanup.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::IntoBlackBox)
                .with_system(black_state::into_black_box.system()),
        )
        .run();

    Ok(())
}
