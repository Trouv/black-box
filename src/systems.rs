use crate::{
    components::{BlackBox, BoxOut, BoxReader, Button, Progression, ProgressionPiece, BUTTON_NUMS},
    level::GREEN,
};
use bevy::prelude::*;

#[system]
#[read_component(Entity)]
#[read_component(Button)]
#[read_component(AnimationSet<usize, Transform>)]
#[write_component(AnimationControlSet<usize, Transform>)]
pub fn render_button(world: &mut SubWorld, buffer: &mut CommandBuffer) {
    let mut query = <(Entity, Read<Button>, Read<AnimationSet<usize, Transform>>)>::query();
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

pub fn push_button(box_query: Query<&BlackBox>, button_query: Query<&mut Button>, input: Res<Input>) {
    for box_ in box_query.iter() {
        for (i, b) in box_.buttons.iter().enumerate() {
            let mut button = button_query.get_component::<Button>(*b).unwrap();
            let last_pressed = button.pressed;
            button.pressed = input.action_is_down(BUTTON_NUMS[i]).unwrap();
            button.just_pressed = button.pressed && !last_pressed;
            button.just_unpressed = !button.pressed && last_pressed;
        }
    }
}

pub fn update_box_state(box_query: Query<&mut BlackBox>, button_query: Query<&Button>)>) {
    for mut box_ in box_query.iter_mut() {
        let mut state = box_.state;
        for b in &box_.buttons {
            let button = button_query
                .get_component::<Button>(*b)
                .unwrap();
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

pub fn render_display(reader_query: Query<(&mut BoxReader, &mut UiText)>, box_query: Query<&BlackBox>, time: Res<Time>) {
    for (reader, mut text) in reader_query.iter_mut() {
        if let Some(out) = box_query
            .get_component::<BlackBox>(reader.box_.unwrap())
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

pub fn update_box_progress(reader_query: Query<(&mut Progression, &mut BoxReader)>, box_query: Query<&BlackBox>, piece_query: Query<&ProgressionPiece>) {
    for (progress, reader) in reader_query.iter_mut() {
        let box_ = box_query.get_component_mut::<BlackBox>(reader.box_.unwrap()).unwrap();

        for out in box_.output_channel.read(reader.reader_id.as_mut().unwrap()) {
            progress.answer.push(out.clone());

            while !progress.answer.is_empty()
                && !progress
                    .prompt
                    .iter()
                    .map(|p| {
                        piece_query
                            .get_component::<ProgressionPiece>(*p)
                            .unwrap()
                            .0
                            .clone()
                    })
                    .collect::<Vec<BoxOut>>()
                    .starts_with(progress.answer.as_slice())
            {
                progress.answer.remove(0);
            }
        }
    }
}

pub fn render_progression(prog_query: Query<&Progression>, image_query: Query<&mut UiImage>) {
    for progress in prog_query.iter() {
        for (i, piece) in progress.prompt.iter().enumerate() {
            let color = if i < progress.answer.len() {
                GREEN
            } else {
                [0.9, 0.9, 0.9, 1.0]
            };
            image_query
                .get_component_mut::<UiImage>(*piece)
                .unwrap() = UiImage::SolidColor(color);
        }
    }
}
