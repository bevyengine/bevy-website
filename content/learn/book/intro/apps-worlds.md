+++
title = "Apps and Worlds"
insert_anchor_links = "right"
[extra]
weight = 5
status = 'hidden'
+++

The last core concepts are higher level: **Apps** and **Worlds**

## The World

Everything we've talked about so far (entities, components, systems, resources, queries, and commands) exist within the context of a world.
The world contains all the data that's in your game or application.
In the database model, the world *is* the database.

Entities, components, and resources are stored inside the world, and systems, queries, and commands are executed *on* a world.
It is possible to have multiple worlds, and it's also possible to run a world completely headlessly, but the most common case by far is to run a single world inside of a single **app**.

## The App

Bevy provides a modular multi-threaded runtime called an [`App`](../../the-game-loop/app). If you have used web servers before, the basic ideas of an app will probably be familiar: you configure your app with settings and logic, then `run()` it to enter an update loop. It tends to look something like this:

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
It also handles other app-level config, like windowing settings.

## Off you go now

With that, you should have everything you need to start [exploring our examples](https://github.com/bevyengine/bevy/tree/latest/examples#examples),
or diving into the rest of the book.

The remaining chapters of this book are non-linear: you can read or referenced them in any order.
