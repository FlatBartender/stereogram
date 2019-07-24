use rand::prelude::*;
use amethyst::{
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::Camera,
};

pub struct Game;

fn initialize_camera(world: &mut World) {
    use crate::{WIDTH, HEIGHT};

    let mut transform = Transform::default();
    transform.set_translation_xyz(WIDTH * 0.5, HEIGHT * 0.5, 1.0);

    world.create_entity()
        .with(Camera::standard_2d(WIDTH, HEIGHT))
        .with(transform)
        .build();
}

fn initialize_cursor(world: &mut World) {
    use crate::{WIDTH, HEIGHT, TARGET_WIDTH, TARGET_HEIGHT};

    let mut transform = Transform::default();
}

fn initialize_target(world: &mut World) {
    let target = Target::new();
    let transform = target.random_position();

    world.create_entity()
        .with(Target::new())
        .with(transform)
        .build();
}

impl SimpleState for Game {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        world.register::<Target>();

        initialize_camera(world);
        initialize_cursor(world);
        initialize_target(world);
    }
}

pub struct Target {
    pub width: f32,
    pub height: f32,
    pub clicked: bool,
}

impl Target {
    fn new() -> Target {
        use crate::{TARGET_WIDTH, TARGET_HEIGHT};
        Target {
            width: TARGET_WIDTH,
            height: TARGET_HEIGHT,
            clicked: false,
        }
    }

    fn random_position(&self) -> Transform {
        use crate::{WIDTH, HEIGHT, TARGET_WIDTH, TARGET_HEIGHT};

        let mut transform = Transform::default();
        let mut rng = thread_rng();

        let x: f32 = rng.gen_range(TARGET_WIDTH, WIDTH - TARGET_WIDTH);
        let y: f32 = rng.gen_range(TARGET_HEIGHT, HEIGHT - TARGET_HEIGHT);

        transform.set_translation_xyz(TARGET_WIDTH * 0.5 + x, TARGET_HEIGHT * 0.5 + y, 0.0);

        transform
    }
}

impl Component for Target {
    type Storage = DenseVecStorage<Self>;
}

pub struct Cursor;

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}

pub struct Stereogram;

impl Component for Stereogram {
    type Storage = DenseVecStorage<Self>;
}
