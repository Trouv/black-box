use crate::{
    components::{
        BlackBox, BoxReader, Button, OutputEvent, Progression, ProgressionPiece, BUTTON_NUMS,
    },
    level::ColorHandles,
};
use bevy::prelude::*;

pub fn render_button(mut button_query: Query<(&Button, &mut Transform)>) {
    for (button, mut transform) in button_query.iter_mut() {
        if button.pressed {
            transform.translation = Vec3::new(0., -0.02, 0.);
        } else {
            transform.translation = Vec3::ZERO;
        }
    }
}

pub fn button_input(
    box_query: Query<&BlackBox>,
    mut button_query: Query<&mut Button>,
    input: Res<Input<KeyCode>>,
) {
    for box_ in box_query.iter() {
        for (i, b) in box_.buttons.iter().enumerate() {
            let mut button = button_query.get_component_mut::<Button>(*b).unwrap();
            let last_pressed = button.pressed;
            button.pressed = input.pressed(BUTTON_NUMS[i]);
            button.just_pressed = button.pressed && !last_pressed;
            button.just_unpressed = !button.pressed && last_pressed;
        }
    }
}

pub fn update_box_state(
    mut box_query: Query<(Entity, &mut BlackBox, &mut Progression)>,
    button_query: Query<&Button>,
    mut event_writer: EventWriter<OutputEvent>,
) {
    for (entity, mut box_, mut progression) in box_query.iter_mut() {
        let mut state = box_.state;
        for b in &box_.buttons {
            let button = button_query.get_component::<Button>(*b).unwrap();
            if button.just_unpressed {
                for action in &button.action {
                    let out = action.evaluate(&mut state);
                    if let Some(o) = out {
                        progression.update(o.clone());
                        event_writer.send(OutputEvent {
                            box_: entity,
                            output: o.clone(),
                        });
                    }
                }
                log::debug!("Action performed, current state: {:?}", box_.state);
            }
        }
        box_.state = state;
    }
}

pub fn render_display(
    mut reader_query: Query<(&mut BoxReader, &mut Text)>,
    mut event_reader: EventReader<OutputEvent>,
    time: Res<Time>,
) {
    for (reader, mut text) in reader_query.iter_mut() {
        for output_event in event_reader.iter() {
            if output_event.box_ == reader.box_.expect("BoxReader should have Some box_") {
                text.sections[0].value = output_event.output.to_string();
                text.sections[0].style.color.set_a(1.);
            }
        }
        let alpha = (text.sections[0].style.color.a() - (2. * time.delta_seconds())).max(0.4);
        text.sections[0].style.color.set_a(alpha);
    }
}

pub fn render_progression(
    prog_query: Query<&Progression>,
    mut piece_query: Query<(&mut Handle<ColorMaterial>, &ProgressionPiece)>,
    color_handles: Res<ColorHandles>,
) {
    for (mut color, piece) in piece_query.iter_mut() {
        *color = if piece.index
            < prog_query
                .get_component::<Progression>(piece.progression)
                .unwrap()
                .answer
                .len()
        {
            color_handles.green.clone_weak()
        } else {
            color_handles.white.clone_weak()
        };
    }
}
