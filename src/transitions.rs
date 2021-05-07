use crate::resources::ColorHandles;
use bevy::prelude::*;

pub fn camera_setup(mut commands: Commands) {
    commands.spawn_bundle(UiCameraBundle::default());
    commands.spawn_bundle(PerspectiveCameraBundle {
        transform: Transform::from_xyz(0., 1.1, 0.8).looking_at(Vec3::new(0., 0., -0.2), Vec3::Y),
        ..Default::default()
    });
    commands.spawn_bundle(LightBundle {
        transform: Transform::from_xyz(-2., 2., 2.),
        ..Default::default()
    });
}

pub fn add_colors(mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
    commands.insert_resource(ColorHandles {
        white: materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.9))),
        green: materials.add(ColorMaterial::color(Color::rgb(0.36, 0.63, 0.36))),
    });
}
