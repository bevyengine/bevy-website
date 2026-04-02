+++
title = "Apps"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Every Bevy program can be referred to as an [`App`]. The simplest Bevy app looks like this:

{{ file_code_block(file="quick-start/getting_started_v1.rs", anchor="basic_app") }}

The `use bevy::prelude::*` statement brings in the essential things from Bevy. For brevity, this guide may omit it in later steps.

Nice and simple right? Copy the code above into your `main.rs` file, then run `cargo run` in the command line while in your project folder.

You will notice that... nothing happens. This is because we haven't told our app to do anything yet! Apps are just empty shells capable of running our application logic.

## What Makes An App?

So, what sort of data does our [`App`] _actually_ store? An `App` contains our `World`, and our `World` contains our game's data. An `App` also contains the logic for controlling the outer loop of our game, allowing us to orchestrate the data in our `World` into the gameplay we want to create.

Generally, you'll be building your game at a more granular level than this. `App` is typically only used to setup the structure of your game, which is done by chaining its methods with the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html). Using these `App` methods, you'll be able to add systems, insert unique resources, and create the entities and components needed for your gameplay.

[`App`] provides us with tools for:

  1. Initializing resources in the [`World`] to store globally available data that we only need a single copy of.
  2. Adding systems to our [`Schedule`], which can read and modify resources and our entities' components, according to our game logic.
  3. Importing other blocks of [`App`]-modifying code using [`Plugins`].

We will cover these a bit more in the next pages. Speaking of which, let's finally add some logic to our App using Bevy ECS!

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`Plugins`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
