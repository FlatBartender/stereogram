use crate::states::game::*;

use amethyst::{
    core::transform::Transform,
    ecs::prelude::*,
    input::*,
};
use winit::MouseButton;

struct CursorMoveSystem;

impl<'s> System<'s> for CursorMoveSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, Cursor>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, cursors, input): Self::SystemData) {
        for (cursor, transform) in (&cursors, &mut transforms).join() {
            if let Some((x, y)) = input.mouse_position() {
                transform.prepend_translation_x(x);
                transform.prepend_translation_y(y);
            }
        }
    }
}

pub struct TargetClickSystem;

impl<'s> System<'s> for TargetClickSystem {
    type SystemData = (
        ReadStorage<'s, Cursor>,
        ReadStorage<'s, Transform>,
        WriteStorage<'s, Target>,
        WriteStorage<'s, Stereogram>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (cursors, transforms, mut targets, mut stereograms, input): Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left) {
            for (_, cursor_transform) in (&cursors, &transforms).join() {
                let cursor_pos = cursor_transform.translation();
                for (target, target_transform) in (&mut targets, &transforms).join() {
                let target_pos = target_transform.translation();
                if cursor_pos.x > target_pos.x &&
                    cursor_pos.y > target_pos.y &&
                    cursor_pos.x < target_pos.x + target.width.into() &&
                    cursor_pos.y < target_pos.y + target.height.into() {
                        target.clicked = true;
                    }
                }
            }
        }
    }
}

//pub struct TargetRemoveSystem;
//
//impl<'s> System<'s> for TargetRemoveSystem {
//
//}
