use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::box_internal::actions::{Action, BoxOut};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Pressable {
    #[serde(skip)]
    pressed: bool,
    #[serde(skip)]
    previous: bool,
}

impl Pressable {
    pub fn update(&mut self, pressed: bool) {
        self.previous = self.pressed;
        self.pressed = pressed;
    }

    pub fn update_necessary(&self, pressed: bool) -> bool {
        self.previous != self.pressed || self.pressed != pressed
    }

    pub fn pressed(&self) -> bool {
        self.pressed
    }

    pub fn just_pressed(&self) -> bool {
        self.pressed && !self.previous
    }

    pub fn just_unpressed(&self) -> bool {
        !self.pressed && self.previous
    }
}

pub type ActionScript = Vec<Action>;

pub type BoxState = [f32; 8];

#[derive(Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Progression {
    prompt: Vec<BoxOut>,
    #[serde(skip)]
    answer: Vec<BoxOut>,
}

impl Progression {
    pub fn new(prompt: Vec<BoxOut>) -> Progression {
        Progression {
            prompt,
            answer: Vec::new(),
        }
    }

    pub fn update(&mut self, output: BoxOut) {
        self.answer.push(output);

        while !self.answer.is_empty() && !self.prompt.starts_with(self.answer.as_slice()) {
            self.answer.remove(0);
        }
    }

    pub fn progress(&self) -> usize {
        self.answer.len()
    }

    pub fn total(&self) -> usize {
        self.prompt.len()
    }
}

/// Component that implies its entity is a member of an ordered list (at index), associated with
/// some other Entity (collector).
/// It is the reverse of the collector Entity having a Vec\<Entity\> component, containing this
/// Entity.
#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct Itemized {
    pub collector: Entity,
    pub index: usize,
}
