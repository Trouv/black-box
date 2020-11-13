use amethyst::ecs::{Component, DenseVecStorage};
use amethyst::input::VirtualKeyCode;

pub struct Button {
    pub pressed: bool,
    pub num: usize,
}

impl Component for Button {
    type Storage = DenseVecStorage<Self>;
}

pub const BUTTON_NUMS: [&str; 6] = [
    "button_0", "button_1", "button_2", "button_3", "button_4", "button_5",
];

impl Default for Button {
    fn default() -> Self {
        Button {
            pressed: false,
            num: 0,
        }
    }
}
