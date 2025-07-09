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

## Adding your own schedules

This repeating game loop is set up by the [`ScheduleRunnerPlugin`],
which is included as part of both [`MinimalPlugins`] and [`DefaultPlugins`].

That plugin causes your app to repeatedly run the [`Main`] schedule, which is a schedule that loops over a vector of schedules.
While it is rarely helpful, you can add your own schedules to this game loop, by modifying the [`MainScheduleOrder`].
To create your own schedule, simply define a [`ScheduleLabel`], and add your systems to your app under that schedule label.

```rust,hide_lines=1-2
# use bevy::prelude::*;
# fn my_system() {}

#[derive(ScheduleLabel)]
struct MySchedule;

App::new()
  .add_systems(MySchedule, my_system);
```

However, in most cases, the existing schedules are more than enough,
and system ordering should be used to control the exact timing of your systems relative to each other.

A more helpful pattern is calling [`World::run_schedule`] manually.
You can organize systems into complex groups, with well-defined ordering between them,
and then evaluate some complex condition inside of a system to determine if and when this schedule gets evaluated.
This can be very helpful for turn-based games, simulations, networked servers and more.
Bevy itself uses this pattern for both the [`Main`] schedule and our built-in [fixed time] solution.

For even more control over your game loop, read the [custom loops] chapter!

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
[`Main`]: https://docs.rs/bevy/latest/bevy/app/struct.Main.html
[systems]: /learn/book/control-flow/systems
[app]: /learn/book/the-game-loop/app
[plugins]: /learn/book/architecture/plugins
[state machine abstraction]: /learn/book/architecture/states
[fixed update loop]: /learn/book/the-game-loop/fixed-time
[custom loops]: /learn/book/the-game-loop/custom-loops
[`ScheduleRunnerPlugin`]: https://docs.rs/bevy/latest/bevy/app/struct.ScheduleRunnerPlugin.html
[`MinimalPlugins`]: https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html
[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[`MainScheduleOrder`]: https://docs.rs/bevy/latest/bevy/app/struct.MainScheduleOrder.html
[`World::run_schedule`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html#method.run_schedule
[fixed time]: ./fixed-time.md
