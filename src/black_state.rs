use amethyst::{
    core::{math::Vector3, transform::Transform},
    ecs::{Entity, World},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{
        light::{Light, PointLight},
        palette::rgb::Rgb,
        Camera,
    },
};

use crate::{
    components::Progression,
    level::{LevelData, LEVEL_ORDER},
};

use std::convert::TryFrom;

pub struct BlackState {
    level_num: usize,
    level_data: Option<LevelData>,
    progress: Option<Entity>,
}

impl From<usize> for BlackState {
    fn from(level_num: usize) -> BlackState {
        BlackState {
            level_num,
            level_data: None,
            progress: None,
            // This sort of thing would probably be handled better by resources, but I'm planning
            // on having many boxes in one scene so this is a temporary solution
        }
    }
}

impl SimpleState for BlackState {
    fn on_start(&mut self, data: StateData<'_, GameData>) {
        log::debug!("Starting level...");
        let world = data.world;

        if (&world.read_storage::<Camera>(),).join().count() == 0 {
            init_camera_and_light(world);
        }

        let mut level_data =
            LevelData::try_from(LEVEL_ORDER[(self.level_num - 1) % LEVEL_ORDER.len()]).unwrap();

        self.progress = Some(level_data.init(world, self.level_num));
        self.level_data = Some(level_data);
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        Trans::None
    }

    fn fixed_update(&mut self, _data: StateData<'_, GameData>) -> SimpleTrans {
        let world = _data.world;

        let prog_storage = world.read_storage::<Progression>();
        if let Some(prog_entity) = self.progress {
            if let Some(progression) = prog_storage.get(prog_entity) {
                if progression.answer.len() >= progression.prompt.len() {
                    return Trans::Switch(Box::new(BlackState::from(self.level_num + 1)));
                }
            }
        }

        Trans::None
    }

    fn on_stop(&mut self, data: StateData<'_, GameData>) {
        if let Some(level_data) = &self.level_data {
            for entity in &level_data.entities {
                data.world
                    .delete_entity(*entity)
                    .expect("Failed to delete entity");
            }
        }
    }
}

pub const CAM_RES_X: f32 = 426.;
pub const CAM_RES_Y: f32 = 240.;

fn init_camera_and_light(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation_xyz(0., 1., 0.7);
    transform.face_towards(Vector3::new(0.0, 0.0, -0.5), Vector3::new(0.0, 1.0, 0.));

    world
        .create_entity()
        .with(Camera::standard_3d(CAM_RES_X, CAM_RES_Y))
        .with(transform)
        .build();

    let light: Light = PointLight {
        intensity: 5.0,
        color: Rgb::new(1.0, 0.8, 0.8),
        ..PointLight::default()
    }
    .into();
    let mut transform = Transform::default();
    transform.set_translation_xyz(-1., 1., 1.);

    world.create_entity().with(light).with(transform).build();
}
