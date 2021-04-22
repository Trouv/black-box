use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::actions::Action;

#[derive(Default, Clone, Serialize, Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Button {
    #[serde(skip)]
    pub pressed: bool,
    #[serde(skip)]
    pub just_pressed: bool,
    #[serde(skip)]
    pub just_unpressed: bool,
    pub action: Vec<Action>,
}

pub const BUTTON_NUMS: [&str; 6] = [
    "button_0", "button_1", "button_2", "button_3", "button_4", "button_5",
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
pub type BoxResult = (BoxState, Option<BoxOut>);
pub type ButtonAction = fn(BoxState) -> BoxResult;

impl From<Vec<Action>> for Button {
    fn from(action: Vec<Action>) -> Self {
        let mut button = Button::default();
        button.action = action;
        button
    }
}

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BlackBox {
    pub state: BoxState,
    #[serde(skip)]
    pub buttons: Vec<Entity>,
}

impl Clone for BlackBox {
    fn clone(&self) -> Self {
        BlackBox {
            state: self.state,
            buttons: self.buttons.clone(),
        }
    }
}

impl BlackBox {
    pub fn new(buttons: Vec<Entity>) -> BlackBox {
        BlackBox {
            state: BoxState::default(),
            buttons,
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Progression {
    #[serde(skip)]
    pub prompt: Vec<Entity>,
    pub answer: Vec<BoxOut>,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProgressionPiece(pub BoxOut);

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BoxReader {
    #[serde(skip)]
    pub box_: Option<Entity>,
}

impl Clone for BoxReader {
    fn clone(&self) -> Self {
        BoxReader { box_: self.box_ }
    }
}

impl BoxReader {
    pub fn new(box_: Entity) -> BoxReader {
        BoxReader { box_: Some(box_) }
    }
}

pub struct OutputEvent {
    pub box_: Entity,
    pub output: BoxOut,
}
