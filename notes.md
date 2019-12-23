# Spawning bullets
- Need to update how Im handling the sprite sheet to make it available to the systems.

Load the `Handle<SpriteSheet>` like I'm doing now, but need to insert it into the world
so that it is available as a resource.
```rs
world.insert(sheet_handle.clone());
```

Then get the sprite sheet in the system
```rs
type SystemData = (
    //...
    ReadExpect<'s, Handle<SpriteSheet>>, // Not sure if I can use this type, or wrap is some container.
    //...
);
```


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