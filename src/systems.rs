use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::{timing::Time, transform::Transform},
    ecs::{Read, System},
    input::InputHandler,
    ui::{UiImage, UiText},
};

use crate::{
    components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece, BUTTON_NUMS},
    level::GREEN,
};

#[derive(SystemDesc)]
pub struct ButtonRender;

impl<'a> System<'a> for ButtonRender {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Button>,
        ReadStorage<'a, AnimationSet<usize, Transform>>,
        WriteStorage<'a, AnimationControlSet<usize, Transform>>,
    );

    fn run(&mut self, (entities, buttons, sets, mut controls): Self::SystemData) {
        for (entity, set, button) in (&entities, &sets, &buttons).join() {
            if let Some(control_set) = get_animation_set(&mut controls, entity) {
                if button.just_pressed {
                    control_set.add_animation(
                        0,
                        set.get(&0).unwrap(),
                        EndControl::Stay,
                        4.0,
                        AnimationCommand::Start,
                    );
                } else if button.just_unpressed {
                    control_set.add_animation(
                        1,
                        set.get(&1).unwrap(),
                        EndControl::Stay,
                        4.0,
                        AnimationCommand::Start,
                    );
                }
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
                    log::debug!("Action performed, current state: {:?}", box_.state);
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
                    GREEN
                } else {
                    [0.9, 0.9, 0.9, 1.0]
                };
                *images.get_mut(*piece).unwrap() = UiImage::SolidColor(color);
            }
        }
    }
}
