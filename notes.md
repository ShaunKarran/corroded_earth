# Game States

                            |-----------|-------> EndGame (Not yet) -> MainMenu (Not yet)
                            |           |
Main Menu (Not yet) -> PlayerTurn -> AITurn(s)
                            ^           |
                            |-----------|

## Systems based on state
I expected something to be able to activate/deactivate systems based on the current state. That doesn't seem to be the case.
Instead the docs (book -> Concepts -> System) shows an example of storing some data (a struct) as a resource in the game which contains an enum of the current state. Then in the systems you can switch on the current state to control the systems behaviour based on state.
https://book.amethyst.rs/stable/concepts/system.html#changing-states-through-resources

After making initial changes to switch states the result is that the state changes to AITurn before the system actually fires the bullet.
I think the solution might be handling the fire bullet input in the PlayerTurnState.handle_event method. I could record some 'PlayerAction' in the Game and then fire the bullet.
I could also have a state for while the bullet is in flight?
Maybe both.

I think I need to be more "event based". Not sure exactly what that looks like, but it doesn't make much sense to be handling of some input inside systems.
For aiming the tank gun, it does make sense. But not for firing.
Can I do stuff like create the bullet in the event? I'm pretty sure all the resources are available in both events and systems.

# Plan for handling Tanks.

- Have a Tank component.
    - The Tank component contains a reference to its gun entity.
        I'm not sure how this works yet, but an Entity is just an ID, so surely you can
        get an Entity's components in a system by its ID.

- TankGun component.
    - Not sure if this needs anything. It's really just an entity to hold the tank gun sprite,
        as well as the gun transform.
    - It will need the Tank as a Parent transform.

## Potential issues.
- The above Tank/TankGun plan requires the TankGun have the Tank entity as its Parent transform,
    but the Tank requires the TankGun entity. Is this possible? Will need to create both entities,
    then update both with the others entity ID.

## New idea
- Don't store the gun entity in the tank.
    Just store the gun angle.
    Have a system that iterates over tank guns, gets the parent entity's Tank component
    and updates its transform based on the angle.
    
    Need to include `ReadStorage<Tank>` in the system data so we can get the Tank component.
    From the Parent component do `.entity` to get the parent Entity.
    Then do `tanks.get(parent_entity).expect('TankGun did not have a parent Tank')`.
