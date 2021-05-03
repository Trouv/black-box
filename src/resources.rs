use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct LevelNum(pub usize);

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct ColorHandles {
    pub white: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
}

pub fn add_colors(mut materials: ResMut<Assets<ColorMaterial>>, mut commands: Commands) {
    commands.insert_resource(ColorHandles {
        white: materials.add(ColorMaterial::color(Color::rgb(0.9, 0.9, 0.9))),
        green: materials.add(ColorMaterial::color(Color::rgb(0.36, 0.63, 0.36))),
    });
}
