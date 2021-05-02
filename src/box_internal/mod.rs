use bevy::prelude::*;
use ron::de::from_reader;
use serde::{Deserialize, Serialize};
use std::{convert::TryFrom, path::Path};

pub mod actions;
pub mod components;

use components::{BoxOut, ButtonScript};

pub struct OutputEvent {
    pub box_: Entity,
    pub output: BoxOut,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct ButtonData {
    pub button: ButtonScript,
    pub translation: Vec3,
}

#[derive(Serialize, Deserialize, Default, Clone, Debug)]
#[serde(deny_unknown_fields)]
pub struct LevelData {
    pub prompt: Vec<BoxOut>,
    pub buttons: Vec<ButtonData>,
}

impl TryFrom<&str> for LevelData {
    type Error = ron::error::Error;

    fn try_from(path: &str) -> ron::error::Result<LevelData> {
        let input_path = Path::new("assets/levels").join(path);
        let f = std::fs::File::open(&input_path)?;
        from_reader(f)
    }
}

pub mod systems {
    use crate::box_internal::{
        components::{BoxState, ButtonScript, Itemized, Pressable, Progression},
        OutputEvent,
    };
    use bevy::prelude::*;

    pub fn update(
        mut box_query: Query<(&mut BoxState, &mut Progression)>,
        button_query: Query<(&Pressable, &ButtonScript, &Itemized)>,
        mut event_writer: EventWriter<OutputEvent>,
    ) {
        for (pressable, button_script, itemized) in button_query.iter() {
            if pressable.just_unpressed() {
                let (mut box_, mut progression) = box_query
                    .get_mut(itemized.collector)
                    .expect("Itemized component on button isn't pointing to a Box!");
                for action in &button_script.0 {
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
