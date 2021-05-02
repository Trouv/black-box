use bevy::prelude::*;
use std::{env, num::ParseIntError};

pub mod box_internal;
pub mod resources;
pub mod standard_box;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    StandardBox,
    StandardBoxTransition,
}

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, SystemLabel)]
enum SystemLabels {
    Input,
}

fn main() -> Result<(), ParseIntError> {
    let args: Vec<String> = env::args().collect();
    let level_num = resources::LevelNum(if args.len() >= 2 {
        args[1].parse::<usize>()?
    } else {
        1
    });

    App::build()
        .add_plugins(DefaultPlugins)
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(level_num)
        .add_state(AppState::StandardBox)
        .add_event::<box_internal::OutputEvent>()
        .add_startup_system(transitions::camera_setup.system())
        .add_system_set(
            SystemSet::on_enter(AppState::StandardBox)
                .with_system(standard_box::transitions::black_box_setup.system())
                .with_system(resources::add_colors.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::StandardBox)
                .label(SystemLabels::Input)
                .with_system(standard_box::systems::button_input.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::StandardBox)
                .after(SystemLabels::Input)
                .with_system(box_internal::systems::update.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::StandardBox)
                .with_system(standard_box::transitions::level_completion.system())
                .with_system(standard_box::systems::render_button.system())
                .with_system(standard_box::systems::render_display.system())
                .with_system(standard_box::systems::render_progression.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::StandardBox)
                .with_system(standard_box::transitions::black_box_cleanup.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::StandardBoxTransition)
                .with_system(standard_box::transitions::into_black_box.system()),
        )
        .run();

    Ok(())
}

mod transitions {
    use bevy::prelude::*;

    pub fn camera_setup(mut commands: Commands) {
        commands.spawn_bundle(UiCameraBundle::default());
        commands.spawn_bundle(PerspectiveCameraBundle {
            transform: Transform::from_xyz(0., 1.1, 0.8)
                .looking_at(Vec3::new(0., 0., -0.2), Vec3::Y),
            ..Default::default()
        });
        commands.spawn_bundle(LightBundle {
            transform: Transform::from_xyz(-2., 2., 2.),
            ..Default::default()
        });
    }
}
