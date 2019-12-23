use amethyst::{
    assets::Handle,
    core::{transform::components::Parent, Transform},
    ecs::{Entities, Join, Read, ReadExpect, ReadStorage, System, WriteStorage},
    input::{InputHandler, StringBindings},
    renderer::{SpriteRender, SpriteSheet},
};

use crate::game_state::{Player, Tank, TankBullet, TankGun};

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
        ReadExpect<'s, Handle<SpriteSheet>>,
        WriteStorage<'s, SpriteRender>,
        WriteStorage<'s, TankBullet>,
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
            sheet_handle,
            mut sprite_renders,
            mut tank_bullets,
        ): Self::SystemData,
    ) {
        // Update the gun angle for any tanks that have a Player component.
        // It should really just be a single player tank at a time.
        // Should probably do something with the player ID and current turn or something.
        for (_, tank) in (&players, &mut tanks).join() {
            if let Some(angle_delta) = input.axis_value("gun_angle") {
                if angle_delta != 0.0 {
                    // Keep the gun angle between 0 and 90 degrees.
                    tank.gun_angle = (tank.gun_angle + angle_delta).min(90.0).max(0.0);
                }
            }
        }

        // Update the transform of the tank guns.
        for (parent, _, transform) in (&parents, &tank_guns, &mut transforms).join() {
            let parent_tank = tanks
                .get(parent.entity)
                .expect("TankGun did not have parent Tank");

            transform.set_rotation_2d(parent_tank.gun_angle.to_radians());
        }

        if input.action_is_down("shoot").unwrap_or(false) {
            let mut bullet_transform = Transform::default();

            for (_, _, transform) in (&players, &mut tanks, &mut transforms).join() {
                bullet_transform = transform.clone();
            }

            //Assign the sprite for the tank gun.
            let bullet_sprite_render = SpriteRender {
                sprite_sheet: sheet_handle.clone(),
                sprite_number: 1, // tank gun is the second sprite in the sprite_sheet.
            };

            // Create the bullet.
            entities
                .build_entity()
                .with(bullet_sprite_render.clone(), &mut sprite_renders)
                .with(bullet_transform, &mut transforms)
                .with(
                    TankBullet {
                        velocity: [10.0, 10.0],
                    },
                    &mut tank_bullets,
                )
                .build();
        }
    }
}

// impl<'s> System<'s> for AimSystem {
//     type SystemData = (
//         WriteStorage<'s, Transform>,
//         ReadStorage<'s, TankGun>,
//         WriteStorage<'s, TankBullet>,
//         Read<'s, InputHandler<StringBindings>>,
//         Entities<'s>,
//         WriteStorage<'s, SpriteRender>,
//         ReadExpect<'s, Handle<SpriteSheet>>,
//     );

//     fn run(
//         &mut self,
//         (
//             mut transforms,
//             tank_guns,
//             mut tank_bullets,
//             input,
//             entities,
//             mut sprite_renders,
//             sheet_handle,
//         ): Self::SystemData,
//     ) {
//         // We want to only update the angle of tank guns in this loop
//         // So we iterate over (tank_guns, transforms).join() even though
//         // we don't actually use the tank_gun objects.
//         for (_, transform) in (&tank_guns, &mut transforms).join() {
//             let axis_value = input.axis_value("gun_angle");

//             if let Some(angle_delta) = axis_value {
//                 if angle_delta != 0.0 {
//                     let scaled_delta = angle_delta * AIM_SPEED_FACTOR;
//                     let angle = transform.rotation().angle();

//                     transform.set_rotation_2d(
//                         (angle + scaled_delta)
//                             // Limit the range of the gun between straight up and horizontal.
//                             .min((90.0 as f32).to_radians())
//                             .max((0.0 as f32).to_radians()),
//                     );
//                 }
//             }
//         }

//         // TODO: This probably shouldn't be handled in this system. Consider where it should be.
//         // Maybe could use an EventChannel and have a system that just listens to that channel
//         // and spawns things.
//         let mut bullet_transform = Transform::default();
//         if input.action_is_down("shoot").unwrap_or(false) {
//             // This currently doesn't work as you'd expect because the tank_guns position
//             // is (0, 0, 0) because they rely on the parent tank entities position.
//             for (_, transform) in (&tank_guns, &mut transforms).join() {
//                 bullet_transform = transform.clone();
//             }

//             // Assign the sprite for the tank gun.
//             let bullet_sprite_render = SpriteRender {
//                 sprite_sheet: sheet_handle.clone(),
//                 sprite_number: 1, // tank gun is the second sprite in the sprite_sheet.
//             };

//             // Create the bullet.
//             entities
//                 .build_entity()
//                 .with(bullet_sprite_render.clone(), &mut sprite_renders)
//                 .with(bullet_transform, &mut transforms)
//                 .with(TankBullet { velocity: [1.0, 1.0] }, &mut tank_bullets)
//                 .build();
//         }
//     }
// }
