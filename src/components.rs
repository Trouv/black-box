use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::input::VirtualKeyCode;

#[derive(Default)]
pub struct Button {
    pub pressed: bool,
    pub last_pressed: bool,
    pub just_pressed: bool,
    pub just_unpressed: bool,
    pub num: usize,
}

impl Component for Button {
    type Storage = DenseVecStorage<Self>;
}

pub const BUTTON_NUMS: [&str; 6] = [
    "button_0", "button_1", "button_2", "button_3", "button_4", "button_5",
];

impl Button {
    pub fn new(num: usize) -> Self {
        let mut button = Button::default();
        button.num = num;
        button
    }
}
