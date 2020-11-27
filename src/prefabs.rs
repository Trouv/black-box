use crate::components::*;
use amethyst::{
    assets::{PrefabData, ProgressCounter},
    core::transform::Transform,
    derive::PrefabData,
    ecs::{Entity, WriteStorage},
    renderer::sprite::prefab::SpriteRenderPrefab,
    ui::{UiImagePrefab, UiTransformData},
    Error,
};
use serde::{Deserialize, Serialize};

pub struct SelectionGroup {}

#[derive(Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct BoxPrefab {
    black_box: BlackBox,
    sprite: SpriteRenderPrefab,
    transform: Transform,
}

#[derive(Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct ProgressPrefab {
    progression: Progression,
    ui_transform: UiTransformData<SelectionGroup>,
    box_reader: BoxReader,
}

// Needs to be level-specific
pub struct ButtonPrefab {}

// Needs to be generated based off the prompt
pub struct ProgressPiecePrefab {}

#[derive(Serialize, Deserialize, PrefabData)]
#[serde(deny_unknown_fields)]
pub struct Display {
    ui_transform: UiTransformData<SelectionGroup>,
    ui_image: UiImagePrefab,
    box_reader: BoxReader,
}
