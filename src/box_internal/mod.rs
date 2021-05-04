pub mod actions;
pub mod components;

use bevy::prelude::*;
use components::{ActionScript, BoxOut};
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::Path};

#[derive(Clone, PartialEq, Debug)]
pub struct OutputEvent {
    pub box_: Entity,
    pub output: BoxOut,
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
        components::{ActionScript, BoxState, Itemized, Pressable, Progression},
        OutputEvent,
    };
    use bevy::prelude::*;

    pub fn update(
        mut box_query: Query<(&mut BoxState, &mut Progression)>,
        button_query: Query<(&Pressable, &ActionScript, &Itemized), Changed<Pressable>>,
        mut event_writer: EventWriter<OutputEvent>,
    ) {
        for (pressable, action_script, itemized) in button_query.iter() {
            if pressable.just_unpressed() {
                let (mut box_, mut progression) = box_query
                    .get_mut(itemized.collector)
                    .expect("Itemized component on button isn't pointing to a Box!");
                for action in action_script {
                    let out = action.evaluate(&mut box_);
                    if let Some(o) = out {
                        progression.update(o.clone());
                        event_writer.send(OutputEvent {
                            box_: itemized.collector,
                            output: o.clone(),
                        });
                    }
                }
            }
        }
    }
}
