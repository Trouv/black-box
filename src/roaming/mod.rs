use crate::AppState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components {
    const PI: f32 = 3.14159265;
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};
    #[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
    pub struct Strafes;
    #[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
    pub struct Turn {
        theta: f32,
    }

    impl Turn {
        pub fn update(&mut self, delta: f32) {
            self.theta += delta;
            let tau = PI * 2.;
            self.theta = ((self.theta % tau) + tau) % tau;
        }

        pub fn new(theta: f32) -> Self {
            let mut turn = Turn::default();
            turn.update(theta);
            turn
        }
    }

    impl From<Turn> for Quat {
        fn from(turn: Turn) -> Quat {
            Quat::from_axis_angle(Vec3::Y, turn.theta)
        }
    }

    #[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
    pub struct Tilt {
        phi: f32,
    }

    impl Tilt {
        pub fn update(&mut self, delta: f32) {
            self.phi += delta;
            self.phi = self.phi.min(PI / 2.).max(PI / -2.);
        }

        pub fn new(phi: f32) -> Self {
            let mut tilt = Tilt::default();
            tilt.update(phi);
            tilt
        }
    }

    impl From<Tilt> for Quat {
        fn from(tilt: Tilt) -> Quat {
            Quat::from_axis_angle(Vec3::X, tilt.phi)
        }
    }

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
            velocity.linear = turn_quat * linear;
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
                    .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                    .insert(Turn::default())
                    .insert(Player)
                    .with_children(|parent| {
                        let transform = Transform::from_xyz(0., 1.1, 0.)
                            .looking_at(Vec3::new(0., 0., -1.), Vec3::Y);

                        parent
                            .spawn_bundle(PerspectiveCameraBundle {
                                transform,
                                ..Default::default()
                            })
                            .insert(Player)
                            .insert(Tilt::new(transform.rotation.to_axis_angle().1 * -1.));
                    });
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
            .insert_resource(resources::LookSensitivity(0.06))
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
                    .with_system(systems::body_turn.system())
                    .with_system(systems::camera_tilt.system()),
            )
            .add_system_set(SystemSet::on_update(AppState::Roaming))
            .add_system_set(SystemSet::on_exit(AppState::Roaming));
    }
}
