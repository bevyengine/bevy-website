+++
title = "App"
insert_anchor_links = "right"
[extra]
weight = 0
+++

An [`App`] is the entry-point to a bevy game.
Apps exist to configure bevy before start up: registering [systems], initializing [resources], and registering [events], and so on.
Once configured, an app can be started with [`App::run`].
This takes control of the main thread and enters the game loop.

You can think of an `App` as containing three things:

+ A [`World`], which stores all the data for the app.
+ A run-loop function, to execute [schedules].
+ A set of [plugins], which are important enough to deserve their own section.

Most app configuration -- registering systems, etc. -- boils down to modifying the world.
To simplify this, [`App`] proxies most of the [`World`] API.
The world can also be accessed directly via [`App::world_mut`].

The main thing that distinguishes an [`App`] from a world is the runner function.
It's this function that defines the game loop, and like everything else in bevy it's configurable:
You can manually set a runner with [`App::set_runner`], but you shouldn't have to:
When using [`DefaultPlugins`] or [`MinimalPlugins`] the runner is set for you.
We will take a closer look at the runner function in the [custom loops] chapter.

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`App::run`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.run
[`App::world_mut`]:  https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.world_mut
[`App::set_runner`]:  https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.set_runner
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/index.html
[systems]: /learn/book/control-flow/systems
[resources]: /learn/book/storing-data/resources
[events]: /learn/book/control-flow/events
[schedules]: /learn/book/the-game-loop/schedules
[custom loops]: /learn/book/the-game-loop/schedules
[plugins]: /learn/book/architecture/plugins
[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[`MinimalPlugins`]: https://docs.rs/bevy/latest/bevy/struct.MinimalPlugins.html
