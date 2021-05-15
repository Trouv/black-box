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
use bevy_mod_picking::PickingPlugin;
use heron::prelude::*;
use std::num::ParseIntError;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub enum AppState {
    StandardBox,
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
    App::build()
        .add_plugins(DefaultPlugins)
        .add_plugin(PhysicsPlugin::default())
        .add_plugin(PickingPlugin)
        .insert_resource(Msaa { samples: 1 })
        .add_state(AppState::Roaming)
        .add_event::<box_internal::OutputEvent>()
        .add_startup_system(transitions::add_colors.system())
        .add_plugin(standard_box::StandardBoxPlugin)
        .add_plugin(roaming::RoamingPlugin)
        .run();

    Ok(())
}

pub mod resources {
    use bevy::prelude::*;

    #[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
    pub struct ColorHandles {
        pub white: Handle<ColorMaterial>,
        pub green: Handle<ColorMaterial>,
    }
}
