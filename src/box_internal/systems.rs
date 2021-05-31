use crate::box_internal::{
    components::{ActionScript, BoxState, Itemized, PipeIn, PipePass, Pressable, Progression},
    BoxCompletedEvent, OutputEvent,
};
use bevy::prelude::*;

pub fn button_output(
    mut box_query: Query<&mut BoxState>,
    button_query: Query<(&Pressable, &ActionScript, &Itemized), Changed<Pressable>>,
    mut output_writer: EventWriter<OutputEvent>,
) {
    for (pressable, action_script, itemized) in button_query.iter() {
        if pressable.just_unpressed() {
            let mut box_ = box_query
                .get_mut(itemized.collector)
                .expect("Itemized component on button isn't pointing to a Box!");
            for action in action_script {
                let out = action.evaluate(&mut box_);
                if let Some(o) = out {
                    output_writer.send(OutputEvent {
                        box_: itemized.collector,
                        output: o.clone(),
                    });
                }
            }
        }
    }
}

pub fn progression(
    mut prog_query: Query<(Entity, &mut Progression, &PipeIn)>,
    mut output_reader: EventReader<OutputEvent>,
    mut completed_writer: EventWriter<BoxCompletedEvent>,
) {
    for output in output_reader.iter() {
        for (entity, mut progression, pipe_in) in prog_query.iter_mut() {
            if let Some(e) = pipe_in.out_entity {
                if e == output.box_ {
                    progression.update(output.output.clone());
                }
                if progression.progress() >= progression.total() {
                    completed_writer.send(BoxCompletedEvent { box_: entity });
                }
            }
        }
    }
}

pub fn pipe_pass_in(
    pass_query: Query<(Entity, &PipeIn), With<PipePass>>,
    mut output_reader: EventReader<OutputEvent>,
) -> Vec<OutputEvent> {
    let mut output_events = Vec::new();
    for output in output_reader.iter() {
        for (entity, pipe_in) in pass_query.iter() {
            if let Some(e) = pipe_in.out_entity {
                if e == output.box_ {
                    output_events.push(OutputEvent {
                        box_: entity,
                        ..output.clone()
                    });
                }
            }
        }
    }
    output_events
}

pub fn pipe_pass_out(In(data): In<Vec<OutputEvent>>, mut output_writer: EventWriter<OutputEvent>) {
    for output in data {
        output_writer.send(output);
    }
}