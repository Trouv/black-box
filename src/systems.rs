use amethyst::ecs::{Join, System, SystemData};
use amethyst::ecs::{Read, ReadStorage, WriteStorage};
use amethyst::input::{InputHandler, StringBindings};
use amethyst::renderer::SpriteRender;
use amethyst::SystemDesc;

use crate::components;

#[derive(SystemDesc)]
pub struct ButtonRender;

impl<'a> System<'a> for ButtonRender {
    type SystemData = (
        WriteStorage<'a, SpriteRender>,
        ReadStorage<'a, components::Button>,
    );

    fn run(&mut self, (mut sprites, buttons): Self::SystemData) {
        for (sprite, button) in (&mut sprites, &buttons).join() {
            if button.just_pressed {
                sprite.sprite_number = 1;
            } else if button.just_unpressed {
                sprite.sprite_number = 0;
            }
        }
    }
}

pub struct ButtonPush;

impl<'a> System<'a> for ButtonPush {
    type SystemData = (
        WriteStorage<'a, components::Button>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut buttons, input): Self::SystemData) {
        for (mut button,) in (&mut buttons,).join() {
            let last_pressed = button.last_pressed;
            button.last_pressed = button.pressed;
            button.pressed = input
                .action_is_down(components::BUTTON_NUMS[button.num])
                .unwrap();
            button.just_pressed = button.pressed && !button.last_pressed;
            button.just_unpressed = !button.pressed && button.last_pressed;
        }
    }
}

pub struct BoxStateSystem;

impl<'a> System<'a> for BoxStateSystem {
    type SystemData = (
        WriteStorage<'a, components::BlackBox>,
        ReadStorage<'a, components::Button>,
    );

    fn run(&mut self, (mut boxes, buttons): Self::SystemData) {
        for (mut box_,) in (&mut boxes,).join() {
            let mut state = box_.state;
            for b in &box_.buttons {
                let button = buttons.get(*b).unwrap();
                if button.just_pressed {
                    state = button.action.unwrap()(state);
                }
            }
            box_.state = state;
        }
    }
}
