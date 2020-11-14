use amethyst::{
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
    SystemDesc,
};

use crate::components::{BlackBox, BoxProgress, Button, BUTTON_NUMS};

#[derive(SystemDesc)]
pub struct ButtonRender;

impl<'a> System<'a> for ButtonRender {
    type SystemData = (WriteStorage<'a, SpriteRender>, ReadStorage<'a, Button>);

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
        WriteStorage<'a, Button>,
        ReadStorage<'a, BlackBox>,
        Read<'a, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut buttons, boxes, input): Self::SystemData) {
        for (box_,) in (&boxes,).join() {
            for (i, b) in box_.buttons.iter().enumerate() {
                let button = buttons.get_mut(*b).unwrap();
                let last_pressed = button.pressed;
                button.pressed = input.action_is_down(BUTTON_NUMS[i]).unwrap();
                button.just_pressed = button.pressed && !last_pressed;
                button.just_unpressed = !button.pressed && last_pressed;
            }
        }
    }
}

pub struct BoxStateSystem;

impl<'a> System<'a> for BoxStateSystem {
    type SystemData = (WriteStorage<'a, BlackBox>, ReadStorage<'a, Button>);

    fn run(&mut self, (mut boxes, buttons): Self::SystemData) {
        for (mut box_,) in (&mut boxes,).join() {
            let mut state = box_.state;
            for b in &box_.buttons {
                let button = buttons.get(*b).unwrap();
                if button.just_pressed {
                    let (new_state, out) = button.action.unwrap()(state);
                    state = new_state;
                    if let Some(o) = out {
                        box_.output_queue.push_back(o);
                    }
                }
            }
            box_.state = state;
        }
    }
}

pub struct BoxProgressSystem;

impl<'a> System<'a> for BoxProgressSystem {
    type SystemData = (WriteStorage<'a, BoxProgress>, WriteStorage<'a, BlackBox>);

    fn run(&mut self, (mut progresses, mut boxes): Self::SystemData) {
        for (mut progress,) in (&mut progresses,).join() {
            let box_ = boxes.get_mut(progress.box_.unwrap()).unwrap();
            while let Some(out) = box_.output_queue.pop_front() {
                if out == progress.prompt[progress.index] {
                    progress.index += 1;
                } else {
                    progress.index = 0;
                }
                println!("{:?}", progress.prompt);
                println!("{}", progress.index);
            }
        }
    }
}
