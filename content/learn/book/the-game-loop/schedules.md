+++
title = "Scheduling"
insert_anchor_links = "right"
[extra]
weight = 2
+++

At the heart of a bevy game is the "game loop": 
a continuously running for-loop which reads input, calls logic and updates the renderer.
Since bevy organizes game logic into [systems], the game loop boils down to repeatedly running a specific set of systems in sequence. 
To help manage the execution order of these systems, we group them into **Schedules**.

A [`Schedule`] is just a collection of systems, with information about the order in which they can be run. 
For convenience, schedules are identified with types (implementing the [`ScheduleLabel`] trait).
When adding a system to an app (with [`App::add_systems`]) you always pass in the label for the schedule you want it to run within.

When a bevy [app] starts, it typically executes several schedules in order:
+ [`StateTransition`]: Part of bevy's [state machine abstraction].
+ [`PreStartup`]: Library setup which must proceed application setup.
+ [`Startup`]: Setup for the application itself.
+ [`PostStartup`]: Stuff that must follow application setup.

[`PreStartup`] and [`PostStartup`] are primarily intended to allow libraries and crates to set things up before/after user code runs.
Libraries should use [`PreStartup`] to prepare the ECS for the user initialization.
Likewise, [`PostStartup`] should be used to react or respond to things the user did during setup.

These schedules are run only once, after [plugins] are initialized but before the game loop itself starts.
As the names imply, they are intended for initial setup tasks.
After the [`PostStartup`] schedule completes, the app shifts into the main game loop.
The following schedules are then executed in order each "tick":
+ [`First`]: Logic that needs to run before everything else each tick.
+ [`PreUpdate`]: Library updates that must proceed application updates.
+ [`StateTransition`]: Part of bevy's [state machine abstraction].
+ The [fixed update loop] may run multiple schedules before progressing.
+ [`Update`]: Updates for the application itself.
+ [`PostUpdate`]: Library updates that must follow application updates.
+ [`Last`]: Logic that needs to run after everything else each tick.

[`PreUpdate`] and [`PostUpdate`] (like [`PreStartup`] and [`PostStartup`]) allow library code to "sandwich" user code.
[`First`] and [`Last`] extend this further, allowing running before and after the majority of other logic.
Libraries should generally prefer the update schedules unless they have a very good reason to use `First` or `Last`.

[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`ScheduleLabel`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/trait.ScheduleLabel.html
[`App::add_systems`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_systems
[`StateTransition`]: https://docs.rs/bevy/latest/bevy/state/state/struct.StateTransition.html
[`PreStartup`]: https://docs.rs/bevy/latest/bevy/app/struct.PreStartup.html
[`Startup`]: https://docs.rs/bevy/latest/bevy/app/struct.Startup.html
[`PostStartup`]: https://docs.rs/bevy/latest/bevy/app/struct.PostStartup.html
[`First`]: https://docs.rs/bevy/latest/bevy/app/struct.First.html
[`PreUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html
[`StateTransition`]: https://docs.rs/bevy/latest/bevy/app/struct.StateTransition.html
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`PostUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PostUpdate.html
[`Last`]: https://docs.rs/bevy/latest/bevy/app/struct.Last.html
[systems]: /learn/book/control-flow/systems
[app]: /learn/book/the-game-loop/app
[plugins]: /learn/book/architecture/plugins
[state machine abstraction]: /learn/book/architecture/states
[fixed update loop]: /learn/book/the-game-loop/fixed-time
