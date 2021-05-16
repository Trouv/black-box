pub mod components;
pub mod systems;
pub mod transitions;

use crate::{AppState, SystemLabels};
use bevy::prelude::*;
use bevy_mod_raycast::{build_rays, update_raycast, PluginState, RaycastSystem};
use heron::prelude::*;
use serde::{Deserialize, Serialize};
use std::marker::PhantomData;

pub mod resources {
    use serde::{Deserialize, Serialize};
    #[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
    pub struct WalkSpeed(pub f32);

    #[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
    pub struct LookSensitivity(pub f32);
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct RoamingPlugin;

impl Plugin for RoamingPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.insert_resource(resources::WalkSpeed(3.))
            .insert_resource(resources::LookSensitivity(0.06))
            .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
            .add_system_set(
                SystemSet::on_enter(AppState::Roaming)
                    .with_system(transitions::camera_setup.system())
                    .with_system(transitions::light_setup.system())
                    .with_system(transitions::floor_setup.system())
                    .with_system(transitions::grab_cursor.system())
                    .with_system(transitions::black_box_setup.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Roaming)
                    .with_system(systems::roaming_movement.system())
                    .label(SystemLabels::InputLabel),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Roaming)
                    .with_system(transitions::enter_box.system())
                    .after(SystemLabels::InputLabel),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Roaming)
                    .with_system(systems::body_turn.system())
                    .with_system(systems::camera_tilt.system())
                    .with_system(systems::box_interaction.system()),
            );
    }
}

pub struct RaycastingPluginNoDebug<T: 'static + Send + Sync>(pub PhantomData<T>);

impl<T: 'static + Send + Sync> Plugin for RaycastingPluginNoDebug<T> {
    fn build(&self, app: &mut AppBuilder) {
        app.init_resource::<PluginState<T>>()
            .add_system_to_stage(
                CoreStage::PostUpdate,
                build_rays::<T>.system().label(RaycastSystem::BuildRays),
            )
            .add_system_to_stage(
                CoreStage::PostUpdate,
                update_raycast::<T>
                    .system()
                    .label(RaycastSystem::UpdateRaycast)
                    .after(RaycastSystem::BuildRays),
            );
    }
}
impl<T: 'static + Send + Sync> Default for RaycastingPluginNoDebug<T> {
    fn default() -> Self {
        RaycastingPluginNoDebug(PhantomData::<T>)
    }
}
