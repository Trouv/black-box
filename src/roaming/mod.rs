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
    use crate::roaming::{
        components::*,
        resources::{LookSensitivity, WalkSpeed},
    };
    use bevy::{input::mouse::MouseMotion, prelude::*};
    use heron::prelude::*;

    const PI: f32 = 3.14159265;
    pub fn roaming_movement(
        mut velocity_query: Query<(&mut Velocity, &mut Transform), (With<Player>, With<Strafes>)>,
        input: Res<Input<KeyCode>>,
        time: Res<Time>,
        walk_speed: Res<WalkSpeed>,
        look_sensitivity: Res<LookSensitivity>,
        mut mouse_listener: EventReader<MouseMotion>,
    ) {
        for (mut velocity, mut transform) in velocity_query.iter_mut() {
            for motion_event in mouse_listener.iter() {
                transform.rotate(Quat::from_rotation_y(
                    motion_event.delta.x * -1.0 * look_sensitivity.0 * time.delta_seconds(),
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
            if linear.length() > 1.0 {
                linear.normalize();
            }
            linear *= walk_speed.0;
            velocity.linear = transform.rotation * linear;
        }
    }

    pub fn camera_tilt(
        mut transform_query: Query<&mut Transform, (With<Player>, With<Tilts>)>,
        time: Res<Time>,
        look_sensitivity: Res<LookSensitivity>,
        mut mouse_listener: EventReader<MouseMotion>,
    ) {
        for motion_event in mouse_listener.iter() {
            for mut transform in transform_query.iter_mut() {
                transform.rotate(Quat::from_rotation_x(
                    motion_event.delta.y * -1. * look_sensitivity.0 * time.delta_seconds(),
                ));

                let mut euler = transform.rotation.to_axis_angle();
                euler.1 = euler.1.min(PI / 2.);
                transform.rotation = Quat::from_axis_angle(euler.0, euler.1);
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

    pub fn grab_cursor(mut windows: ResMut<Windows>) {
        let window = windows.get_primary_mut().unwrap();

        window.set_cursor_lock_mode(true);
        window.set_cursor_visibility(false);
    }
}

pub mod resources {
    use serde::{Deserialize, Serialize};
    #[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
    pub struct WalkSpeed(pub f32);

    #[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
    pub struct LookSensitivity(pub f32);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct RoamingPlugin;

impl Plugin for RoamingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(resources::WalkSpeed(2.))
            .insert_resource(resources::LookSensitivity(0.2))
            .add_system_set(
                SystemSet::on_enter(AppState::Roaming)
                    .with_system(transitions::camera_setup.system())
                    .with_system(transitions::world_setup.system())
                    .with_system(transitions::grab_cursor.system())
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
