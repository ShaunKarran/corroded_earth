use amethyst::{
    input::{is_key_down, VirtualKeyCode},
    prelude::*,
};

use log::info;

use crate::{CurrentState, Game};

pub struct AITurnState;

impl SimpleState for AITurnState {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        // mark that the current state is a gameplay state.
        data.world.write_resource::<Game>().current_state = CurrentState::AITurn;
    }

    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        event: StateEvent,
    ) -> SimpleTrans {
        if let StateEvent::Window(event) = &event {
            if is_key_down(&event, VirtualKeyCode::Space) {
                info!("popping AITurnState state");
                return Trans::Pop;
            }
        }

        Trans::None
    }
}
