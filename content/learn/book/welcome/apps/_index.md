+++
title = "Apps"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
+++

Bevy programs store and execute all of their game logic and data with a single [`App`] data structure.

Let's make a trivial Hello World app to demonstrate how that works in practice.
The process is straightforward: we first create a new [`App`].
Then, we add a simple system, which prints "Hello, Bevy!" when it is run.
Finally once we're done configuring the app, we call [`App`] to actually make our app *do things*.

```no_run,rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(hello)
        .run();
}

fn hello() {
    println!("Hello, Bevy!")
}
```

## What makes an App?

So, what sort of data does our  [`App`] really store?
Looking at the docs linked, we find three fields: `world`, `schedule` and `runner`.
The `world` field stores all of our game's data, the `schedule` holds the systems that operate on this data (and the order in which they do so) and the `runner` interprets the schedule to control the broad execution strategy.
You can read more about these by exploring the reference documentation linked just above.

Generally, you'll be operating at a more granular level than these basic primitives: controlling data in terms of specific resources or components and adding systems to an existing schedule.
To do so, customize your own [`App`] by chaining its methods with the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
The most basic tools are:

  1. Initializing resources in the [`World`] to store globally available data that we only need a single copy of.
  2. Adding systems to our [`Schedule`], which can read and modify resources and our entities' components, according to our game logic.
  3. Importing other blocks of [`App`]-modifying code using [`Plugins`].
Let's write a very simple demo that shows how those work.

```no_run,rust
use bevy::prelude::*;

fn main() {
    App::new()
        // Plugins are App code that was written elsewhere,
        // imported as a single unit for organization and clarity
        .add_plugins(MinimalPlugins)
        // Resources are global singleton data stored in the `World`
        .insert_resource(Message {string: "Welcome to Bevy!".to_string()})
        // Systems run every pass of the game loop and perform logic
        .add_system(read_message_system)
        .run();
}

// This resource can store a string
struct Message {
    string: String,
}

// This system reads our Message resource,
// so we add `Res<Message>` to its function parameters
fn read_message_system(message: Res<Message>) {
    println!("{}", message.string);
}
```

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`Plugins`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
