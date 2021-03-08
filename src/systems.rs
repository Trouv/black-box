use amethyst::{
    core::timing::Time,
    ecs::{Join, Read, ReadStorage, System, SystemData, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::SpriteRender,
    ui::{UiImage, UiText},
    SystemDesc,
};

use crate::components::{
    BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece, BUTTON_NUMS,
};

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
                if button.just_unpressed {
                    for action in &button.action {
                        let out = action.evaluate(&mut state);
                        if let Some(o) = out {
                            box_.output_channel.single_write(o.clone());
                        }
                    }
                    //println!("{:?}", state);
                }
            }
            box_.state = state;
        }
    }
}

pub struct DisplayRenderSystem;

impl<'a> System<'a> for DisplayRenderSystem {
    type SystemData = (
        WriteStorage<'a, UiText>,
        WriteStorage<'a, BoxReader>,
        ReadStorage<'a, BlackBox>,
        Read<'a, Time>,
    );

    fn run(&mut self, (mut texts, mut readers, boxes, time): Self::SystemData) {
        for (reader, mut text) in (&mut readers, &mut texts).join() {
            if let Some(out) = boxes
                .get(reader.box_.unwrap())
                .unwrap()
                .output_channel
                .read(reader.reader_id.as_mut().unwrap())
                .last()
            {
                text.text = out.to_string();
                text.color[3] = 1.;
            } else {
                text.color[3] = (text.color[3] - (2. * time.delta_seconds())).max(0.4);
            }
        }
    }
}

pub struct BoxProgressSystem;

impl<'a> System<'a> for BoxProgressSystem {
    type SystemData = (
        WriteStorage<'a, Progression>,
        WriteStorage<'a, BoxReader>,
        WriteStorage<'a, BlackBox>,
        ReadStorage<'a, ProgressionPiece>,
    );

    fn run(&mut self, (mut progresses, mut readers, mut boxes, pieces): Self::SystemData) {
        for (progress, reader) in (&mut progresses, &mut readers).join() {
            let box_ = boxes.get_mut(reader.box_.unwrap()).unwrap();

            for out in box_.output_channel.read(reader.reader_id.as_mut().unwrap()) {
                progress.answer.push(out.clone());

                while !progress.answer.is_empty()
                    && !progress
                        .prompt
                        .iter()
                        .map(|p| pieces.get(*p).unwrap().0.clone())
                        .collect::<Vec<BoxOut>>()
                        .starts_with(progress.answer.as_slice())
                {
                    progress.answer.remove(0);
                }
            }
        }
    }
}

pub struct RenderProgressionSystem;

impl<'a> System<'a> for RenderProgressionSystem {
    type SystemData = (WriteStorage<'a, UiImage>, ReadStorage<'a, Progression>);

    fn run(&mut self, (mut images, progresses): Self::SystemData) {
        for (progress,) in (&progresses,).join() {
            for (i, piece) in progress.prompt.iter().enumerate() {
                let color = if i < progress.answer.len() {
                    [0.5, 1.0, 0.5, 1.0]
                } else {
                    [0.9, 0.9, 0.9, 1.0]
                };
                *images.get_mut(*piece).unwrap() = UiImage::SolidColor(color);
            }
        }
    }
}
