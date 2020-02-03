use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, System, WriteStorage},
};

use crate::states::game::{TankBullet, GROUND_HEIGHT};

const GRAVITY: f32 = -9.81;

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        WriteStorage<'s, TankBullet>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (mut tank_bullets, mut transforms, time): Self::SystemData) {
        for (bullet, transform) in (&mut tank_bullets, &mut transforms).join() {
            // If the bullet has no velocity we don't need to do anything.
            // TODO: This should eventually be not needed because bullets should be removed on impact.
            if bullet.velocity[0] == 0.0 && bullet.velocity[1] == 0.0 {
                continue;
            }

            // Update the bullets velocity due to gravity (and later add wind).
            bullet.velocity[1] += GRAVITY;

            // Update the bullets position based on it's velocity, but not allowing values below the ground.
            transform.set_translation_x(
                (transform.translation().x + (bullet.velocity[0] * time.delta_seconds()))
                    .max(GROUND_HEIGHT),
            );
            transform.set_translation_y(
                (transform.translation().y + (bullet.velocity[1] * time.delta_seconds()))
                    .max(GROUND_HEIGHT),
            );

            // If the bullet has hit the ground it stops moving.
            if transform.translation().y <= GROUND_HEIGHT {
                bullet.velocity[0] = 0.0;
                bullet.velocity[1] = 0.0;
            }
        }
    }
}
