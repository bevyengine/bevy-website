+++
title = "Apps"
insert_anchor_links = "right"
aliases = ["learn/book/getting-started/apps"]
[extra]
weight = 2
+++

Every Bevy program can be referred to as an [`App`]. The simplest Bevy app looks like this:

{{ file_code_block(file="quick-start/getting_started_v1.rs", anchor="basic_app") }}

The `use bevy::prelude::*` statement brings in the essential things from Bevy. For brevity, this guide may omit it in later steps.

Nice and simple right? Copy the code above into your `main.rs` file, then run:

```sh
cargo run
```

in your project folder. You will notice that... nothing happens. This is because we haven't told our app to do anything yet! Apps are just empty shells capable of running our application logic. If you'd like to learn more, then continue reading. Otherwise you can head to the next page to learn how to add logic to our App!

## What Makes an App?

So, what sort of data does our [`App`] really store? Looking at the docs linked, we find three fields: `world`, `schedule`, and `runner`. The `world` field stores all of our game's data, the `schedule` holds the systems that operate on this data (and the order in which they do so) and the `runner` interprets the schedule to control the broad execution strategy. You can read more about these by exploring the reference documentation linked just above.

Generally, you'll be operating at a more granular level than these basic primitives: controlling data in terms of specific resources or components and adding systems to an existing schedule. To do so, customize your own [`App`] by chaining its methods with the [builder pattern](https://doc.rust-lang.org/1.0.0/style/ownership/builders.html).
The most basic tools are:

  1. Initializing resources in the [`World`] to store globally available data that we only need a single copy of.
  2. Adding systems to our [`Schedule`], which can read and modify resources and our entities' components, according to our game logic.
  3. Importing other blocks of [`App`]-modifying code using [`Plugins`].

We will cover these a bit more in the next pages. Speaking of which, let's finally add some logic to our App using Bevy ECS!

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`Plugins`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
