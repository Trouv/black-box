use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default, Serialize, Deserialize)]
pub struct ProgressionPiece;

#[derive(Copy, Eq, PartialEq, Debug)]
pub struct BoxReader {
    pub box_: Entity,
}

impl Clone for BoxReader {
    fn clone(&self) -> Self {
        BoxReader { box_: self.box_ }
    }
}

impl BoxReader {
    pub fn new(box_: Entity) -> BoxReader {
        BoxReader { box_ }
    }
}
