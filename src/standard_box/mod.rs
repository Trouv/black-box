pub mod systems;
pub mod transitions;

use bevy::prelude::*;

pub const BUTTON_NUMS: [KeyCode; 6] = [
    KeyCode::Key1,
    KeyCode::Key2,
    KeyCode::Key3,
    KeyCode::Key4,
    KeyCode::Key5,
    KeyCode::Key6,
];

pub mod components {
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

    #[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
    pub struct BoxUiRoot(pub Entity);
}
