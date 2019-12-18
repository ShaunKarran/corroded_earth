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
