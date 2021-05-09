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
