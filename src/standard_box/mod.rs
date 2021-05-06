//! Systems, components, helpers and a plugin for the StandardBox state.
//!
//! The StandardBox state allows users to interact with a black box and see progress towards the
//! prompt.
//! This does not contain any of the gameplay logic for the boxes, just the input and rendering for
//! a standard box.
pub mod systems;
pub mod transitions;

use crate::{box_internal, AppState, SystemLabels};
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub const BUTTON_NUMS: [KeyCode; 6] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
];

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct StandardBoxPlugin;

impl Plugin for StandardBoxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::StandardBox)
                .with_system(transitions::black_box_setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::StandardBox)
                .label(SystemLabels::InputLabel)
                .with_system(systems::button_input.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::StandardBox)
                .after(SystemLabels::InputLabel)
                .with_system(box_internal::systems::update.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::StandardBox)
                .with_system(transitions::level_completion.system())
                .with_system(systems::render_button.system())
                .with_system(systems::render_display.system())
                .with_system(systems::render_progression.system()),
        )
        .add_system_set(
            SystemSet::on_exit(AppState::StandardBox)
                .with_system(transitions::black_box_cleanup.system()),
        )
        .add_system_set(
            SystemSet::on_enter(AppState::StandardBoxTransition)
                .with_system(transitions::into_black_box.system()),
        );
    }
}

pub mod components {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, Serialize, Deserialize)]
    pub struct ProgressionPiece;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct BoxReader {
        pub box_: Entity,
    }

    impl BoxReader {
        pub fn new(box_: Entity) -> BoxReader {
            BoxReader { box_ }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct BoxUiRoot(pub Entity);
}
