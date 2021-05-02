use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

use super::actions::Action;

#[derive(Clone, Default, Serialize, Deserialize, Debug)]
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

pub const BUTTON_NUMS: [KeyCode; 6] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
];

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub enum BoxOut {
    Int(i32),
    Flt(f32),
    Str(String),
}

impl fmt::Display for BoxOut {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BoxOut::Int(o) => write!(f, "{}", o),
            BoxOut::Flt(o) => write!(f, "{}", o),
            BoxOut::Str(o) => write!(f, "{}", o),
        }
    }
}

impl Default for BoxOut {
    fn default() -> Self {
        BoxOut::Int(0)
    }
}

pub type BoxState = [f32; 8];

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Progression {
    pub prompt: Vec<BoxOut>,
    #[serde(skip)]
    pub answer: Vec<BoxOut>,
}

impl Progression {
    pub fn update(&mut self, output: BoxOut) {
        self.answer.push(output);

        while !self.answer.is_empty() && !self.prompt.starts_with(self.answer.as_slice()) {
            self.answer.remove(0);
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Itemized {
    pub collector: Entity,
    pub index: usize,
}
