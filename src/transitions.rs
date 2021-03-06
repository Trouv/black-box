use crate::resources::ColorHandles;
use bevy::prelude::*;

pub fn add_colors(mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
    commands.insert_resource(ColorHandles {
        white: materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.9))),
        green: materials.add(ColorMaterial::color(Color::rgb(0.36, 0.63, 0.36))),
    });
}
