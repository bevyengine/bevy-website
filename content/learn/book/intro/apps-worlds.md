+++
title = "Apps and Worlds"
insert_anchor_links = "right"
[extra]
weight = 5
status = 'hidden'
+++

The last core concepts every Bevy user needs to understand are used to organize our data.
A [`World`] stores our ECS data, while an [`App`] stores a [`World`] and controls the outer loop for how our game actually runs.

## Worlds

The world contains all the data that's in your game or application.
Everything we've talked about so far is either data that is *stored* in a [`World`] (entities, components, resources)
or operations that are *evaluated on* a [`World`] (systems, queries, commands).
In the database model, the world *is* the database.

It is possible to have multiple worlds, and it's also possible to have a world without an app, but the most common case by far is to run a single world inside of a single **app**.

## Apps

Bevy provides a modular multi-threaded runtime called an [`App`](@/learn/book/the-game-loop/app.md). If you have used web servers before, the basic ideas of an app will probably be familiar: you configure your app with settings and logic, then `run()` it to enter an update loop. It tends to look something like this:

```rust
use bevy::prelude::*;

fn main() {
    App::new()                          // Create a new app
        .add_systems(Update, my_system) // Configure its behavior
        .run();                         // Enter a run loop
}
```

In most cases, your world will be contained within your app.
The app is responsible for scheduling and executing your systems, and passing the data in and out of them appropriately.

Apps can also be configured using modular, reusable pieces, called [plugins](@/learn/book/modular-architecture/plugins.md):

```rust
App::new()
    .add_plugins((DefaultPlugins, PlayerPlugin, TreePlugin))
    .run();
```

## Next Steps

With that, you should have everything you need to start [exploring our examples](https://github.com/bevyengine/bevy/tree/latest/examples#examples),
or diving into the rest of the book.

The remaining chapters of this book are nonlinear: you can read or reference them in any order.

[`World`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html
[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
