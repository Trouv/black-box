pub mod components;
pub mod systems;
pub mod transitions;

use crate::AppState;
use bevy::prelude::*;
use heron::prelude::*;
use serde::{Deserialize, Serialize};

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
        app.insert_resource(resources::WalkSpeed(2.))
            .insert_resource(resources::LookSensitivity(0.06))
            .insert_resource(Gravity::from(Vec3::new(0.0, -9.81, 0.0)))
            .add_system_set(
                SystemSet::on_enter(AppState::Roaming)
                    .with_system(transitions::camera_setup.system())
                    .with_system(transitions::light_setup.system())
                    .with_system(transitions::floor_setup.system())
                    .with_system(transitions::grab_cursor.system())
                    .with_system(crate::standard_box::transitions::black_box_setup.system()),
            )
            .add_system_set(
                SystemSet::on_update(AppState::Roaming)
                    .with_system(systems::roaming_movement.system())
                    .with_system(systems::body_turn.system())
                    .with_system(systems::camera_tilt.system()),
            )
            .add_system_set(SystemSet::on_update(AppState::Roaming))
            .add_system_set(SystemSet::on_exit(AppState::Roaming));
    }
}
