+++
title = "Plugins and the game loop"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 2
status = 'hidden'
+++
Previously we had to call update manually, we are fixing it by introducing a 
#### Game loop

Let us consider following code
```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);

    app.run();
}
```

Running it spawns a window, but otherwise it's quite boring. Let's add a system to it.
```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Update, print_hello);

    app.run();
}

fn print_hello() {
    println!("Hello");
}
```
Running it spams our console with hello, that is because Update tries to run on every render frame.
We have switched from manually updating our app to letting the rendering update it per frame.

The default plugins include a Time Plugin, Window Plugin, TaskPool Plugin and a Render Plugin. The details of those are left for later chapters, but rest assured, with those sets of plugins we can now try to render to the screen and move objects.

To render something on the screen we will need a camera, so let's spawn a camera.
```rust
use bevy::prelude::*;

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins);
    app.add_systems(Startup, init_scene);

    app.run();
}

fn init_scene(mut commands: Commands) {
    commands.spawn(Camera2d);
}
```
So far, so good. We'll also add a blue triangle.
This will be part of the next chapter.
