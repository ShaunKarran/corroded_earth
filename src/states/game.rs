use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::{components::Parent, Transform},
    ecs::{Component, DenseVecStorage, Entity, LazyUpdate},
    input::{is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

// The height and width of the play space of the game.
// Set to match the resolution of a Nokia 5110 display. Conveniently this very close to a 16:9 ratio.
pub const GAME_HEIGHT: f32 = 48.0;
pub const GAME_WIDTH: f32 = 84.0;
// The vertical position of the "ground".
// For now the number of pixels from the bottom of the screen.
pub const GROUND_HEIGHT: f32 = 5.0;
// The height of the tank sprite. Used for positioning because the origin of the sprite is the centre.
pub const TANK_HEIGHT: f32 = 5.0;
// pub const TANK_WIDTH: f32 = 5.0; // Unused for now, might be useful later.
const AIM_SCALER: f32 = 0.02;

/// This is used to determine what type of things should be happening at any given time.
/// It should probably be split into different actual State objects, but I will do that
/// after I get it working with just GameState.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CurrentState {
    AITurn,
    BulletInFlight,
    PlayerTurn,
}

impl Default for CurrentState {
    fn default() -> Self {
        CurrentState::PlayerTurn
    }
}

pub struct GameResource {
    pub current_state: CurrentState,
    pub player: Entity,
    pub ai: Entity,
}

pub struct GameState;

impl SimpleState for GameState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // These are needed because there are no systems that use them.
        // Components used by systems are automatically registered.
        world.register::<Tank>();
        world.register::<TankGun>();

        init_camera(world);

        let sheet_handle = load_sprites(world);
        world.insert(sheet_handle.clone());

        let player = init_tank(world, sheet_handle.clone(), 30.0, 45.0);
        let ai = init_tank(world, sheet_handle.clone(), -30.0, 135.0);

        world.insert(GameResource {
            current_state: CurrentState::default(),
            player,
            ai,
        });
    }

    fn handle_event(
        &mut self,
        data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }
        }

        let mut game_data = data.world.write_resource::<GameResource>();

        match game_data.current_state {
            CurrentState::PlayerTurn => {
                if let StateEvent::Window(event) = &event {
                    // Listen to any key events
                    if is_key_down(&event, VirtualKeyCode::Space) {
                        // Use the transform of the player as the initial transform of the bullet.
                        let mut bullet_transform = data
                            .world
                            .read_storage::<Transform>()
                            .get(game_data.player)
                            .expect("failed to get transform for player")
                            .clone();

                        let tanks = data.world.read_storage::<Tank>();
                        let tank = tanks
                            .get(game_data.player)
                            .expect("failed to get tank for player");
                        let tank_guns = data.world.read_storage::<TankGun>();
                        let tank_gun = tank_guns
                            .get(tank.gun)
                            .expect("failed to get tank gun for player");

                        // Update the bullets position match the end of the gun barrel.
                        let gun_x_component = 5.0 * tank_gun.angle.cos();
                        let gun_y_component = 5.0 * tank_gun.angle.sin();
                        bullet_transform.prepend_translation_x(gun_x_component);
                        bullet_transform.prepend_translation_y(gun_y_component);

                        // Assign the sprite for the bullet.
                        let bullet_sprite_render = SpriteRender {
                            sprite_sheet: (*data.world.read_resource::<Handle<SpriteSheet>>())
                                .clone(),
                            sprite_number: 2,
                        };

                        // Create the bullet.
                        let lazy_update = data.world.read_resource::<LazyUpdate>();
                        let entities = data.world.entities();
                        lazy_update
                            .create_entity(&entities)
                            .with(bullet_sprite_render.clone())
                            .with(bullet_transform)
                            .with(TankBullet {
                                velocity: [gun_x_component * 10.0, gun_y_component * 10.0],
                            })
                            .build();

                        game_data.current_state = CurrentState::AITurn;
                    }
                }
            }
            CurrentState::AITurn => {
                if let StateEvent::Window(event) = &event {
                    if is_key_down(&event, VirtualKeyCode::Space) {
                        game_data.current_state = CurrentState::PlayerTurn;
                    }
                }
            }
            CurrentState::BulletInFlight => {}
        }

        // Keep going
        Trans::None
    }
}

#[derive(Component, Debug)]
pub struct Tank {
    pub gun: Entity,
}

#[derive(Component, Debug)]
pub struct TankGun {
    // Angle from horizontal of the tanks gun in radians.
    pub angle: f32,
}

#[derive(Component, Debug)]
pub struct TankBullet {
    pub velocity: [f32; 2],
}

fn init_camera(world: &mut World) {
    // Center the camera in the middle of the screen, and let it cover the entire screen.
    let mut transform = Transform::default();
    transform.set_translation_xyz(GAME_WIDTH / 2.0, GAME_HEIGHT / 2.0, 1.0);

    world
        .create_entity()
        .with(Camera::standard_2d(GAME_WIDTH, GAME_HEIGHT))
        .with(transform)
        .build();
}

fn load_sprites(world: &mut World) -> Handle<SpriteSheet> {
    // Load the texture for our sprites.
    // We'll later need to add a handle to this texture to our `SpriteRender`s, so we need to keep a reference to it.
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            "sprites/tank.png",
            ImageFormat::default(),
            (),
            &texture_storage,
        )
    };

    // Load the sprite sheet definition file, which contains metadata on our sprite sheet texture.
    let loader = world.read_resource::<Loader>();
    let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        "sprites/tank.ron",
        SpriteSheetFormat(texture_handle),
        (),
        &sheet_storage,
    )
}

fn init_tank(
    world: &mut World,
    sheet_handle: Handle<SpriteSheet>,
    x_pos: f32,
    gun_angle: f32,
) -> Entity {
    let gun_angle_radians = gun_angle.to_radians(); // Rotate the gun by 45 degrees by default.

    // The tank gun will have the tank as a parent which means the tank gun's transform is relative to the tank.
    // This means we need a new transform.
    let mut gun_transform = Transform::default();
    gun_transform.set_rotation_2d(gun_angle_radians);

    // Assign the sprite for the tank gun.
    let gun_sprite_render = SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 1, // tank gun is the second sprite in the sprite_sheet.
    };

    // Create a TankGun entity.
    let gun = world
        .create_entity()
        .with(TankGun {
            angle: gun_angle_radians,
        })
        .with(gun_sprite_render.clone())
        .with(gun_transform)
        .build();

    // Position the tank in a fixed location for now. x_pos units from the centre.
    let mut tank_transform = Transform::default();
    tank_transform.set_translation_xyz(
        GAME_WIDTH / 2.0 - x_pos,
        GROUND_HEIGHT + TANK_HEIGHT / 2.0,
        0.0,
    );

    // Assign the sprite for the tank.
    let tank_sprite_render = SpriteRender {
        sprite_sheet: sheet_handle.clone(),
        sprite_number: 0, // tank is the first sprite in the sprite_sheet.
    };

    // Create a tank entity.
    let tank_entity = world
        .create_entity()
        .with(Tank { gun })
        .with(tank_sprite_render.clone())
        .with(tank_transform)
        .build();

    // Add the Parent component to the gun so that it will inherit transformations from the tank.
    let mut parents = world.write_storage::<Parent>();
    parents
        .insert(
            gun,
            Parent {
                entity: tank_entity,
            },
        )
        .expect("failed to add Parent component to tank gun");

    tank_entity
}
