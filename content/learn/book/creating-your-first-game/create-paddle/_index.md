+++
title = "Creating the Paddle"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Now that we have our startup system, we can add entities to our world.

## Custom component structs
First, we have to add a marker component struct for the paddle in order for our systems to run on specifically the paddle.

```rs
struct Paddle;
```
Now we can spawn a paddle using our startup system.
```rs
commands
    /* cut for brevity */
    .spawn(Paddle)
```
If you run the game now, you won't see anything, and that's because we haven't added in a `SpriteComponents` for our paddle. To do this, we can add a `with(SpriteComponents)` to our paddle.
```rs
commands
    /* cut for brevity */
    .spawn(Paddle)
    .with(SpriteComponents {
            material: materials.add(Color::rgb(0.2, 0.2, 0.8).into()),
            translation: Translation(Vec3::new(0.0, -215.0, 0.0)),
            sprite: Sprite {
                size: Vec2::new(120.0, 30.0),
            },
            ..Default::default()
        })
```