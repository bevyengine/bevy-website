+++
title = "Bevy 0.15"
date = 2024-12-31 # TODO, fix date
draft = true
[extra]
image = "cover.jpg" # TODO, add cover image
show_image = true
image_subtitle = "TODO"
image_subtitle_link = "https://github.com/TODO"
+++

Thanks to **TODO** contributors, **TODO** pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.15** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.15**, check out our [0.14 to 0.15 Migration Guide](/learn/migration-guides/0-14-to-0-15/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- Required Components
- Entity Picking / Selection
- Animation: generalized entity animation, animation masks, additive blending, animation events
- Curves: the `Curve` trait, cyclic splines, common easing functions, color gradient curves
- Reflection: Function reflection, unique reflect, remote type reflection
- Bevy Remote Protocol (BRP)
- VBAO
- Chromatic Aberration
- Volumetric Fog Improvements: Fog Volumes. Support for Point Lights and Spotlights.
- Order Independent Transparency
- Improved Text Rendering: Cosmic Text
- Gamepads as Entities
- UI Box Shadows


As is now tradition, Bevy 0.15 was prepared using a **release candidate** process to help ensure that you can upgrade right away with peace of mind.
We've worked closely with both plugin authors and ordinary users to catch critical bugs, round the rough corners off our new features, and refine the migration guide.
As we prepared fixes, we've [shipped new release candidates on crates.io](https://crates.io/crates/bevy/versions?sort=date), letting core ecosystem crates update and listening closely for show-stopping problems.
Thank you so much to [everyone who helped out](https://discord.com/channels/691052431525675048/1295069829740499015): these efforts are a vital step towards making Bevy something that teams large and small can trust to work reliably.

<!-- more -->

{{ release_notes(version="0.15") }}

## What's Next?

The features above may be great, but what else does Bevy have in flight?
Peering deep into the mists of time (predictions are _extra_ hard when your team is almost all volunteers!), we can see some exciting work taking shape:

- **Bevy Scene Notation:** Required components mark the first step on Cart's [master plan](https://github.com/bevyengine/bevy/discussions/14437) for BSN. Over the next few months, he's going to be heads-down developing a Bevy-specific file format (complete with matching macro and IDE support), the `Construct` trait (to easily include asset data in scenes), patches (to layer modifications to scenes) and experimenting with approaches to reactivity for UI.
- **Better font support:** While `cosmic_text` is a huge leap forward for text shaping and rendering, our approach to handling fonts and type-faces is still quite crude. Bidirectional text, working with system fonts, a convenient Markdown-style "bold this section of the text" API, font fallback and more are planned.
- **Picking-Powered UI Interaction:** `bevy_picking` introduces a much more powerful and expressive way to handle pointer interactions, but we're [not leveraging its full power](https://github.com/bevyengine/bevy/issues/15550) within `bevy_ui` itself. While picking events are great, a single source of truth for "what's the user doing with this button" is vital for responsive widget styling.
- **`bevy_lint`:** Try as we might, it _is_ possible to misuse Bevy's API! As part of a broader [`bevy_cli`](https://github.com/theBevyFlock/bevy_cli) project, the Bevy community has developed a Bevy-specific linter to catch common mistakes or hazards and are looking for early adopters to try it out!
- **Focus abstraction:** Keeping track of which UI element is focused is vital to allow users of screen readers, gamepads and keyboards to comfortably navigate the UI. We're planning to build on our success with `bevy_picking` and develop a complementary [focus-tracking solution](https://github.com/bevyengine/bevy/issues/15378), along with a few simple backends to opt-in to keyboard or gamepad-based UI navigation.
- **Immutable components:** Component hooks and observers are really powerful for responding to changes and upholding invariants, but they're easily bypassed by simply mutating the component. The mad science crew has been [experimenting with](https://github.com/bevyengine/bevy/issues/16208) a way to opt-out of direct mutation, opening the door to more robust hierarchies, complex observer-powered reactions and a first-party component indexing solution.
- **Actually Retained Rendering:** While the render world is _technically_ retained in Bevy 0.15, most of our existing code still spawns and despawns entities every frame to reduce the risk of introducing bugs during the migration. We're looking forward to gradually changing this and profiling the performance impact!
- **`no_std` Bevy:** To better support weird platforms (like the [Playdate](https://play.date/)!) and make life easier for devs experimenting with Bevy on modern consoles, we've been [working towards](https://github.com/bevyengine/bevy/issues/15460) ensuring that (much of) Bevy can compile and run without Rust's standard library.

{{ support_bevy() }}
{{ contributors(version="0.15") }}
{{ changelog(version="0.15")}}
