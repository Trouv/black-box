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
                .with_system(systems::button_input.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::BlackBox)
                .after(SystemLabels::Input)
                .with_system(systems::update_box_state.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::BlackBox)
                .with_system(black_state::level_completion.system())
                .with_system(systems::render_button.system())
                .with_system(systems::render_display.system())
                .with_system(systems::render_progression.system()),
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
