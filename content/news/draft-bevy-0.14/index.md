+++
title = "Bevy 0.14"
date = 2024-05-17
[extra]
public_draft = 1188
+++

Thanks to **TODO** contributors, **TODO** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.14** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.14**, check out our [0.13 to 0.14 Migration Guide](/learn/migration-guides/0-13-to-0-14/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **Animation blending:** our new low-level animation graph adds support for animation blending, and sets the stage for first- and third-party graphical, asset-driven animation tools.
- **Shiny 3D rendering features:** meshlets, hierarchical levels of detail, depth of field, god rays, motion blur and more!
- **Better colors:** type-safe colors make it clear which color space you're operating in, and offer an awesome array of useful methods.
- **Observers and hooks:** automatically respond to component addition and removal as we lay the foundations for fast, reliable relations between entities.
- **Computed states and substates:** modeling complex app state is a breeze with these type-safe extensions to our `States` abstraction.
- **Rounded corners:** rounding off one of `bevy_ui`'s roughest edges, you can now procedurally set the corner radius on your UI elements.

<!-- more -->

{{ release_notes(version="0.14") }}

## What's Next?

Sure this release was great, but what does the future hold for Bevy?
Peering deep into the mists of time (predictions are *extra* hard when your team is almost all volunteers!), we can see some exciting work taking shape:

- **Better scenes:** Scenes are one of Bevy's core building blocks: designed to be a powerful tool for saving games, authoring levels and creating reusable game objects, whether they're a radio button widget or a monster. More features and a unified syntax between assets and code should unblock progress on a UI widget abstraction, tools less boilerplates. Check out the [design doc](TODO) for more information.
- **Relations please?:** Relations (a first-class feature for linking entities together) is wildly desired but remarkably complex, driving features and refactors to our ECS internals. The [working group](https://discord.com/channels/691052431525675048/1237010014355456115) has been patiently laying out what we need to do and why in this [RFC](https://github.com/bevyengine/rfcs/pull/79).
- **Better audio:** Bevy's built-in audio solution has never really hit the right notes. The [Better Audio working group](https://discord.com/channels/691052431525675048/1236113088793677888) is plotting a path forward and exploring [ECS-ified interface](https://github.com/SolarLiner/bevy-kira-components) to the popular [`kira`](https://crates.io/crates/kira) audio backend.
- **Contributing book:** Our documentation on how to contribute is scattered to the four corners of our repositories. By gathering this together, the [Contributing Book working group](https://discord.com/channels/691052431525675048/1236112637662724127) hopes to make it easier to discover and maintain.
- **Curve abstraction:** Curves come up all of the time in game dev, and the mathmagicians that make up the [Curve Crew](https://discord.com/channels/691052431525675048/1236110755212820581) are [designing a trait](https://github.com/bevyengine/rfcs/pull/80) to unify and power them.
- **Better text:** our existing text solution isn't up to the demands of modern UI. We're looking at replacing it with a better solution.
- **A unified view on dev tools:** In 0.14, we've added a stub `bevy_dev_tools` crate: a place for tools and overlays that speed up game development such as performance monitors, fly cameras, or in-game commands to spawn game objects. We're working on adding more tools, and creating a [dev tool abstraction](https://github.com/bevyengine/rfcs/pull/77). This will give us a unified way to enable/disable, customise and group this grab bag of tools into toolboxes to create something like Quake console or VSCode Command Palette with tools from around the ecosystem.
- **A modular, maintainable render graph:** Bevy's existing rendering architecture is already quite good at providing reusable renderer features like `RenderPhases`, batching, and draw commands. However, the render graph itself is one remaining pain point: since it's distributed across many files the control flow is hard to understand, and its heavy use of ECS resources for passing around rendering data actively works against modularity. While the exact design hasn't been finalized (and feedback is very welcome!), we've been actively working to [redesign the render graph](https://github.com/bevyengine/bevy/pull/13397) in order to build up to a larger refactor of the renderer towards modularity and ease of use.

{{ support_bevy() }}
{{ contributors(version="0.14") }}
{{ changelog(version="0.14")}}
