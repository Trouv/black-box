use crate::{
    box_internal::components::BoxState,
    roaming::{
        components::*,
        resources::{LookSensitivity, WalkSpeed},
    },
    standard_box::components::Active,
    AppState,
};
use bevy::{input::mouse::MouseMotion, prelude::*};
use bevy_mod_picking::{PickableMesh, PickingCamera};
use heron::prelude::*;

pub fn roaming_movement(
    mut velocity_query: Query<&mut Velocity, (With<Player>, With<Strafes>)>,
    turn_query: Query<&Turn, With<Player>>,
    input: Res<Input<KeyCode>>,
    walk_speed: Res<WalkSpeed>,
) {
    for mut velocity in velocity_query.iter_mut() {
        let mut linear = Vec3::ZERO;

        if input.pressed(KeyCode::W) {
            linear += Vec3::new(0., 0., -1.);
        }
        if input.pressed(KeyCode::A) {
            linear += Vec3::new(-1., 0., 0.);
        }
        if input.pressed(KeyCode::S) {
            linear += Vec3::new(0., 0., 1.);
        }
        if input.pressed(KeyCode::D) {
            linear += Vec3::new(1., 0., 0.);
        }
        if linear.length() > 1.0 {
            linear.normalize();
        }
        linear *= walk_speed.0;
        let turn_quat = Quat::from(
            *turn_query
                .iter()
                .next()
                .expect("There should exist a Player that can Turn"),
        );
        linear = turn_quat * linear;
        velocity.linear.x = linear.x;
        velocity.linear.z = linear.z;
    }
}

pub fn camera_tilt(
    mut tilt_query: Query<(&mut Transform, &mut Tilt), With<Player>>,
    time: Res<Time>,
    look_sensitivity: Res<LookSensitivity>,
    mut mouse_listener: EventReader<MouseMotion>,
) {
    for motion_event in mouse_listener.iter() {
        for (mut transform, mut tilt) in tilt_query.iter_mut() {
            tilt.update(motion_event.delta.y * -1. * look_sensitivity.0 * time.delta_seconds());

            transform.rotation = Quat::from(*tilt);
        }
    }
}

pub fn body_turn(
    mut turn_query: Query<(&mut Transform, &mut Turn), With<Player>>,
    time: Res<Time>,
    look_sensitivity: Res<LookSensitivity>,
    mut mouse_listener: EventReader<MouseMotion>,
) {
    for motion_event in mouse_listener.iter() {
        for (mut transform, mut turn) in turn_query.iter_mut() {
            turn.update(motion_event.delta.x * -1. * look_sensitivity.0 * time.delta_seconds());

            transform.rotation = Quat::from(*turn);
        }
    }
}

pub fn box_interaction(
    mut commands: Commands,
    picking_query: Query<&PickingCamera, With<Player>>,
    box_query: Query<Entity, (With<BoxState>, With<PickableMesh>)>,
    input: Res<Input<KeyCode>>,
    mut state: ResMut<State<AppState>>,
) {
    for picking_camera in picking_query.iter() {
        if let Some((picked_entity, intersection)) = picking_camera.intersect_top() {
            if box_query.get(picked_entity).is_ok() && intersection.distance() <= 2. {
                if input.just_pressed(KeyCode::E) {
                    commands.entity(picked_entity).insert(Active);
                    state.push(AppState::StandardBox).unwrap();
                }
            }
        }
    }
}
