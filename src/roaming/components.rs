const PI: f32 = 3.14159265;
use bevy::prelude::*;
use serde::{Deserialize, Serialize};
#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Strafes;
#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Turn {
    theta: f32,
}

impl Turn {
    pub fn update(&mut self, delta: f32) {
        self.theta += delta;
        let tau = PI * 2.;
        self.theta = ((self.theta % tau) + tau) % tau;
    }

    pub fn new(theta: f32) -> Self {
        let mut turn = Turn::default();
        turn.update(theta);
        turn
    }
}

impl From<Turn> for Quat {
    fn from(turn: Turn) -> Quat {
        Quat::from_axis_angle(Vec3::Y, turn.theta)
    }
}

#[derive(Copy, Clone, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct Tilt {
    phi: f32,
}

impl Tilt {
    pub fn update(&mut self, delta: f32) {
        self.phi += delta;
        self.phi = self.phi.min(PI / 2.).max(PI / -2.);
    }

    pub fn new(phi: f32) -> Self {
        let mut tilt = Tilt::default();
        tilt.update(phi);
        tilt
    }
}

impl From<Tilt> for Quat {
    fn from(tilt: Tilt) -> Quat {
        Quat::from_axis_angle(Vec3::X, tilt.phi)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct Player;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Hash, Serialize, Deserialize)]
pub struct BoxRayCastSet;
