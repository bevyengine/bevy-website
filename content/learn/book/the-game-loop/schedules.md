+++
title = "Schedules"
insert_anchor_links = "right"
[extra]
weight = 2
+++

At the heart of a Bevy game is the "game loop": a continuously running for-loop which reads input, calls logic and updates the renderer.
Since Bevy organizes game logic into [systems], the game loop boils down to repeatedly running a specific set of systems in sequence.
To help manage the execution order of these systems, we group them into **schedules**.

A [`Schedule`] is just a collection of systems which will all be executed together.
You add systems to a schedule as part of your app setup.

```rust
// Add the `move_players` system to the `Update` schedule.
app.add_plugin(Update, move_players)
```

Now, whenever Bevy runs the `Update` schedule, `move_players` will execute.

The `Update` schedule is one of many built-in schedules provided by Bevy, each of which runs at a different point.
Each schedule runs at a different point during the life-cycle of a Bevy app, so by controlling where your system is registered you can also control how it's run.

## The standard Bevy schedules

Let's look at the other schedules provided by Bevy by default.
When a Bevy [app] starts, it typically executes several schedules in order:

+ [`StateTransition`]: Part of Bevy's [state machine abstraction].
+ [`PreStartup`]: Library setup which must proceed application setup.
+ [`Startup`]: Setup for the application itself.
+ [`PostStartup`]: Stuff that must follow application setup.

These schedules are run only once, after [plugins] are initialized but before the game loop itself starts.
As the names imply, they are intended for initial setup tasks.
Application logic should usually be set up in the `Startup` schedule.
Libraries should use [`PreStartup`] to prepare the ECS for the user initialization.
Likewise, [`PostStartup`] should be used to react or respond to things the user did during setup.

After the [`PostStartup`] schedule completes, the app shifts into the main game loop.
The following schedules are then executed in order each "tick":

+ [`First`]: Logic that needs to run before everything else each tick.
+ [`PreUpdate`]: Library updates that must proceed application updates.
+ [`StateTransition`]: Part of Bevy's [state machine abstraction].
+ The [fixed update loop] may run multiple schedules before progressing.
+ [`Update`]: Updates for the application itself.
+ [`PostUpdate`]: Library updates that must follow application updates.
+ [`Last`]: Logic that needs to run after everything else each tick.

[`PreUpdate`] and [`PostUpdate`] (like [`PreStartup`] and [`PostStartup`]) allow library code to "sandwich" user code.
[`First`] and [`Last`] extend this further, allowing running before and after the majority of other logic.
Libraries should generally prefer the update schedules unless they have a very good reason to use `First` or `Last`.

## The Main Schedule

It may also be helpful for you to know about the `Main` schedule, though this begins to get into the internal implementation a little.
The "main loop" is itself a schedule called `Main`, which execute other schedules each tick (`First`, then `PreUpdate` and so on).
Most people will never need to interact with it directly, but it's good to know that it exists.

It's also possible for users to add their own schedules, either extending the `Main` schedule or [replacing it entirely].

[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`StateTransition`]: https://docs.rs/bevy/latest/bevy/state/state/struct.StateTransition.html
[`PreStartup`]: https://docs.rs/bevy/latest/bevy/app/struct.PreStartup.html
[`Startup`]: https://docs.rs/bevy/latest/bevy/app/struct.Startup.html
[`PostStartup`]: https://docs.rs/bevy/latest/bevy/app/struct.PostStartup.html
[`First`]: https://docs.rs/bevy/latest/bevy/app/struct.First.html
[`PreUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`PostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PostUpdate.html
[`Last`]: https://docs.rs/bevy/latest/bevy/app/struct.Last.html
[systems]: /learn/book/control-flow/systems
[app]: /learn/book/the-game-loop/app
[plugins]: /learn/book/architecture/plugins
[state machine abstraction]: /learn/book/architecture/states
[fixed update loop]: /learn/book/the-game-loop/fixed-time
[replacing it entirely]: /learn/book/the-game-loop/custom-loops
