use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game_state::{TankGun};

pub struct AimSystem;

impl<'s> System<'s> for AimSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TankGun>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, tank_guns, input): Self::SystemData) {
        for (_, transform) in (&tank_guns, &mut transforms).join() {
            let movement = input.axis_value("gun_angle");
            if let Some(mv_amount) = movement {
                if mv_amount != 0.0 {
                    transform.rotate_2d(mv_amount * 0.02);
                }
            }
        }
    }
}
