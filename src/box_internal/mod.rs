//! Systems, components, and helpers for the gameplay logic of a BlackBox.
//!
//! This module is only concerned with the internal logic of a Black Box, not its rendering or
//! input.
//! As a result, there is no state associated with this module, and no plugin.
//! Instead, other states may implement this module's systems and components as needed.
pub mod actions;
pub mod components;
pub mod systems;

use crate::SystemLabels;
use actions::BoxOut;
use bevy::prelude::*;
use components::ActionScript;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::Path};

#[derive(Clone, PartialEq, Debug)]
pub struct OutputEvent {
    pub box_: Entity,
    pub output: BoxOut,
}

pub struct BoxCompletedEvent {
    pub box_: Entity,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ButtonData {
    pub button: ActionScript,
    pub translation: Vec3,
}

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BoxData {
    pub prompt: Vec<BoxOut>,
    pub buttons: Vec<ButtonData>,
}

impl TryFrom<&str> for BoxData {
    type Error = ron::error::Error;

    fn try_from(path: &str) -> ron::error::Result<BoxData> {
        let input_path = Path::new("assets/levels").join(path);
        let f = std::fs::File::open(&input_path)?;
        from_reader(f)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BoxInternalPlugin;

impl Plugin for BoxInternalPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system(
            systems::button_output
                .system()
                .after(SystemLabels::InputLabel),
        )
        .add_system(
            systems::pipe_pass_in
                .system()
                .chain(systems::pipe_pass_out.system()),
        )
        .add_system(systems::progression.system());
    }
}
