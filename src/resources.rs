use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct LevelNum(pub usize);

#[derive(Clone, Eq, PartialEq, Debug, Default, Hash)]
pub struct ColorHandles {
    pub white: Handle<ColorMaterial>,
    pub green: Handle<ColorMaterial>,
}
