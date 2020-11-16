use amethyst::ecs::{Component, DenseVecStorage, Entity, NullStorage};
use std::collections::VecDeque;

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
    pub output_queue: VecDeque<BoxOut>,
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
            output_queue: VecDeque::new(),
        }
    }
}

#[derive(Default)]
pub struct Progression {
    pub prompt: Vec<BoxOut>,
    pub box_: Option<Entity>,
    pub answer: Vec<BoxOut>,
}

impl Component for Progression {
    type Storage = DenseVecStorage<Self>;
}

impl Progression {
    pub fn new(prompt: Vec<BoxOut>, box_: Entity) -> Progression {
        Progression {
            prompt,
            box_: Some(box_),
            answer: Vec::new(),
        }
    }
}

#[derive(Default)]
pub struct ProgressionPiece(BoxOut);

impl Component for ProgressionPiece {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct BoxDisplay;

impl Component for BoxDisplay {
    type Storage = NullStorage<Self>;
}
