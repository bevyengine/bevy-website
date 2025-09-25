+++
title = "ECS"
insert_anchor_links = "right"
aliases = ["learn/book/getting-started/ecs"]
[extra]
weight = 3
+++

All app logic in Bevy uses the Entity Component System paradigm, which is often shortened to ECS. ECS is a software pattern that involves breaking your program up into **Entities**, **Components**, and **Systems**. **Entities** are unique "things" that are assigned groups of **Components**, which are then processed using **Systems**.

For example, one entity might have a `Position` and `Velocity` component, whereas another entity might have a `Position` and `UI` component. Systems are logic that runs on a specific set of component types. You might have a `movement` system that runs on all entities with a `Position` and `Velocity` component.

The ECS pattern encourages clean, decoupled designs by forcing you to break up your app data and logic into its core components. It also helps make your code faster by optimizing memory access patterns and making parallelism easier.

## Bevy ECS

Bevy ECS is Bevy's implementation of the ECS pattern. Unlike other Rust ECS implementations, which often require complex lifetimes, traits, builder patterns, or macros, Bevy ECS uses normal Rust datatypes for all of these concepts:

* **Components**: Rust structs that implement the [`Component`] trait

{{file_code_block(file="quick-start/position_ecs.rs", anchor="position_component")}}

* **Systems**: normal Rust functions

{{file_code_block(file="quick-start/position_ecs.rs", anchor="position_system")}}

* **Entities**: a simple type containing a unique integer

{{file_code_block(file="quick-start/mock_entity.rs", anchor="entity_struct")}}

Now let's see how this works in practice!

[`Component`]: https://docs.rs/bevy/latest/bevy/ecs/component/trait.Component.html

## Your First System

Paste the following function into your `main.rs` file:

{{file_code_block(file="quick-start/getting_started_v2.rs", anchor="hello_world")}}

This will be our first system. The only remaining step is to add it to our [`App`]!

{{file_code_block(file="quick-start/getting_started_v2.rs", anchor="app_main")}}

The [`add_systems`] function adds the system to your App's [`Update`] [`Schedule`], but we'll cover that more later.

Now run your app again using `cargo run`. You should see `hello world!` printed once in your terminal.

[`add_systems`]: https://docs.rs/bevy_app/latest/bevy_app/struct.App.html#method.add_systems
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`Schedule`]: https://docs.rs/bevy_ecs/latest/bevy_ecs/schedule/struct.Schedule.html
[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html

## Your First Components

Greeting the whole world is great, but what if we want to greet specific people? In ECS, you would generally model people as entities with a set of components that define them. Let's start simple with a `Person` component.

Add this struct to your `main.rs` file:

{{file_code_block(file="quick-start/getting_started_v3.rs", anchor="person_component")}}

But what if we want our people to have a name? In a more traditional design, we might just tack on a `name: String` field to `Person`. But other entities might have names too! For example, dogs should probably also have a name. It often makes sense to break up datatypes into small pieces to encourage code reuse. So let's make `Name` its own component:

{{file_code_block(file="quick-start/getting_started_v3.rs", anchor="name_component")}}

We can then add people to our [`World`] using a "startup system". Startup systems are just like normal systems, but they run exactly once, before all other systems, right when our app starts. Let's use [`Commands`] to spawn some entities into our [`World`]\:

{{file_code_block(file="quick-start/getting_started_v3.rs", anchor="add_people_system")}}

Now register the startup system like this:

{{file_code_block(file="quick-start/getting_started_v3.rs", anchor="app_main")}}

[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html

## Your First Query

We could run this now and the `add_people` system would run first, followed by `hello_world`. But our new people don't have anything to do yet! Let's make a system that properly greets the new citizens of our [`World`]:

{{file_code_block(file="quick-start/getting_started_v4.rs", anchor="greet_people_system")}}

The parameters we pass into a "system function" define what data the system runs on. In this case, `greet_people` will run on all entities with the `Person` and `Name` component.

You can interpret the [`Query`] above as: "iterate over every `Name` component for entities that also have a `Person` component".

Now we just register the system in our `App`. Note that you can pass more than one system into an `add_systems` call by using a tuple!

[`Query`]: <https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html>

{{file_code_block(file="quick-start/getting_started_v4.rs", anchor="app_main")}}

Running our app will result in the following output:

**Quick Note**: "hello world!" might show up in a different order than it does below. This is because systems run in parallel by default whenever possible.

```txt
hello world!
hello Elaina Proctor!
hello Renzo Hume!
hello Zayna Nieves!
```

Marvelous!

### Your First mutable Query

If we want to change the names of some people (perhaps they got married!), for example, we can do this using a mutable query:

{{file_code_block(file="quick-start/getting_started_v5.rs", anchor="update_people_system")}}

We need to make `query` mutable, and use a mutable reference (`&mut`) to the components we want to change.

Don’t forget to add the system to the [`Update`] schedule:

{{file_code_block(file="quick-start/getting_started_v5.rs", anchor="app_main")}}

Note that we have used `.chain()` on the two systems. This is because we want both of them to run in exactly the order they're listed in the code: with `update_people` occurring before `greet_people`.
If they weren’t, the name might change after we greet the people.

But we don’t add the `hello_world` system to the chain, because it doesn’t matter when it runs. This way, Bevy can run `hello_world` in parallel while the other systems are running.
