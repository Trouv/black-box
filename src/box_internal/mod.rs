//! Systems, components, and helpers for the gameplay logic of a BlackBox.
//!
//! This module is only concerned with the internal logic of a Black Box, not its rendering or
//! input.
//! As a result, there is no state associated with this module, and no plugin.
//! Instead, other states may implement this module's systems and components as needed.
pub mod actions;
pub mod components;

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

pub mod systems {
    use crate::box_internal::{
        components::{ActionScript, BoxState, Itemized, PipeIn, PipePass, Pressable, Progression},
        BoxCompletedEvent, OutputEvent,
    };
    use bevy::prelude::*;

    pub fn button_output(
        mut box_query: Query<&mut BoxState>,
        button_query: Query<(&Pressable, &ActionScript, &Itemized), Changed<Pressable>>,
        mut output_writer: EventWriter<OutputEvent>,
    ) {
        for (pressable, action_script, itemized) in button_query.iter() {
            if pressable.just_unpressed() {
                let mut box_ = box_query
                    .get_mut(itemized.collector)
                    .expect("Itemized component on button isn't pointing to a Box!");
                for action in action_script {
                    let out = action.evaluate(&mut box_);
                    if let Some(o) = out {
                        output_writer.send(OutputEvent {
                            box_: itemized.collector,
                            output: o.clone(),
                        });
                    }
                }
            }
        }
    }

    pub fn progression(
        mut prog_query: Query<(Entity, &mut Progression, &PipeIn)>,
        mut output_reader: EventReader<OutputEvent>,
        mut completed_writer: EventWriter<BoxCompletedEvent>,
    ) {
        for output in output_reader.iter() {
            for (entity, mut progression, pipe_in) in prog_query.iter_mut() {
                if let Some(e) = pipe_in.out_entity {
                    if e == output.box_ {
                        progression.update(output.output.clone());
                    }
                    if progression.progress() >= progression.total() {
                        completed_writer.send(BoxCompletedEvent { box_: entity });
                    }
                }
            }
        }
    }

    pub fn pipe_pass(
        pass_query: Query<(Entity, &PipeIn), With<PipePass>>,
        mut output_reader: EventReader<OutputEvent>,
        mut output_writer: EventWriter<OutputEvent>,
    ) {
        for output in output_reader.iter() {
            for (entity, pipe_in) in pass_query.iter() {
                if let Some(e) = pipe_in.out_entity {
                    if e == output.box_ {
                        output_writer.send(OutputEvent {
                            box_: entity,
                            ..output.clone()
                        });
                    }
                }
            }
        }
    }
}
