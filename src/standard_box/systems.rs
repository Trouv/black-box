use crate::{
    box_internal::{
        components::{ActionScript, Itemized, Pressable, Progression},
        OutputEvent,
    },
    resources::ColorHandles,
    standard_box::{
        components::{BoxReader, ProgressionPiece},
        BUTTON_NUMS,
    },
};
use bevy::prelude::*;

pub fn button_input(
    mut button_query: Query<(&mut Pressable, &Itemized), With<ActionScript>>,
    input: Res<Input<KeyCode>>,
) {
    if input.is_changed() {
        for (mut pressable, itemized) in button_query.iter_mut() {
            let pressed = input.pressed(BUTTON_NUMS[itemized.index]);
            if pressable.update_necessary(pressed) {
                pressable.update(input.pressed(BUTTON_NUMS[itemized.index]));
            }
        }
    }
}

pub fn render_button(
    mut button_query: Query<(&Pressable, &mut Transform), (With<ActionScript>, Changed<Pressable>)>,
) {
    for (pressable, mut transform) in button_query.iter_mut() {
        if pressable.pressed() {
            transform.translation = Vec3::new(0., -0.02, 0.);
        } else {
            transform.translation = Vec3::ZERO;
        }
    }
}

pub fn render_display(
    mut reader_query: Query<(&mut BoxReader, &mut Text)>,
    mut event_reader: EventReader<OutputEvent>,
    time: Res<Time>,
) {
    for (reader, mut text) in reader_query.iter_mut() {
        for output_event in event_reader.iter() {
            if output_event.box_ == reader.box_ {
                text.sections[0].value = output_event.output.to_string();
                text.sections[0].style.color.set_a(1.);
            }
        }
        let alpha = (text.sections[0].style.color.a() - (2. * time.delta_seconds())).max(0.4);
        text.sections[0].style.color.set_a(alpha);
    }
}

pub fn render_progression(
    prog_query: Query<&Progression, Changed<Progression>>,
    mut piece_query: Query<(&mut Handle<ColorMaterial>, &Itemized), With<ProgressionPiece>>,
    color_handles: Res<ColorHandles>,
) {
    for (mut color, piece) in piece_query.iter_mut() {
        if let Ok(progression) = prog_query.get_component::<Progression>(piece.collector) {
            *color = if piece.index < progression.progress() {
                color_handles.green.clone_weak()
            } else {
                color_handles.white.clone_weak()
            };
        }
    }
}
