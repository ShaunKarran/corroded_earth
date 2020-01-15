use amethyst::prelude::*;

pub struct AITurnState;

impl SimpleState for AITurnState {
    fn handle_event(
        &mut self,
        mut _data: StateData<'_, GameData<'_, '_>>,
        _: StateEvent,
    ) -> SimpleTrans {
        Trans::None
    }
}
