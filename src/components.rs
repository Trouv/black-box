use amethyst::{
    core::shrev::{EventChannel, ReaderId},
    ecs::{Component, DenseVecStorage, Entity, NullStorage, World},
    ui::{Anchor, UiImage, UiText, UiTransform},
};
use std::{collections::VecDeque, fmt};

#[derive(Default)]
pub struct Button {
    pub pressed: bool,
    pub just_pressed: bool,
    pub just_unpressed: bool,
    pub action: Option<ButtonAction>,
}

impl Component for Button {
    type Storage = DenseVecStorage<Self>;
}

pub const BUTTON_NUMS: [&str; 6] = [
    "button_0", "button_1", "button_2", "button_3", "button_4", "button_5",
];

#[derive(Debug, PartialEq, Clone)]
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

impl Button {
    pub fn new(action: Option<ButtonAction>) -> Self {
        let mut button = Button::default();
        button.action = action;
        button
    }
}

#[derive(Default)]
pub struct BlackBox {
    pub state: BoxState,
    pub buttons: Vec<Entity>,
    pub display: Option<Entity>,
    pub output_channel: EventChannel<BoxOut>,
}

impl Component for BlackBox {
    type Storage = DenseVecStorage<Self>;
}

impl BlackBox {
    pub fn new(buttons: Vec<Entity>, display: Option<Entity>) -> BlackBox {
        BlackBox {
            state: BoxState::default(),
            buttons,
            display,
            output_channel: EventChannel::new(),
        }
    }
}

#[derive(Default)]
pub struct Progression {
    pub prompt: Vec<Entity>,
    pub box_: Option<Entity>,
    pub reader_id: Option<ReaderId<BoxOut>>,
    pub answer: Vec<BoxOut>,
}

impl Component for Progression {
    type Storage = DenseVecStorage<Self>;
}

impl Progression {
    pub fn new(box_: Entity, reader_id: ReaderId<BoxOut>) -> Progression {
        Progression {
            prompt: Vec::new(),
            box_: Some(box_),
            reader_id: Some(reader_id),
            answer: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct ProgressionPiece(pub BoxOut);

impl Component for ProgressionPiece {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct BoxDisplay;

impl Component for BoxDisplay {
    type Storage = NullStorage<Self>;
}
