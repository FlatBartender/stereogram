use rand::prelude::*;
use crate::stereogram::*;
use amethyst::{
    assets::{AssetStorage, Loader, Handle},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
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
    use crate::{WIDTH, HEIGHT, TARGET_WIDTH, TARGET_HEIGHT};

    let mut rng = thread_rng();

    let x: f32 = rng.gen_range(TARGET_WIDTH, WIDTH - TARGET_WIDTH);
    let y: f32 = rng.gen_range(TARGET_HEIGHT, HEIGHT - TARGET_HEIGHT);

    let mut transform = Transform::default();

    transform.set_translation_xyz(TARGET_WIDTH * 0.5 + x, TARGET_HEIGHT * 0.5 + y, 0.0);

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
}

impl Target {
    fn new() -> Target {
        use crate::{TARGET_WIDTH, TARGET_HEIGHT};
        Target {
            width: TARGET_WIDTH,
            height: TARGET_HEIGHT,
        }
    }
}

impl Component for Target {
    type Storage = DenseVecStorage<Self>;
}

pub struct TargetClickSystem;

impl<'s> System<'s> for TargetClickedSystem {
    type SystemData = (
        ReadStorage<'s, Cursor>,
        ReadStorage<'s, Target>,
        WriteStorage<'s, Transform>,
        WriteStorage<'s, Stereogram>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (cursors, targets, mut transforms, mut stereograms, input): Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left) {
            let mut target_clicked = false;
            for ((_, cursor_transform), (target, target_transform)) in ((&cursors, &transforms).join(), (&targets, &transforms).join()) {
                if cursor_transform.x() > target_transform.x() &&
                    cursor_transform.y() > target_transform.y() &&
                    cursor_transform.x() < target_transform.x() + target.width &&
                    cursor_transform.y() < target_transform.y() + target.height && {
                    
                    target_clicked = true;
                }
            }

            if target_clicked {
                
            }
        }
    }
}

pub struct Cursor;

impl Component for Cursor {
    type Storage = DenseVecStorage<Self>;
}

pub struct CursorMoveSystem;

impl<'s> System<'s> for CursorMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Cursor>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cursors, input): Self::Systemata) {
        for (cursor, transform) in (&cursors, &mut transforms).join() {
            if let Some((x, y)) = input.mouse_position() {
                transform.prepend_translation(Vector3::new(x, y, 0.0));
            }
        }
    }
}
