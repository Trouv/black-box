use amethyst::{
    core::transform::Transform,
    ecs::{Entity, Join, World, WorldExt},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::Camera,
};

use crate::{
    components::{BoxReader, Button, Progression, ProgressionPiece},
    level::{LevelData, LEVEL_ORDER},
};

use std::convert::TryFrom;

pub struct BlackState {
    level_num: usize,
    progression: Option<Entity>,
    box_: Option<Entity>,
}

impl From<usize> for BlackState {
    fn from(level_num: usize) -> BlackState {
        BlackState {
            level_num,
            progression: None,
            box_: None,
            // This sort of thing would probably be handled better by resources, but I'm planning
            // on having many boxes in one scene so this is a temporary solution
        }
    }
}

impl SimpleState for BlackState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        init_camera(world);

        let level = LevelData::try_from(LEVEL_ORDER[self.level_num % LEVEL_ORDER.len()]).unwrap();
        let (box_, progression) = level.init(world);
        self.box_ = Some(box_);
        self.progression = Some(progression);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData<'_, '_>>) -> SimpleTrans {
        let world = _data.world;

        let prog_storage = world.read_storage::<Progression>();
        if let Some(prog_entity) = self.progression {
            if let Some(progression) = prog_storage.get(prog_entity) {
                if progression.answer.len() >= progression.prompt.len() {
                    return Trans::Switch(Box::new(BlackState::from(self.level_num + 1)));
                }
            }
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        if let Some(box_) = self.box_ {
            if data.world.delete_entity(box_).is_ok() {
                self.box_ = None
            }
        }

        if let Some(progression) = self.progression {
            if data.world.delete_entity(progression).is_ok() {
                self.progression = None
            }
        }

        let buttons = (&data.world.entities(), &data.world.read_storage::<Button>())
            .join()
            .map(|(entity, _button)| entity)
            .collect::<Vec<Entity>>();
        data.world
            .delete_entities(&buttons)
            .expect("failed to delete all buttons");

        let prog_pieces = (
            &data.world.entities(),
            &data.world.read_storage::<ProgressionPiece>(),
        )
            .join()
            .map(|(entity, _prog_piece)| entity)
            .collect::<Vec<Entity>>();
        data.world
            .delete_entities(&prog_pieces)
            .expect("failed to delete all prog_pieces");

        let box_readers = (
            &data.world.entities(),
            &data.world.read_storage::<BoxReader>(),
        )
            .join()
            .map(|(entity, _box_reader)| entity)
            .collect::<Vec<Entity>>();
        data.world
            .delete_entities(&box_readers)
            .expect("failed to delete all box_readers");
    }
}

pub const CAM_RES_X: f32 = 426.;
pub const CAM_RES_Y: f32 = 240.;

fn init_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(CAM_RES_X / 2., CAM_RES_Y / 2., 1.);

    world
        .create_entity()
        .with(Camera::standard_2d(CAM_RES_X, CAM_RES_Y))
        .with(transform)
        .build();
}
