use crate::AppState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components {
    use serde::{Deserialize, Serialize};
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct Strafes;
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct Tilts;
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct Player;
}

pub mod systems {
    use crate::roaming::components::*;
    use bevy::{input::mouse::MouseMotion, prelude::*};
    use heron::prelude::*;

    pub fn roaming_movement(
        mut velocity_query: Query<(&mut Velocity, &mut Transform), (With<Player>, With<Strafes>)>,
        input: Res<Input<KeyCode>>,
        time: Res<Time>,
        mut mouse_listener: EventReader<MouseMotion>,
    ) {
        for (mut velocity, mut transform) in velocity_query.iter_mut() {
            for motion_event in mouse_listener.iter() {
                transform.rotate(Quat::from_rotation_y(
                    motion_event.delta.x * -0.1 * time.delta_seconds(),
                ));
            }

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
            velocity.linear = transform.rotation * linear;
        }
    }

    pub fn camera_tilt(
        mut transform_query: Query<&mut Transform, (With<Player>, With<Tilts>)>,
        time: Res<Time>,
        mut mouse_listener: EventReader<MouseMotion>,
    ) {
        for motion_event in mouse_listener.iter() {
            for mut transform in transform_query.iter_mut() {
                transform.rotate(Quat::from_rotation_x(
                    motion_event.delta.y * -0.1 * time.delta_seconds(),
                ));
            }
        }
    }
}

pub mod transitions {
    use crate::roaming::components::*;
    use bevy::prelude::*;
    use heron::prelude::*;
    pub fn camera_setup(mut commands: Commands) {
        commands.spawn_bundle(UiCameraBundle::default());

        commands
            .spawn_bundle((
                Transform::from_xyz(0., 0., 0.8),
                GlobalTransform::identity(),
            ))
            .insert(Body::Capsule {
                radius: 0.5,
                half_segment: 2.0,
            })
            .insert(RotationConstraints::restrict_to_y_only())
            .insert(Velocity::default())
            .insert(Player)
            .insert(Strafes)
            .with_children(|parent| {
                parent
                    .spawn_bundle(PerspectiveCameraBundle {
                        transform: Transform::from_xyz(0., 1.1, 0.)
                            .looking_at(Vec3::new(0., 0., -1.), Vec3::Y),
                        ..Default::default()
                    })
                    .insert(Tilts)
                    .insert(Player);
            });
    }
    pub fn world_setup(mut commands: Commands) {
        commands.spawn_bundle(LightBundle {
            transform: Transform::from_xyz(-2., 2., 2.),
            ..Default::default()
        });
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct RoamingPlugin;

impl Plugin for RoamingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_system_set(
            SystemSet::on_enter(AppState::Roaming)
                .with_system(transitions::camera_setup.system())
                .with_system(transitions::world_setup.system())
                .with_system(crate::standard_box::transitions::black_box_setup.system()),
        )
        .add_system_set(
            SystemSet::on_update(AppState::Roaming)
                .with_system(systems::roaming_movement.system())
                .with_system(systems::camera_tilt.system()),
        )
        .add_system_set(SystemSet::on_update(AppState::Roaming))
        .add_system_set(SystemSet::on_exit(AppState::Roaming));
    }
}
