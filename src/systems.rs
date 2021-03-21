use amethyst::{
    animation::{
        get_animation_set, AnimationCommand, AnimationControlSet, AnimationSet, EndControl,
    },
    core::{timing::Time, transform::Transform},
    ecs::*,
    input::InputHandler,
    ui::{UiImage, UiText},
};

use crate::{
    components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece, BUTTON_NUMS},
    level::GREEN,
};

#[system]
#[read_component(Entity)]
#[read_component(Button)]
#[read_component(AnimationSet<usize, Transform>)]
#[write_component(AnimationControlSet<usize, Transform>)]
fn render_button(world: &mut SubWorld, buffer: &mut CommandBuffer) {
    let query = <(Entity, Read<Button>, Read<AnimationSet<usize, Transform>>)>::query();
    let (query_world, mut sub_world) = world.split_for_query(&query);
    for (entity, button, set) in query.iter(&query_world) {
        if let Some(control_set) = get_animation_set(&mut sub_world, buffer, *entity) {
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

#[system]
#[read_component(Entity)]
#[write_component(Button)]
#[read_component(BlackBox)]
fn push_button(world: &mut SubWorld, #[resource] input: &InputHandler) {
    let query = <(Entity, Read<BlackBox>)>::query();
    let (query_world, mut sub_world) = world.split_for_query(&query);
    for (entity, box_) in query.iter(&query_world) {
        for (i, b) in box_.buttons.iter().enumerate() {
            let entry = sub_world.entry_mut(*b).unwrap();
            let button = entry.get_component_mut::<Button>().unwrap();
            let last_pressed = button.pressed;
            button.pressed = input.action_is_down(BUTTON_NUMS[i]).unwrap();
            button.just_pressed = button.pressed && !last_pressed;
            button.just_unpressed = !button.pressed && last_pressed;
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

#[system]
#[read_component(Entity)]
#[write_component(BlackBox)]
#[read_component(Button)]
fn update_box_state(world: &mut SubWorld) {
    let query = <(Entity, Write<BlackBox>)>::query();
    let (query_world, sub_world) = world.split_for_query(&query);
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
