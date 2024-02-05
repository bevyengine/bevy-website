+++
title = "Bevy 0.13"
date = 2024-02-03
[extra]
author = "Bevy Contributors"
image = "TODO.gif"
show_image = true
image_subtitle = "TODO"
image_subtitle_link = "TODO"

+++

Thanks to **TODO** contributors, **TODO** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.13** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.
And to see what the engine has to offer hands-on, check out the entries in the [latest Bevy Jam](https://itch.io/jam/bevy-jam-4/entries), including the winner [That's a lot of beeeeees](https://andrewb330.itch.io/thats-a-lot-of-beeeeees)

To update an existing Bevy App or Plugin to **Bevy 0.13**, check out our [0.12 to 0.13 Migration Guide](/learn/migration-guides/0.12-0.13/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **First-party primitive shapes:** basic shapes are a core building block of both game engines and video games: we've added a polished collection of them for you to use!
* **System stepping:** completely pause and advance through your game frame-by-frame or system-by-system to interactively debug game logic, all while rendering continues to update.
* **Dynamic queries:** refining queries from within systems is extremely expressive, and is the last big puzzle piece for runtime-defined types and third-party modding and scripting integration.
* **Automatically inferred command flush points:** tired of reasoning about where to put `apply_deferred` and confused about why your commands weren't being applied? Us too! Now, Bevy's scheduler uses ordinary `.before` and `.after` constraints and inspects the system parameters to automatically infer (and deduplicate) synchronization points.
* **Slicing, tiling and nine-patch sprites:** ninepatch layout is a popular tool for smoothly scaling stylized tilesets and UIs. Now in Bevy!
* **Lightmaps:** the first step towards baked global illumination: a fast, popular and pretty lighting technique.
* **Animation interpolation modes:** Bevy now supports non-linear interpolation modes in exported glTF animations.

## Primitive shapes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## System Stepping

<div class="release-feature-authors">authors: @dmlary</div>

The new system stepping feature (`bevy_debug_stepping`) adds debugger-style
execution control for systems.  The [`Stepping`] resource can control which
systems within a schedule execute each frame, and provides step, break, and
continue facilities to enable live debugging.  This feature is enabled by
default in the bevy crate.

The `Stepping` resource is configured with a list of schedules that it will
control execution of when stepping is enabled.  The collection of systems
within those schedules can be thought of as the stepping frame.  It can take
multiple step or continue calls to traverse through the entire stepping frame.
During this time, all schedules that have not been added to `Stepping` are
executing each frame.  This allows rendering & input handling to continue while
stepping through systems.

### Configuration

To get started the [`Stepping`] resource must be configured with the schedules
it will be controlling, then added to the world:

```rust
// create a new Stepping resource, and add schedules to debug
let mut stepping = Stepping::new();
stepping.add_schedule(Update);
stepping.add_schedule(FixedUpdate);

// add the Stepping resource to the world
app.insert_resource(stepping);
```

The [`Stepping`] resource has no effect until it is enabled with a call to
`Stepping::enable()`. When the `Stepping` resource is present and enabled,
systems within the added schedules will not be run unless we're performing
a system step, continuing the stepping frame, or the system has been exempt
from stepping.

### Execution Control: System Step & Continue Frame

While stepping is enabled, the `Stepping` resource tracks its location within
the stepping frame, maintaining a stepping cursor pointing at the next system
to be executed.
There are two ways systems get executed while stepping is enabled: system step,
and continue frame.

System step (`Stepping::step_frame()`) runs the system at the stepping
cursor, and advances the cursor during the next render frame.
This is useful to see individual changes made by
systems, and see the state of the world prior to executing a system

Continue frame (`Stepping::continue_frame()`) will execute systems starting
from the stepping cursor to the end of the stepping frame during the next frame.
It may stop before the end of the stepping frame if it encounters a system with
a breakpoint. This is useful for advancing quickly through an entire frame,
getting to the start of the next frame, or in combination with breakpoints.

In this video we demonstrate system step & continue frame on the breakout
example with an egui interface.  The stepping cursor can be seen moving through
the systems list as we click the `step` button.  When the `continue` button is
clicked, you can see the game progress one stepping frame for each click.

<video controls><source src="stepping-step-continue.mp4" type="video/mp4"/></video>

### Breakpoints

When a schedule grows to a certain point, it can take a long time to step
through every system in the schedule just to see the effects of a few systems.
In this case, stepping provides system breakpoints.

You can set a breakpoint on those systems you care about, then use
`Stepping::continue_frame()` to run systems starting at the stepping cursor
until a breakpoint is encountered, or the end of the stepping frame.
If the stepping cursor points at a system with a breakpoint when you call
`Stepping::continue_frame()` that system will run.  This allows you to set
breakpoints through a system, and repeatedly continue the frame to stop prior
to each system of interest.

In this video of the breakout example, we add a breakpoint to
`check_for_collisions()` so we can verify the collision detection & handling
behavior each frame without stepping through all the other systems.

The video shows the stepping cursor moves from the start of the stepping frame
to `check_for_collisions()` the first time we click `continue` in the ui.  On
the next click, `check_for_collisions()` and all remaining systems are run,
moving the cursor back up to the start of the stepping frame.

<video controls><source src="stepping-breakpoint.mp4" type="video/mp4"/></video>

### Disabling Systems

During debugging, it can be helpful to disable systems to narrow down the
source of the problem.  `Stepping::never_run()` and `Stepping::never_run_node()`
can be used to disable systems while stepping is enabled.  Note that disabling
systems in this manner only has an effect while stepping is enabled.  When
stepping is disabled, all disabled systems will resume executing.

In this video of the breakout example, we disable the `check_for_collisions()`
system and use continue frame to move the ball into the center of the blocks,
then re-enable the system to have fun destroying all the blocks from the
inside.

<video controls><source src="stepping-disable-system.mp4" type="video/mp4"/></video>

### Excluding Systems from Stepping

It may be necessary to ensure some systems still run while stepping is enabled.
While best-practice is to have them in a schedule that has not been added to
the `Stepping` resource, it is possible to configure systems to always run
while stepping is enabled.  This is primarily useful for event & input handling
systems.

Systems can be configured to always run by calling
`Stepping::always_run()`, or `Stepping::always_run_node()`.
When a system is configured to always run, it will run each rendering frame
even when stepping is enabled.

In this video of the breakout example the `move_paddle()` system is configured
to always run.  We then use the arrow keys to move the paddle while the rest
of the game systems are stopped by stepping.  This allows us to position the
paddle precisely before continuing frame-by-frame as the ball impacts the
paddle.

<video controls><source src="stepping-run-always.mp4" type="video/mp4"/></video>

### Limitations

In this initial implementation of stepping there are some limitations:

* Any system that reads events likely will not step properly
  * Frames still advance normally while stepping is enabled
  * Events can be cleared before a stepped system can read them
  * Best approach here is to configure event-based systems to always run,
    or put them in a schedule not added to `Stepping`
  * Continue with breakpoints may also work in this scenario
* Conditional systems may not run as expected when stepping
  * Similar to event-based systems, if the condition is true for only a short
    time, system may not run when stepped

### Detailed Examples

* [Text-based stepping example](https://github.com/bevyengine/bevy/blob/main/examples/ecs/system_stepping.rs)
* Non-interactive [bevy UI example stepping plugin](https://github.com/bevyengine/bevy/blob/main/examples/games/stepping.rs) used in the breakout example
* Interactive [egui stepping plugin](https://gist.github.com/dmlary/3fd57ebf1f88bb9afa8a6604737dac97) used in demo videos

[`Stepping`]: https://docs.rs/bevy/0.13.0/bevy/ecs/schedule/stepping/Stepping.html

## Dynamic queries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Entity optimizations

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WorldQuery trait split

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Automatically inserted sync points

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Input for one-shot systems

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WGPU upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Texture atlas rework

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Sprite slicing and tiling

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Exposure settings

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Minimal reflection probes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light maps

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light RenderLayers

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Approximate indirect specular occlusion

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Unload render assets from RAM

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Bind group layout entries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Type-safe labels for the `RenderGraph`

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Camera-driven UI

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Winit upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Animation interpolation

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## `Animatible` trait

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## gltF extensions

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Extensionless asset support

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Gizmo configuration

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## <a name="what-s-next"></a>What's Next?

We have plenty of work in progress! Some of this will likely land in **Bevy 0.14**.

Check out the [**Bevy 0.14 Milestone**](https://github.com/bevyengine/bevy/milestone/20) for an up-to-date list of current work that contributors are focusing on for **Bevy 0.14**.

* **More editor experimentation:** TODO
* **bevy_dev_tools:** TODO
* **A revised scene format:** TODO
* **bevy_ui improvements:** TODO
* **The steady march towards relations:** TODO
* **Animation blending:** TODO
* **Irradiance volumes:** TODO

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the 185 contributors that made this release (and associated docs) possible! In random order:

TODO: add contributors

## Full Changelog

The changes mentioned above are only the most appealing, highest impact changes that we've made this cycle.
Innumerable bug fixes, documentation changes and API usability tweaks made it in too.
For a complete list of changes, check out the PRs listed below.

TODO: add full changelog, sorting by area.
