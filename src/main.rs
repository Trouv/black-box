//! # Black Box
//! Black Box is, so far, a very simple puzzle game written with Rust + Bevy.
//!
//! This project uses bevy's new State Machine logic, and its modules are structured around these
//! states.
//! Then, they are further split into components, systems, transitions (for entering and
//! exiting systems) and resources, as necessary.
//! There are also such sub-modules on the top-level, for objects and systems that aren't
//! associated with a particular state.
//! See the sub-module documentation for more details.
pub mod box_internal;
pub mod roaming;
pub mod standard_box;
pub mod transitions;

use bevy::prelude::*;
use heron::prelude::*;
use std::{env, num::ParseIntError};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    StandardBox,
    StandardBoxTransition,
    Roaming,
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
    // Left as an enum for potential future labels
    InputLabel,
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
        .add_plugin(PhysicsPlugin::default())
        .insert_resource(Msaa { samples: 1 })
        .insert_resource(level_num)
        .add_state(AppState::StandardBox)
        .add_event::<box_internal::OutputEvent>()
        .add_startup_system(transitions::add_colors.system())
        .add_plugin(standard_box::StandardBoxPlugin)
        .add_plugin(roaming::RoamingPlugin)
        .run();

    Ok(())
}

pub mod resources {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct LevelNum(pub usize);

    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    pub struct ColorHandles {
        pub white: Handle<ColorMaterial>,
        pub green: Handle<ColorMaterial>,
    }
}
