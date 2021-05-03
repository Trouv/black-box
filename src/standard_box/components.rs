use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash, Default, Serialize, Deserialize)]
pub struct ProgressionPiece;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
pub struct BoxReader {
    pub box_: Entity,
}

impl BoxReader {
    pub fn new(box_: Entity) -> BoxReader {
        BoxReader { box_ }
    }
}
