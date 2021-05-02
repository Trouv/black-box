use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ProgressionPiece;

#[derive(Default, Serialize, Deserialize)]
#[serde(default)]
#[serde(deny_unknown_fields)]
pub struct BoxReader {
    #[serde(skip)]
    pub box_: Option<Entity>,
}

impl Clone for BoxReader {
    fn clone(&self) -> Self {
        BoxReader { box_: self.box_ }
    }
}

impl BoxReader {
    pub fn new(box_: Entity) -> BoxReader {
        BoxReader { box_: Some(box_) }
    }
}
