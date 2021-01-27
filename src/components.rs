use amethyst::{
    assets::PrefabData,
    core::shrev::{EventChannel, ReaderId},
    derive::PrefabData,
    ecs::{Component, DenseVecStorage, Entity, WriteStorage},
    Error,
};
use serde::{Deserialize, Serialize};
use std::fmt;

use crate::actions::Action;

#[derive(Default, Clone, Serialize, Deserialize, PrefabData, Debug)]
#[prefab(Component)]
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

impl Component for Button {
    type Storage = DenseVecStorage<Self>;
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

#[derive(Default, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BlackBox {
    pub state: BoxState,
    #[serde(skip)]
    pub buttons: Vec<Entity>,
    #[serde(skip)]
    pub output_channel: EventChannel<BoxOut>,
}

impl Clone for BlackBox {
    fn clone(&self) -> Self {
        BlackBox {
            state: self.state,
            buttons: self.buttons.clone(),
            output_channel: EventChannel::new(),
        }
    }
}

impl Component for BlackBox {
    type Storage = DenseVecStorage<Self>;
}

impl BlackBox {
    pub fn new(buttons: Vec<Entity>) -> BlackBox {
        BlackBox {
            state: BoxState::default(),
            buttons,
            output_channel: EventChannel::new(),
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct Progression {
    #[serde(skip)]
    pub prompt: Vec<Entity>,
    pub answer: Vec<BoxOut>,
}

impl Component for Progression {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Clone, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(deny_unknown_fields)]
pub struct ProgressionPiece(pub BoxOut);

impl Component for ProgressionPiece {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default, Serialize, Deserialize, PrefabData)]
#[prefab(Component)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BoxReader {
    #[serde(skip)]
    pub box_: Option<Entity>,
    #[serde(skip)]
    pub reader_id: Option<ReaderId<BoxOut>>,
}

impl Clone for BoxReader {
    fn clone(&self) -> Self {
        BoxReader {
            box_: self.box_,
            reader_id: None,
        }
    }
}

impl Component for BoxReader {
    type Storage = DenseVecStorage<Self>;
}

impl BoxReader {
    pub fn new(box_: Entity, reader_id: ReaderId<BoxOut>) -> BoxReader {
        BoxReader {
            box_: Some(box_),
            reader_id: Some(reader_id),
        }
    }
}
