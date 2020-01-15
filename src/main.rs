use amethyst::{
    core::transform::TransformBundle,
    input::{InputBundle, StringBindings},
    prelude::*,
    renderer::{
        plugins::{RenderFlat2D, RenderToWindow},
        types::DefaultBackend,
        RenderingBundle,
    },
    utils::application_root_dir,
};

mod states;
mod systems;

use states::PlayerTurnState;

#[derive(Default)]
pub struct Game {
    current_state: CurrentState,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum CurrentState {
    PlayerTurn,
    AITurn,
}

impl Default for CurrentState {
    fn default() -> Self { CurrentState::PlayerTurn }
}

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let app_root = application_root_dir()?;

    let resources = app_root.join("resources");
    let display_config = resources.join("display_config.ron");

    let configs = app_root.join("configs");
    let bindings = configs.join("bindings.ron");

    let input_bundle = InputBundle::<StringBindings>::new().with_bindings_from_file(bindings)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(systems::AimSystem, "aim_system", &["input_system"])
        .with(systems::BulletSystem, "bullet_system", &[])
        .with_bundle(
            RenderingBundle::<DefaultBackend>::new()
                .with_plugin(
                    RenderToWindow::from_config_path(display_config)
                        .with_clear([0.34, 0.36, 0.52, 1.0]),
                )
                .with_plugin(RenderFlat2D::default()),
        )?;

    let mut game = Application::new(resources, PlayerTurnState, game_data)?;
    game.run();

    Ok(())
}
