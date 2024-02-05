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

## Primitive Shapes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## System Stepping

<div class="release-feature-authors">authors: @TODO</div>

## Dynamic Queries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Entity Optimizations

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## `WorldQuery` Trait Split

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Automatically insert `apply_deferred` systems

<div class="release-feature-authors">authors: @hymm</div>

A common scheduling issue is that one system needs to see the effects of commands executed in another system. Before 0.13, you would have to manually insert an
`apply_deferred` system between the two. Bevy now detects when a system with commands
is ordered relative to another and inserts the `apply_deferred` for you.

```rust
// Before 0.13
app.add_systems(
    Update,
    (
        system_with_commands,
        apply_deferred,
        another_system,
    ).chain()
);
```

```rust
// After 0.13
app.add_systems(
    Update,
    (
        system_with_commands,
        another_system,
    ).chain()
);
```

It also optimizes the automatically inserted `apply_deferred` systems by merging them if
possible. In most cases, it is recommended to remove all manually inserted
`apply_deferred` systems, as allowing Bevy to insert and merge these systems as needed will
usually be faster.

```rust
// This will only add one apply_deferred system.
app.add_systems(
    Update,
    (
        (system_1_with_commands, system_2).chain(),
        (system_3_with_commands, system_4).chain(),
    )
);
```

If this new behavior does not work for you, please consult the migration guide.
There are several new APIs for opting out of this.

## Input for one-shot systems
## Input for One-Shot Systems

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WGPU Upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Texture Atlas Rework

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Sprite Slicing and Tiling

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Exposure Settings

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Minimal Reflection Probes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light Maps

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light `RenderLayers`

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Approximate Indirect Specular Occlusion

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Unload Render Assets From RAM

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Bind Group Layout Entries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Type-Safe Labels for the `RenderGraph`

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Camera-Driven UI

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Winit Upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Animation Interpolation

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## `Animatable` Trait

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## glTF Extensions

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Extensionless Asset Support

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Gizmo Configuration

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
