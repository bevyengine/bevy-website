+++
title = "First Steps"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

All Bevy games start with an {{rust_type(type="struct", crate="bevy_app", name="App", no_mod=true)}}, and most will utilize the default plugins, which we include with `add_default_plugins()`.

```rs
use bevy::prelude::*;

fn main() {
    App::build()
        .add_default_plugins()
        .run();
}
```

## Startup system

To populate our world with entities, we will use a startup system:

```rs
fn setup(
    mut commands: Commands,
) {
    //TODO: Add all the things
}
```
To tell Bevy to run this function at startup, we add it to our `App::build()` chain:
```rs
App::build()
    .add_default_plugins()
    .add_startup_system(setup.system())
    .run();
```

Before we can draw any sprites, we have to add our camera to the scene. We can do this using our startup system.

```rs
fn setup(
    mut commands: Commands,
) {
    commands
        .spawn(Camera2dComponents::default())
}
```

Let's also change the `ClearColor` of our window by adding a resource.

First, change the use statement to include the `ClearColor` struct.
```rs
use bevy::{
    prelude::*,
    render::pass::ClearColor,
};
```
Now we can add it in as a resource. The systems responsible for rendering your game read in this resource and change the background color of the game.
```rs
App::build()
    .add_resource(ClearColor(Color::rgb(0.7, 0.7, 0.7)))
    /* Cut for brevity */
```