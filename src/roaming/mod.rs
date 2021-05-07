use crate::AppState;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod components {}
pub mod systems {}
pub mod transitions {
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
            .with_children(|parent| {
                parent.spawn_bundle(PerspectiveCameraBundle {
                    transform: Transform::from_xyz(0., 1.1, 0.)
                        .looking_at(Vec3::new(0., 0., -1.), Vec3::Y),
                    ..Default::default()
                });
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
        .add_system_set(SystemSet::on_update(AppState::Roaming))
        .add_system_set(SystemSet::on_update(AppState::Roaming))
        .add_system_set(SystemSet::on_exit(AppState::Roaming));
    }
}
