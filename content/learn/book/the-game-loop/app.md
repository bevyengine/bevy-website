+++
title = "App"
insert_anchor_links = "right"
[extra]
weight = 0
+++

[`App`] is the entry-point to a bevy application.
Apps exist to configure bevy before start up: registering systems, initializing resources, and registering events, and so on.
After it's configured, the can be started with [`App::run`].
This typically takes control of the main thread and enters the game loop.

You can think of an `App` as containing three things:
+ A [world], which stores all the data for the app.
+ A run-loop function, to execute [schedules].
+ A set of [plugins], which are important enough to deserve their own section.

Most app configuration -- registering systems, ect -- boils down to modifying the world. 
To simplify this, [`App`] proxies most of the [`World`] api.
The world can also be accessed directly via [`App::world_mut`].

The main thing that distinguishes an [`App`] from a world is the runner function. 
It's this function that defines the game loop, and like everything else in bevy it's configurable:
You can manually set a runner with [`App::set_runner`].
Generally this will be set for you by `DefaultPlugins` and you shouldn't need to call it yourself.
We will take a closer look at the runner function in the [scheduling] chapter.

[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html
[`App::run`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.run
[`App::world_mut`]:  https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.world_mut
[`App::set_runner`]:  https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.set_runner
[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/index.html
[schedules]: /learn/book/the-game-loop/schedules
[scheduling]: /learn/book/the-game-loop/schedules
[world]: /learn/book/storing-data/world
[plugins]: /learn/book/architecture/plugins
