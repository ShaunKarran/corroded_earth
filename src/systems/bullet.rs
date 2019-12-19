use amethyst::{
    core::{
        timing::Time,
        Transform,
    },
    ecs::{Join, Read, ReadStorage, System, WriteStorage},
};

use crate::game_state::TankBullet;

pub struct BulletSystem;

impl<'s> System<'s> for BulletSystem {
    type SystemData = (
        ReadStorage<'s, TankBullet>,
        WriteStorage<'s, Transform>,
        Read<'s, Time>,
    );

    fn run(&mut self, (tank_bullets, mut transforms, time): Self::SystemData) {
        for (bullet, transform) in (&tank_bullets, &mut transforms).join() {
            // Update the bullets velocity due to gravity (and later add wind).
            // TODO.

            // Update the bullets position based on it's velocity.
            transform.prepend_translation_x(bullet.velocity[0] * time.delta_seconds());
            transform.prepend_translation_y(bullet.velocity[1] * time.delta_seconds());
        }
    }
}
