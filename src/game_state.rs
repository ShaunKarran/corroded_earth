use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    core::transform::Transform,
    ecs::prelude::{Component, DenseVecStorage},
    input::{get_key, is_close_requested, is_key_down, VirtualKeyCode},
    prelude::*,
    renderer::{Camera, ImageFormat, SpriteRender, SpriteSheet, SpriteSheetFormat, Texture},
};

use log::info;

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

pub struct GameState {
    pub number_of_enemies: u8,
}

impl SimpleState for GameState {
    // On start will run when this state is initialized. For more
    // state lifecycle hooks, see:
    // https://book.amethyst.rs/stable/concepts/state.html#life-cycle
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // This is needed while we have no Systems using the Tank Component.
        world.register::<Tank>();

        // // Get the screen dimensions so we can initialize the camera and
        // // place our sprites correctly later. We'll clone this since we'll
        // // pass the world mutably to the following functions.
        // let dimensions = world.read_resource::<ScreenDimensions>().clone();

        // Place the camera
        init_camera(world);

        // Load our sprites and display them
        // let sprites = load_sprites(world);
        let sheet_handle = load_sprites(world);
        // init_sprites(world, &sprites, &dimensions);
        init_tank(world, sheet_handle.clone());
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            // Check if the window should be closed
            if is_close_requested(&event) || is_key_down(&event, VirtualKeyCode::Escape) {
                return Trans::Quit;
            }

            // Listen to any key events
            if let Some(event) = get_key(&event) {
                info!("handling key event: {:?}", event);
            }

            // If you're looking for a more sophisticated event handling solution,
            // including key bindings and gamepad support, please have a look at
            // https://book.amethyst.rs/stable/pong-tutorial/pong-tutorial-03.html#capturing-user-input
        }

        // Keep going
        Trans::None
    }
}

pub struct Tank {
    gun_angle: f32,
}

impl Tank {
    fn new(gun_angle: f32) -> Tank {
        Tank { gun_angle }
    }
}

impl Component for Tank {
    type Storage = DenseVecStorage<Self>;
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

// fn load_sprites(world: &mut World) -> Vec<SpriteRender> {
fn load_sprites(world: &mut World) -> Handle<SpriteSheet> {
    // Load the texture for our sprites. We'll later need to
    // add a handle to this texture to our `SpriteRender`s, so
    // we need to keep a reference to it.
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
    let sheet_handle = {
        let loader = world.read_resource::<Loader>();
        let sheet_storage = world.read_resource::<AssetStorage<SpriteSheet>>();
        loader.load(
            "sprites/tank.ron",
            SpriteSheetFormat(texture_handle),
            (),
            &sheet_storage,
        )
    };

    sheet_handle
}

fn init_tank(world: &mut World, sheet_handle: Handle<SpriteSheet>) {
    let mut transform = Transform::default();

    // Position the tank in a fixed location for now. 10 units left of centre.
    transform.set_translation_xyz(GAME_WIDTH / 2.0 - 10.0, GROUND_HEIGHT + TANK_HEIGHT / 2.0, 0.0);

    // Assign the sprite for the tank.
    let sprite_render = SpriteRender {
        sprite_sheet: sheet_handle,
        sprite_number: 0, // tank is the first (currently only) sprite in the sprite_sheet.
    };

    // Create a tank entity.
    world
        .create_entity()
        .with(sprite_render.clone())
        .with(Tank::new(45.0)) // Gun angle starts at 45 degrees.
        .with(transform)
        .build();
}
