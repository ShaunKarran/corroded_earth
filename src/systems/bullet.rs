use amethyst::{
    core::{timing::Time, Transform},
    ecs::{Join, Read, System, WriteStorage},
};

use crate::states::player_turn::TankBullet;

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
            // Update the bullets velocity due to gravity (and later add wind).
            bullet.velocity[1] += GRAVITY;

            // Update the bullets position based on it's velocity.
            transform.prepend_translation_x(bullet.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(bullet.velocity[1] * time.delta_seconds());
        }
    }
}
