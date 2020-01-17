use amethyst::{
    core::{transform::components::Parent, Transform},
    ecs::{
        Join, Read, ReadStorage, System, WriteStorage,
    },
    input::{InputHandler, StringBindings},
};

use crate::{
    CurrentState,
    Game,
    states::player_turn::{Player, Tank, TankGun},
};

const AIM_SCALER: f32 = 0.02;

pub struct AimSystem;

impl<'s> System<'s> for AimSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        Read<'s, Game>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, TankGun>,
        WriteStorage<'s, Tank>,
        WriteStorage<'s, Transform>,
    );

    fn run(
        &mut self,
        (
            input,
            game,
            parents,
            players,
            tank_guns,
            mut tanks,
            mut transforms,
        ): Self::SystemData,
    ) {
        match game.current_state {
            CurrentState::PlayerTurn => {
                // Update the gun angle for any tanks that have a Player component.
                // It should really just be a single player tank at a time.
                // Should probably do something with the player ID and current turn or something.
                for (_, tank) in (&players, &mut tanks).join() {
                    if let Some(angle_delta) = input.axis_value("gun_angle") {
                        if angle_delta != 0.0 {
                            // Keep the gun angle between 0 and 90 degrees.
                            tank.gun_angle = (tank.gun_angle + angle_delta * AIM_SCALER)
                                .min((90.0 as f32).to_radians())
                                .max((0.0 as f32).to_radians());
                        }
                    }
                }

                // Update the transform of the tank guns.
                for (parent, _, transform) in (&parents, &tank_guns, &mut transforms).join() {
                    let parent_tank = tanks
                        .get(parent.entity)
                        .expect("TankGun did not have parent Tank");

                    transform.set_rotation_2d(parent_tank.gun_angle);
                }
            }
            _ => {}
        }
    }
}
