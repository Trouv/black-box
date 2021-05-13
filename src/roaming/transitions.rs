use crate::roaming::components::*;
use bevy::prelude::*;
use heron::prelude::*;
pub fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());

    commands
        .spawn_bundle((Transform::from_xyz(0., 1., 2.), GlobalTransform::identity()))
        .insert(Body::Capsule {
            radius: 0.5,
            half_segment: 1.,
        })
        .insert(RotationConstraints::lock())
        .insert(Velocity::default())
        .insert(Player)
        .insert(Strafes)
        .with_children(|parent| {
            parent
                .spawn_bundle((Transform::default(), GlobalTransform::identity()))
                .insert(Turn::default())
                .insert(Player)
                .with_children(|parent| {
                    let transform = Transform::from_xyz(0., 0.8, 0.)
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

pub fn light_setup(mut commands: Commands) {
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(-2., 10., 2.),
        ..Default::default()
    });
}

pub fn floor_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 100. })),
            material: materials.add(StandardMaterial {
                base_color: Color::GRAY,
                ..Default::default()
            }),
            ..Default::default()
        })
        .insert(BodyType::Static)
        .insert(Body::Cuboid {
            half_extends: Vec3::new(50., 0., 50.),
        });
}

pub fn grab_cursor(mut windows: ResMut<Windows>) {
    let window = windows.get_primary_mut().unwrap();

    window.set_cursor_lock_mode(true);
    window.set_cursor_visibility(false);
}