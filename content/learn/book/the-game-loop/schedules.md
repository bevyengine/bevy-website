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
app.add_systems(Update, move_players)
```

Now, whenever Bevy runs the `Update` schedule, `move_players` will execute.

The `Update` schedule is one of many built-in schedules provided by Bevy (collectively called the [`Main`] schedule), each of which runs at a different point.
Each schedule runs at a different point during the life-cycle of a Bevy app, so by controlling where your system is registered you can also control how it's run.

## The Standard Bevy Schedules

Let's look at the rest of the [`Main`] schedules provided by Bevy by default.
When a Bevy [app] starts, it typically executes several schedules in order:

1. [`StateTransition`]: Part of Bevy's [state machine abstraction].
2. [`PreStartup`]: Library setup which must precede application setup.
3. [`Startup`]: Setup for the application itself.
4. [`PostStartup`]: Stuff that must follow application setup.

These schedules are run only once, after [plugins] are initialized but before the game loop itself starts.
As the names imply, they are intended for initial setup tasks.
Application logic should usually be set up in the `Startup` schedule.
Libraries should use [`PreStartup`] to prepare the ECS for the user initialization.
Likewise, [`PostStartup`] should be used to react or respond to things the user did during setup.

After the [`PostStartup`] schedule completes, the app shifts into the main game loop.
This loop consists of the application repeatedly calling [`App::update()`], and results in the evaluation of a single game "frame".
The following schedules are then executed in order each "frame":

1. [`First`]: Logic that needs to run before everything else each frame.
2. [`PreUpdate`]: Library updates that must precede application updates.
3. [`StateTransition`]: Part of Bevy's [state machine abstraction].
4. [`RunFixedMainLoop`]: May run the [`FixedMain`] schedule zero to many times before progressing.
5. [`Update`]: Updates for the application itself.
6. [`PostUpdate`]: Library updates that must follow application updates.
7. [`Last`]: Logic that needs to run after everything else each frame.

[`PreUpdate`] and [`PostUpdate`] (like [`PreStartup`] and [`PostStartup`]) allow library code to "sandwich" user code.
[`First`] and [`Last`] extend this further, allowing running before and after the majority of other logic.
Libraries should generally prefer the update schedules unless they have a very good reason to use `First` or `Last`.

### Fixed Schedules

Schedules running in [`Main`] are only evaluated once per frame.
This is great if our game generates an exact amount of frames every second, but usually this isn't the case.
If we can't count on our game generating frames at a consistent rate, how can we reliably (and deterministically) update game logic that requires a consistent rate?

To solve this, Bevy provides a set of "fixed" schedules through [`FixedMain`].
We call them "fixed" since they run at a consistent rate (which we call a game "tick").
This is opposed to the "frame"-based schedules that run in [`Main`], which only run every time a frame is generated.

Much like in [`Main`], [`FixedMain`] provides several schedules that are executed in order:

1. [`FixedFirst`]: Logic that needs to run first before anything else in [`FixedMain`].
2. [`FixedPreUpdate`]: Logic that runs before [`FixedUpdate`].
3. [`FixedUpdate`]: Updates for gameplay logic that runs at a set rate rather than every frame.
4. [`FixedPostUpdate`]: Logic that needs to run after [`FixedUpdate`].
5. [`FixedLast`]: Logic that needs to run last after everything else in [`FixedMain`].

Every time the [`Main`] schedule runs, the [`RunFixedMainLoop`] schedule will check to see how many "ticks" have occurred since the last `Main` schedule.
This is done by accessing the [`Time<Fixed>`] resource, which contains the rate at which our "fixed" schedules will run (which is 64hz by default).
Because of the way we check this, it's possible for each schedule in [`FixedMain`] to run multiple times.

{% callout(type="info") %}
#### When to Use Fixed Schedules

Placing a system inside of a "fixed" schedule (like [`FixedUpdate`]) means that it will be evaluated at a consistent rate (64 times a second by default, to be exact).
But deciding what systems to place in [`FixedUpdate`] over [`Update`] might not always be an obvious choice.
What logic do you want to run on a "fixed" timestep (a truly consistent basis)?
What should only be updated on a "frame-by-frame" basis (only updated when a new frame is rendered)?
Ultimately, it boils down to personal choices about the game you're making.

Take handling input, for example.
If you were to update your game based on input data in the [`FixedUpdate`] schedule, you would be updating at a consistent rate.
This might sound great at first, but what happens if the game starts to visibly lag for the player?
If input data is still being sent, the player might continue to move or perform an action even if they can't see the action happening in game.
This might be preferable for some games, but chances are that this behavior isn't intended.
Whereas handling input handling logic in the [`Update`] schedule will avoid this since the input data is only sent for each frame that is rendered.

To better understand how the "fixed" schedules are evaluated, you can read the [Fixing Your Timestep section](@/learn/book/the-game-loop/game-time.md#fixing-your-timestep) of the [Time page](@/learn/book/the-game-loop/game-time.md#).
Otherwise, we can provide a general recommendation that systems relating to UI, input handling, and audio should be placed in the [`Update`] schedule.
For gameplay systems needing to run at a fixed rate (like physics, AI behaviors, networking, and game rules), we recommend placing them in the [`FixedUpdate`] schedule.

[`FixedUpdate`]: https://docs.rs/bevy/latest/bevy/prelude/struct.FixedUpdate.html
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html

{% end %}

## Adding Custom Schedules

The repeating game loop is set up by the [`ScheduleRunnerPlugin`],
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

## Arranging Systems in Schedules





[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`StateTransition`]: https://docs.rs/bevy/latest/bevy/state/state/struct.StateTransition.html
[`PreStartup`]: https://docs.rs/bevy/latest/bevy/app/struct.PreStartup.html
[`Startup`]: https://docs.rs/bevy/latest/bevy/app/struct.Startup.html
[`PostStartup`]: https://docs.rs/bevy/latest/bevy/app/struct.PostStartup.html
[`App::update()`]: https://docs.rs/bevy/latest/bevy/prelude/struct.App.html#method.update
[`First`]: https://docs.rs/bevy/latest/bevy/app/struct.First.html
[`PreUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html
[`RunFixedMainLoop`]: https://docs.rs/bevy/latest/bevy/prelude/struct.RunFixedMainLoop.html
[`FixedMain`]: https://docs.rs/bevy/latest/bevy/app/struct.FixedMain.html
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`PostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PostUpdate.html
[`Last`]: https://docs.rs/bevy/latest/bevy/app/struct.Last.html
[`FixedFirst`]: https://docs.rs/bevy/latest/bevy/prelude/struct.FixedFirst.html
[`FixedPreUpdate`]: https://docs.rs/bevy/latest/bevy/prelude/struct.FixedPreUpdate.html
[`FixedUpdate`]: https://docs.rs/bevy/latest/bevy/prelude/struct.FixedUpdate.html
[`FixedPostUpdate`]: https://docs.rs/bevy/latest/bevy/prelude/struct.FixedPostUpdate.html
[`FixedLast`]: https://docs.rs/bevy/latest/bevy/prelude/struct.FixedLast.html
[`Main`]: https://docs.rs/bevy/latest/bevy/app/struct.Main.html
[systems]: @/learn/book/control-flow/systems.md
[app]: @/learn/book/the-game-loop/app.md
[plugins]: @/learn/book/modular-architecture/plugins.md
[state machine abstraction]: @/learn/book/control-flow/states.md
[`Time<Fixed>`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Time.html#impl-Time%3CFixed%3E
[`ScheduleRunnerPlugin`]: https://docs.rs/bevy/latest/bevy/app/struct.ScheduleRunnerPlugin.html
[`MinimalPlugins`]: https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html
[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[`MainScheduleOrder`]: https://docs.rs/bevy/latest/bevy/app/struct.MainScheduleOrder.html
[`ScheduleLabel`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.ScheduleLabel.html
[`World::run_schedule`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html#method.run_schedule
[fixed time]: @/learn/book/the-game-loop/game-time.md#frame-rate-independence-and-delta-time
