+++
title = "Bevy 0.15"
date = 2024-11-29
[extra]
image = "cover.png"
show_image = true
image_subtitle = "A snake statue in volumetric fog illuminated by volumetric lighting"
image_subtitle_link = "https://sketchfab.com/3d-models/snake-statue-794b77a3e4654a669cf259d20dc89ec7"
+++

Thanks to **294** contributors, **1217** pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.15** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevy.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.15**, check out our [0.14 to 0.15 Migration Guide](/learn/migration-guides/0-14-to-0-15/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **Required Components**: A rethink of how spawning entities works that significantly improves the Bevy user experience
- **Entity Picking / Selection**: A modular system for selecting entities across contexts
- **Animation Improvements**: generalized entity animation, animation masks, additive blending, and animation events
- **Curves**: a new `Curve` trait, cyclic splines, common easing functions, color gradient curves
- **Reflection Improvements**: Function reflection, unique reflect, remote type reflection
- **Bevy Remote Protocol (BRP)**: A new protocol that allows external clients (such as editors) to interact with running Bevy games
- **Visibility Bitmask Ambient Occlusion (VBAO)**: An improved GTAO algorithm that improves ambient occlusion quality
- **Chromatic Aberration**: A new post processing effect that simulates lenses that fail to focus light to a single point
- **Volumetric Fog Improvements**: "Fog volumes" that define where volumetric fog is rendered (and what form it takes), along with Point Lights and Spotlight compatibility
- **Order Independent Transparency**: A new opt-in transparency algorithm that improves the stability / quality of transparent objects as their distance from the camera changes
- **Improved Text Rendering**: We've switched to Cosmic Text for our text rendering, which significantly improves our ability to render text, especially for non-Latin-based languages that require font shaping and bidirectional text
- **Gamepads as Entities**: Gamepads are now represented as entities, making them much easier to interact with
- **UI Box Shadows**: Bevy UI nodes can now render configurable box shadows

Bevy 0.15 was prepared using our new **release candidate** process to help ensure that you can upgrade right away with peace of mind. We worked closely with both plugin authors and ordinary users to catch critical bugs, polish new features, and refine the migration guide. For each release candidate, we prepared fixes, [shipped a new release candidate on crates.io](https://crates.io/crates/bevy/versions?sort=date), let core ecosystem crates update, and listened closely for show-stopping problems. A huge thanks to [everyone who helped out](https://discord.com/channels/691052431525675048/1295069829740499015)! These efforts are a vital step towards making Bevy something that teams large and small can trust to work reliably.

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
