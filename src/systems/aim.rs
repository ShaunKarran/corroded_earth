use amethyst::{
    assets::Handle,
    core::{transform::components::Parent, Transform},
    ecs::{Builder, Entities, Join, LazyUpdate, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, SpriteSheet},
};

use crate::game_state::{Player, Tank, TankBullet, TankGun};

const AIM_SCALER: f32 = 0.02;

pub struct AimSystem;

impl<'s> System<'s> for AimSystem {
    type SystemData = (
        Read<'s, InputHandler<StringBindings>>,
        ReadStorage<'s, Parent>,
        ReadStorage<'s, Player>,
        ReadStorage<'s, TankGun>,
        WriteStorage<'s, Tank>,
        WriteStorage<'s, Transform>,
        Entities<'s>,
        Read<'s, LazyUpdate>,
        ReadExpect<'s, Handle<SpriteSheet>>,
    );

    fn run(
        &mut self,
        (
            input,
            parents,
            players,
            tank_guns,
            mut tanks,
            mut transforms,
            entities,
            lazy_update,
            sheet_handle,
        ): Self::SystemData,
    ) {
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

        if input.action_is_down("shoot").unwrap_or(false) {
            // Store the x and y components of the guns current position so we can use them to
            // calculate the bullets starting position and velocity based on gun angle.
            // Default values don't really make sense, but the logic doesn't guarantee
            // they will be assigned. Should probably fix this.
            let gun_x_component: f32 = 5.0;
            let gun_y_component: f32 = 5.0;
            let mut bullet_transform = Transform::default();

            for (_, tank, transform) in (&players, &mut tanks, &mut transforms).join() {
                bullet_transform = transform.clone();

                // Make the bullets initial position match the end of the gun barrel.
                let gun_x_component = 5.0 * tank.gun_angle.cos();
                let gun_y_component = 5.0 * tank.gun_angle.sin();
                bullet_transform.prepend_translation_x(gun_x_component);
                bullet_transform.prepend_translation_y(gun_y_component);
            }

            //Assign the sprite for the tank gun.
            let bullet_sprite_render = SpriteRender {
                sprite_sheet: sheet_handle.clone(),
                sprite_number: 1, // tank gun is the second sprite in the sprite_sheet.
            };

            // Create the bullet.
            lazy_update
                .create_entity(&entities)
                .with(bullet_sprite_render.clone())
                .with(bullet_transform)
                .with(
                    TankBullet {
                        velocity: [gun_x_component * 10.0, gun_y_component * 10.0],
                    }
                )
                .build();
        }
    }
}
