use crate::{
    box_internal::{components::*, BoxData},
    roaming::components::*,
    LEVEL_ORDER,
};
use bevy::prelude::*;
use bevy_mod_raycast::{BoundVol, RayCastMesh, RayCastSource};
use heron::prelude::*;
use std::convert::TryFrom;

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
                        .insert(Tilt::new(transform.rotation.to_axis_angle().1 * -1.))
                        .insert(RayCastSource::<BoxRayCastSet>::new_transform_empty());
                });
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
                base_color: Color::DARK_GRAY,
                roughness: 0.8,
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

const BOX_DISTANCE: f32 = 2.;

pub fn light_setup(mut commands: Commands) {
    let num_lights = LEVEL_ORDER.len() / 3;
    let total_length = (LEVEL_ORDER.len() - 1) as f32 * BOX_DISTANCE;
    let light_distance = total_length / (num_lights - 1) as f32;
    for i in 0..num_lights {
        commands.spawn_bundle(LightBundle {
            transform: Transform::from_xyz(i as f32 * light_distance, 10., 0.),
            ..Default::default()
        });
    }
}

pub fn black_box_setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
) {
    for (i, level) in LEVEL_ORDER.iter().enumerate() {
        let level_data =
            BoxData::try_from(*level).unwrap_or_else(|_| panic!("Unable to load level {}", 1));
        spawn_box(
            &level_data,
            Transform::from_xyz(i as f32 * BOX_DISTANCE, 0.5, 0.),
            &mut commands,
            &server,
            &mut meshes,
            &mut standard_materials,
        );
    }
}

pub fn spawn_box(
    level_data: &BoxData,
    base_transform: Transform,
    commands: &mut Commands,
    server: &Res<AssetServer>,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
) -> Entity {
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Cube::new(1.))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            transform: base_transform,
            ..Default::default()
        })
        .insert(BodyType::Static)
        .insert(Body::Cuboid {
            half_extends: Vec3::new(0.5, 0.5, 0.5),
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Box::new(0.4, 0.2, 0.4))),
                    material: materials.add(StandardMaterial {
                        base_color: Color::NONE,
                        ..Default::default()
                    }),
                    transform: Transform::from_xyz(0., 0.625, 0.),
                    ..Default::default()
                })
                .insert(BoxState::default())
                .insert(Progression::new(level_data.prompt.clone()))
                .insert(RayCastMesh::<BoxRayCastSet>::default())
                .insert(BoundVol::default())
                .with_children(|parent| {
                    parent.spawn_scene(server.load("models/box.glb#Scene0"));

                    let box_ = parent.parent_entity();
                    for (i, button_data) in level_data.buttons.iter().enumerate() {
                        parent
                            .spawn_bundle((
                                Transform::from_translation(button_data.translation),
                                GlobalTransform::identity(),
                            ))
                            .with_children(|parent| {
                                parent.spawn_scene(server.load("models/button_base.glb#Scene0"));
                                parent
                                    .spawn_bundle((
                                        Transform::default(),
                                        GlobalTransform::identity(),
                                    ))
                                    .insert(button_data.button.clone())
                                    .insert(Itemized {
                                        collector: box_,
                                        index: i,
                                    })
                                    .insert(Pressable::default())
                                    .with_children(|parent| {
                                        parent.spawn_scene(
                                            server.load("models/button_body.glb#Scene0"),
                                        );
                                    });
                            });
                    }
                });
        })
        .id()
}
