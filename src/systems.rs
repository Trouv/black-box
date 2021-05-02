use crate::{
    components::{
        BoxReader, BoxState, ButtonScript, Itemized, OutputEvent, Pressable, Progression,
        ProgressionPiece, BUTTON_NUMS,
    },
    level::ColorHandles,
};
use bevy::prelude::*;

pub fn render_button(mut button_query: Query<(&Pressable, &mut Transform)>) {
    for (button, mut transform) in button_query.iter_mut() {
        if button.pressed {
            transform.translation = Vec3::new(0., -0.02, 0.);
        } else {
            transform.translation = Vec3::ZERO;
        }
    }
}

pub fn button_input(
    mut button_query: Query<(&mut Pressable, &Itemized), With<ButtonScript>>,
    input: Res<Input<KeyCode>>,
) {
    for (mut pressable, itemized) in button_query.iter_mut() {
        let last_pressed = pressable.pressed;
        pressable.pressed = input.pressed(BUTTON_NUMS[itemized.index]);
        pressable.just_pressed = pressable.pressed && !last_pressed;
        pressable.just_unpressed = !pressable.pressed && last_pressed;
    }
}

pub fn update_box_state(
    mut box_query: Query<(&mut BoxState, &mut Progression)>,
    button_query: Query<(&Pressable, &ButtonScript, &Itemized)>,
    mut event_writer: EventWriter<OutputEvent>,
) {
    for (pressable, button_script, itemized) in button_query.iter() {
        if pressable.just_unpressed {
            let (mut box_, mut progression) = box_query
                .get_mut(itemized.collector)
                .expect("Itemized component on button isn't pointing to a Box!");
            for action in &button_script.0 {
                let out = action.evaluate(&mut box_);
                if let Some(o) = out {
                    progression.update(o.clone());
                    event_writer.send(OutputEvent {
                        box_: itemized.collector,
                        output: o.clone(),
                    });
                }
            }
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
    mut piece_query: Query<(&mut Handle<ColorMaterial>, &Itemized), With<ProgressionPiece>>,
    color_handles: Res<ColorHandles>,
) {
    for (mut color, piece) in piece_query.iter_mut() {
        *color = if piece.index
            < prog_query
                .get_component::<Progression>(piece.collector)
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
