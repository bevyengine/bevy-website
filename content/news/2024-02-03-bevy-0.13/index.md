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
* **Dynamic queries:** refining queries from within systems is extremely expressive, and is the last big puzzle piece for runtime-defined types and third-party modding and scripting integration.
* **Automatically inferred command flush points:** tired of reasoning about where to put `apply_deferred` and confused about why your commands weren't being applied? Us too! Now, Bevy's scheduler uses ordinary `.before` and `.after` constraints and inspects the system parameters to automatically infer (and deduplicate) synchronization points.
* **Slicing, tiling and ninepatch sprites and UI:** ninepatch layout is a popular tool for smoothly scaling stylized tilesets and UIs. Now in Bevy!
* **Lightmaps:** the first step towards baked global illumination: a fast, popular and pretty lighting technique.
* **Animation interpolation modes:** Bevy now supports non-linear interpolation modes in exported glTF animations.

## Primitive shapes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

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

<div class="release-feature-authors">authors: @DasLixou</div>

Bevy uses Rust's type system at many points like for `SystemSets` which gives it a modern feel.
But this didn't apply to Bevy's render graph. In the render graph, hard-coded—and potentially overlapping—strings were used to define nodes and sub-graphs.

```rust
// Before 0.13
impl MyRenderNode {
    pub const NAME: &'static str = "my_render_node"
}
```

We are now introducing a more robust way to name rendernodes and -graphs with the help of Rust's type system and Bevy's label system.

```rust
// After 0.13
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct MyRenderLabel;
```

With those, the long paths for const-values also get shortened and cleaner which results into this

```rust
// Before 0.13
render_app
    .add_render_graph_node::<ViewNodeRunner<MyRenderNode>>(
        core_3d::graph::NAME,
        MyRenderNode::NAME,
    )
    .add_render_graph_edges(
        core_3d::graph::NAME,
        &[
            core_3d::graph::node::TONEMAPPING,
            MyRenderNode::NAME,
            core_3d::graph::node::END_MAIN_PASS_POST_PROCESSING,
        ],
    );

// After 0.13
use bevy::core_pipeline::core_3d::graph::{Labels3d, SubGraph3d};

render_app
    .add_render_graph_node::<ViewNodeRunner<MyRenderNode>>(
        SubGraph3d,
        MyRenderLabel,
    )
    .add_render_graph_edges(
        SubGraph3d,
        (
            Labels3d::Tonemapping,
            MyRenderLabel,
            Labels3d::EndMainPassPostProcessing,
        ),
    );
```

When you need dynamic labels for render nodes, those can still be achieved via e.g. tuple structs:

```rust
#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
pub struct MyDynamicLabel(&'static str);
```

The nice thing is, that this don't has to be a string, it can be any type, also numbers etc.

## Camera-driven UI

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Winit upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Animation interpolation

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
