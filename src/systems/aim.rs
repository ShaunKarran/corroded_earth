use amethyst::{
    core::Transform,
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::game_state::{TankGun};

// Used to scale the value from the input axis to something that moves the gun at a reasonable speed.
const AIM_SPEED_FACTOR: f32 = 0.02;

pub struct AimSystem;

impl<'s> System<'s> for AimSystem {
    type SystemData = (
        WriteStorage<'s, Transform>,
        ReadStorage<'s, TankGun>,
        Read<'s, InputHandler<StringBindings>>,
    );

    fn run(&mut self, (mut transforms, tank_guns, input): Self::SystemData) {
        for (_, transform) in (&tank_guns, &mut transforms).join() {
            let axis_value = input.axis_value("gun_angle");

            if let Some(angle_delta) = axis_value {
                if angle_delta != 0.0 {
                    let scaled_delta = angle_delta * AIM_SPEED_FACTOR;
                    let angle = transform.rotation().angle();

                    transform.set_rotation_2d(
                        (angle + scaled_delta)
                            // Limit the range of the gun between straight up and horizontal.
                            .min((90.0 as f32).to_radians())
                            .max((0.0 as f32).to_radians())
                    );
                }
            }
        }
    }
}
