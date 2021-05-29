use crate::{
    box_internal::{components::*, BoxData},
    resources::ColorHandles,
    roaming::components::*,
    standard_box::{
        components::{Active, BoxOutDisplay, BoxReference},
        StandardBoxEvent,
    },
    AppState, LEVEL_ORDER,
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
    color_handles: Res<ColorHandles>,
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
            &color_handles,
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
    color_handles: &Res<ColorHandles>,
) {
    // Spawn desk
    commands
        .spawn_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Box::new(1., 1., 1.5))),
            material: materials.add(StandardMaterial {
                base_color: Color::WHITE,
                ..Default::default()
            }),
            transform: Transform::from_translation(
                base_transform.translation + Vec3::new(0., 0., -0.25),
            ),
            ..Default::default()
        })
        .insert(BodyType::Static)
        .insert(Body::Cuboid {
            half_extends: Vec3::new(0.5, 0.5, 0.75),
        });
    // Spawn box
    let box_entity = commands
        .spawn_bundle(PbrBundle {
            mesh: server.load("models/box.glb#Mesh0/Primitive0"),
            material: server.load("models/box.glb#Material0"),
            transform: Transform::from_translation(
                base_transform.translation + Vec3::new(0., 0.625, 0.),
            ),
            ..Default::default()
        })
        .insert(BoxState::default())
        .insert(Progression::new(level_data.prompt.clone()))
        .insert(RayCastMesh::<BoxRayCastSet>::default())
        .insert(BoundVol::default())
        .with_children(|parent| {
            let box_ = parent.parent_entity();
            for (i, button_data) in level_data.buttons.iter().enumerate() {
                parent
                    .spawn_bundle((
                        Transform::from_translation(
                            button_data.translation + Vec3::new(0., 0.125, 0.),
                        ),
                        GlobalTransform::identity(),
                    ))
                    .with_children(|parent| {
                        parent.spawn_bundle(PbrBundle {
                            mesh: server.load("models/button.glb#Mesh0/Primitive0"),
                            material: server.load("models/button.glb#Material0"),
                            ..Default::default()
                        });

                        parent
                            .spawn_bundle(PbrBundle {
                                mesh: server.load("models/button.glb#Mesh1/Primitive0"),
                                material: server.load("models/button.glb#Material1"),
                                ..Default::default()
                            })
                            .insert(button_data.button.clone())
                            .insert(Itemized {
                                collector: box_,
                                index: i,
                            })
                            .insert(Pressable::default());
                    });
            }
        })
        .id();
    // Spawn display
    commands.spawn_bundle(PbrBundle {
        mesh: server.load("models/display.glb#Mesh0/Primitive0"),
        material: server.load("models/display.glb#Material0"),
        transform: Transform::from_translation(
            base_transform.translation + Vec3::new(0., 0.625, -0.5),
        ),
        ..Default::default()
    });
    commands
        .spawn_bundle(NodeBundle {
            style: Style {
                position_type: PositionType::Absolute,
                position: Rect {
                    top: Val::Percent(10.),
                    right: Val::Percent(0.),
                    ..Default::default()
                },
                size: Size::new(Val::Percent(100.), Val::Percent(40.)),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..Default::default()
            },
            material: color_handles.none.clone_weak(),
            ..Default::default()
        })
        .with_children(|parent| {
            parent
                .spawn_bundle(TextBundle {
                    text: Text::with_section(
                        "TEST".to_string(),
                        TextStyle {
                            font: server.load("fonts/rainyhearts.ttf"),
                            font_size: 200.,
                            color: Color::rgb(0.36, 0.63, 0.36),
                        },
                        TextAlignment::default(),
                    ),
                    ..Default::default()
                })
                .insert(BoxOutDisplay)
                .insert(BoxReference::new(box_entity));
        });
}

pub fn enter_box(
    mut commands: Commands,
    mut velocity_query: Query<&mut Velocity, (With<Player>, With<Strafes>)>,
    mut state: ResMut<State<AppState>>,
    mut reader: EventReader<StandardBoxEvent>,
) {
    for event in reader.iter() {
        if let StandardBoxEvent::Enter(box_) = event {
            commands.entity(*box_).insert(Active);
            state
                .overwrite_push(AppState::StandardBox)
                .expect("State is already StandardBox");
            for mut velocity in velocity_query.iter_mut() {
                velocity.linear.x = 0.;
                velocity.linear.z = 0.;
            }
        }
    }
}
