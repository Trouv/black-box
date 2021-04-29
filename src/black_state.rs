use crate::{
    components::Progression,
    level::{LevelData, LevelNum, LEVEL_ORDER},
};
use bevy::prelude::*;
use std::convert::TryFrom;

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum AppState {
    BlackBox,
    IntoBlackBox,
}

pub fn black_box_setup(
    mut commands: Commands,
    server: Res<AssetServer>,
    level_num: Res<LevelNum>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut level_data = LevelData::try_from(LEVEL_ORDER[(level_num.0 - 1) % LEVEL_ORDER.len()])
        .expect(format!("Unable to load level {}", level_num.0).as_str());
    level_data.init(&mut commands, &server, &mut materials, level_num.0);
    commands.insert_resource(level_data);
}

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

pub fn level_completion(
    progress_query: Query<&Progression>,
    mut state: ResMut<State<AppState>>,
    mut level_num: ResMut<LevelNum>,
) {
    for progress in progress_query.iter() {
        if progress.answer.len() >= progress.prompt.len() {
            state.replace(AppState::IntoBlackBox).unwrap();
            level_num.0 += 1;
        }
    }
}

pub fn into_black_box(mut state: ResMut<State<AppState>>) {
    state.replace(AppState::BlackBox).unwrap();
}

pub fn black_box_cleanup(mut commands: Commands, level_data: Res<LevelData>) {
    for entity in &level_data.entities {
        commands.entity(*entity).despawn_recursive();
    }
}
