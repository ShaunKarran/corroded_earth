use amethyst::{
    core::Transform,
    ecs::{Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
};

use crate::states::game::{CurrentState, GameResource, Tank, TankGun};

const AIM_SCALER: f32 = 0.02;

pub struct AimSystem;

impl<'s> System<'s> for AimSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadExpect<'s, GameResource>,
        ReadStorage<'s, Tank>,
        WriteStorage<'s, TankGun>,
        WriteStorage<'s, Transform>,
    );

    fn run(&mut self, (input, game_data, tanks, mut tank_guns, mut transforms): Self::SystemData) {
        match game_data.current_state {
            CurrentState::PlayerTurn => {
                // If the user has pressed the controls to change the gun angle, update the gun angle.
                if let Some(angle_delta) = input.axis_value("gun_angle") {
                    if angle_delta != 0.0 {
                        let tank = tanks
                            .get(game_data.player)
                            .expect("failed to get Tank for player");
                        let tank_gun = tank_guns
                            .get_mut(tank.gun)
                            .expect("failed to get TankGun for player");
                        tank_gun.angle = (tank_gun.angle + angle_delta * AIM_SCALER)
                            .min((90.0 as f32).to_radians()) // Keep the gun angle between 0 and 90 degrees.
                            .max((0.0 as f32).to_radians());

                        let transform = transforms
                            .get_mut(tank.gun)
                            .expect("failed to get Transform for player gun");
                        transform.set_rotation_2d(tank_gun.angle);
                    }
                }
            }
            _ => {}
        }
    }
}
