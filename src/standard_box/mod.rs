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

pub enum StandardBoxEvent {
    Enter(Entity),
    Exit(Entity),
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct StandardBoxPlugin;

impl Plugin for StandardBoxPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_plugin(box_internal::BoxInternalPlugin)
            .add_system_set(
                SystemSet::on_enter(AppState::StandardBox)
                    .with_system(transitions::spawn_box_ui.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::StandardBox)
                    .label(SystemLabels::InputLabel)
                    .with_system(systems::button_input.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::StandardBox)
                    .with_system(transitions::exit_on_level_completion.system())
                    .with_system(transitions::exit_on_walk_away.system())
                    .with_system(transitions::pop_out_on_exit.system())
                    .with_system(systems::render_button.system())
                    .with_system(systems::render_display.system())
                    .with_system(systems::render_progression.system()),
            )
            .add_system_set(
                SystemSet::on_exit(AppState::StandardBox)
                    .with_system(transitions::despawn_box_ui.system())
                    .with_system(transitions::deactivate_box.system()),
            );
    }
}

pub mod components {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, Serialize, Deserialize)]
    pub struct ProgressionPiece;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct BoxUiRoot;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct BoxOutDisplay;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct Active;

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct BoxReference {
        pub box_: Entity,
    }

    impl BoxReference {
        pub fn new(box_: Entity) -> BoxReference {
            BoxReference { box_ }
        }
    }

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct ButtonRayCastSet;
}
