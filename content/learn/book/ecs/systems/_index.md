+++
title = "Systems do work"
weight = 4
template = "book-section.html"
page_template = "book-section.html"
+++

In order to make useful, fun or interesting games or apps, you'll need to manipulate the data that you store in components and resources in some way.
In Bevy, virtually all of your logic will be stored in **systems**, functions that automatically receive data from the `World` from the scheduler according to their **system parameters**, and can mutate that data to change the world.
Any type which implements the [`SystemParam`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html) trait can be used as a function parameter in your system: this trait tells the scheduler how to pass out access to the `World` in a safe and efficient way.

Most commonly, you'll be using:

- [`Query`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html), to access entity-component database
- [`Res`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Res.html) and [`ResMut`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.ResMut.html), to access the global singleton data stored in resources
- [`Commands`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html), to queue up complex changes to the world like spawning entities
- [`EventWriter`](https://docs.rs/bevy/latest/bevy/app/struct.EventWriter.html) and [`EventReader`](https://docs.rs/bevy/latest/bevy/app/struct.EventReader.html), to work with events in an ergonomic fashion

You can see the full list by checking the [API docs for `SystemParam`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html#implementors).
If you would like, you can even add your own custom system parameters by deriving the `SystemParam` trait on structs whose fields all `impl SystemParam`.

Systems are added to your app using [`AppBuilder::add_system`](https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_system) and related methods on `App`, which will cause them to run once on every pass through the game loop.

## Startup systems

In many cases, you don't *want* your systems to run constantly: instead, you may only want to have them run a single time at the beginning to perform some setup.
To do this, use a **startup system**.

Startup systems run exactly once, before any ordinary systems, and can add them using [`AppBuilder::add_startup_system](https://docs.rs/bevy/latest/bevy/app/struct.AppBuilder.html#method.add_startup_system).

Carefully controlling if and when systems run is one of the most important tools you have for managing the behavior of the game.
Check out the pages on [system ordering](../../game-logic/system-ordering/_index.md), [run criteria](../../game-logic/run-criteria/_index.md) and [states](../../game-logic/states/_index.md) in the next chapter for more details.

## Generic systems

You are not limited to just *one* of a single system: you can insert more copies in different places through your schedule when you want to perform the work repeatedly.
This fact becomes particularly useful when we combine it with [generic types](https://doc.rust-lang.org/book/ch10-01-syntax.html): creating **generic systems** whose behavior is specialized on individual types.

Generic systems are useful for repeating the same logic across many related types, and are incredibly value for library authors who wish to provide configurable APIs that mesh nicely with their users code.
In the latter case, note that entire *plugins* can be made generic in the same way.

All of the standard tricks for Rust's generics work when used in systems, allowing you to create systems with [trait bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax), multiple generic type parameters and even [const generics](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html) as type arguments.
