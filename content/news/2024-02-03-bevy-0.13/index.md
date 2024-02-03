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

System stepping is a new feature that adds debugger-style step/break/continue
facilities for controlling the execution of systems in a schedule.   This
facility is exposed via a new [`Stepping`] resource.  This feature is enabled
by default, but can be explicitly enabled with the `bevy_debug_stepping`
feature flag.

To get started the [`Stepping`] resource must be configured with the schedules
it will be controlling, then added to the world:
```rust
// configure the stepping resource with the schedules you want to be able to
// step through
let mut stepping = Stepping::new();
stepping.add_schedule(Update);
stepping.add_schedule(FixedUpdate);
app.insert_resource(stepping);
```
The [`Stepping`] resource has no effect until it is enabled with a call to 
`Stepping::enable()`.

Once stepping is enabled, systems within schedules that have been added to
`Stepping` will not run unless:
* `Stepping::step_frame()` or `Stepping::continue_frame()` has been called
    during the previous frame
* The system has been configured to always run with `Stepping::always_run()` or
    `Stepping::always_run_node()`
* Stepping is disabled with `Stepping::disable()`
* The Stepping resource is removed

Systems in other schedules will execute every frame.

### Execution Control: Step & Continue
Controlling execution of systems while stepping is provided by
`Stepping::step_frame()`, and `Stepping::continue_frame()`.
The `Stepping` resource tracks the execution of systems and maintains a
cursor of which schedule & system will execute next.

Calling `Stepping::step_frame()` will execute only the system at the cursor
during the next frame.  This is useful to see individual changes made by
systems, and see the state of the world prior to executing a system

Calling `Stepping::continue_frame()` will execute all
systems from the current cursor to the end of the frame during the next frame.
This is useful for advancing quickly through an entire frame, getting to the
start of the next frame, or in combination with breakpoints.

In this video we demonstrate stepping & continue on the breakout example with
an egui interface.  The cursor can be seen moving through the systems list as
we click the `step` button.  When the `continue` button is clicked, you can see
the game progress one frame for each click.

<video controls><source src="stepping-step-continue.mp4" type="video/mp4"/></video>

### Breakpoints
When a schedule has dozens of systems, but you're only interested in the effects
of a few of them, breakpoints can be added with `Stepping::break()` can be used
to add a breakpoint for a given system:

```rust
stepping.set_breakpoint(FixedUpdate, check_for_collisions);
```

Continuing will resume execution of the frame, but will stop if it encounters a
system with a breakpoint.  If the stepping cursor is at a breakpoint when
`Stepping::continue_frame()` is called, then stepping will execute that system
and any following systems until another breakpoint is encountered, or all
systems have been run.

In this video of the breakout example, we add a breakpoint to
`check_for_collisions()` so we can verify the behavior of the system
each frame without stepping through all the other systems.  The stepping
cursor moves from the start of the frame to `check_for_collisions()` the first
time we click `continue` in the ui.  On the next click, `check_for_collisions()`
and all remaining systems are run, moving the cursor back up to the start of
the system list.

<video controls><source src="stepping-breakpoint.mp4" type="video/mp4"/></video>

### Disabling Systems
During debugging it may be necessary to disable systems to eliminate them as the
source of the problem.  This can be done in stepping with `Stepping::never_run()`
or `Stepping::never_run_node()`.  Note that systems are only disabled while
stepping is enabled; when stepping is disabled, disabled systems will run as
normal.

In this video of the breakout example, we disable the `check_for_collisions()`
system and use continue to move the ball into the center of the blocks, then
re-enable the system to have fun destroying all the blocks from the inside.

<video controls><source src="stepping-disable-system.mp4" type="video/mp4"/></video>

### Excluding Systems from Stepping
There are cases where a system should run when stepping is enabled.  Good
examples of this are input handling functions.  To exclude a system from being
controlled by stepping use `Stepping::always_run()` or
`Stepping::always_run_node()`.  When a system is set to always run, it will run
each rendered frame regardless of stepping state.

In this video of the breakout example, set `move_paddle()` to always run, and
then use the keys to move the paddle while the rest of the game appears to be
paused as the other systems are not running.

<video controls><source src="stepping-run-always.mp4" type="video/mp4"/></video>

### Limitations
In this initial implementation of stepping there are some limitations:
* Any system that reads events likely will not step properly
    * Frames still advance normally while stepping is enabled
    * Events can be cleared before a stepped system can read them
    * Best approach here is to configure event-based systems to always run
    * Continue with breakpoints may also work in this scenario
* Conditional systems may not run as expected when stepping
    * Similar to event-based systems, if the condition is true for only a short
      time, system may not run when stepped

### Further Reading
* [Text-based stepping example](https://github.com/bevyengine/bevy/blob/main/examples/ecs/system_stepping.rs)
* Non-interactive [bevy UI example stepping plugin](https://github.com/bevyengine/bevy/blob/examples/games/stepping.rs) used in the breakout example
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
