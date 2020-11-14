use amethyst::ecs::{Component, DenseVecStorage, Entity};

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

pub type BoxState = [f32; 8];
pub type ButtonAction = fn(BoxState) -> BoxState;

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
}

impl Component for BlackBox {
    type Storage = DenseVecStorage<Self>;
}

impl BlackBox {
    pub fn new(buttons: Vec<Entity>) -> BlackBox {
        BlackBox {
            state: BoxState::default(),
            buttons,
        }
    }
}
