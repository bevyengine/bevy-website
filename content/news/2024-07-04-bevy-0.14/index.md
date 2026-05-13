+++
title = "Bevy 0.14"
date = 2024-07-04
[extra]
image = "cover.jpg"
show_image = true
image_subtitle = "A forested scene illustrating Bevy's new volumetric fog, depth of field, and screen-space reflections"
image_subtitle_link = "https://github.com/IceSentry/bevy_forest_scene"
+++

Thanks to **256** contributors, **993** pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.14** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevy.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.14**, check out our [0.13 to 0.14 Migration Guide](/learn/migration-guides/0-13-to-0-14/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **Virtual Geometry**: Preprocess meshes into "meshlets", enabling efficient rendering of huge amounts of geometry
- **Sharp Screen Space Reflections**: Approximate real time raymarched screen space reflections
- **Depth of Field**: Cause objects at specific depths to go "out of focus", mimicking the behavior of physical lenses
- **Per-Object Motion Blur**: Blur objects moving fast relative to the camera
- **Volumetric Fog / Lighting**: Simulates fog in 3d space, enabling lights to produce beautiful "god rays"
- **Filmic Color Grading**: Fine tune tonemapping in your game with a complete set of filmic color grading tools
- **PBR Anisotropy**: Improve rendering of surfaces whose roughness varies along the tangent and bitangent directions of a mesh, such as brushed metal and hair
- **Auto Exposure**: Configure cameras to dynamically adjust their exposure based on what they are looking at
- **PCF for Point Lights**: Smooth out point light shadows, improving their quality
- **Animation blending:** Our new low-level animation graph adds support for animation blending, and sets the stage for first- and third-party graphical, asset-driven animation tools.
- **ECS Hooks and Observers:** Automatically (and immediately) respond to arbitrary events, such as component addition and removal
- **Better colors:** type-safe colors make it clear which color space you're operating in, and offer an awesome array of useful methods.
- **Computed states and substates:** Modeling complex app state is a breeze with these type-safe extensions to our `States` abstraction.
- **Rounded corners:** Rounding off one of `bevy_ui`'s roughest edges, you can now procedurally set the corner radius on your UI elements.

For the first time, Bevy 0.14 was prepared using a **release candidate** process to help ensure that you can upgrade right away with peace of mind.
We've worked closely with both plugin authors and ordinary users to catch critical bugs, round the rough corners off our new features, and refine the migration guide.
As we prepared fixes, we've [shipped new release candidates on crates.io](https://crates.io/crates/bevy/versions?sort=date), letting core ecosystem crates update and listening closely for show-stopping problems.
Thank you so much to [everyone who helped out](https://discord.com/channels/691052431525675048/1239930965267054623): these efforts are a vital step towards making Bevy something that teams large and small can trust to work reliably.

<!-- more -->

{{ release_notes(version="0.14") }}

## What's Next?

The features above may be great, but what else does Bevy have in flight?
Peering deep into the mists of time (predictions are _extra_ hard when your team is almost all volunteers!), we can see some exciting work taking shape:

- **Better Scenes:** Scenes are one of Bevy's core building blocks: designed to be a powerful tool for authoring levels and creating reusable game objects, whether they're a radio button widget or a monster. We're working on a new scene system with a new syntax that will make defining scenes in assets _and_ in code more powerful and more pleasant. You can check out the(now slightly old) [project kickoff discussion](https://github.com/bevyengine/bevy/discussions/9538) for more information. We're also very close to putting out a design document outlining our plans and the current state of the implementation.
- **ECS Relations:** Relations (a first-class feature for linking entities together) is wildly desired but remarkably complex, driving features and refactors to our ECS internals. The [working group](https://discord.com/channels/691052431525675048/1237010014355456115) has been patiently laying out what we need to do and why in this [RFC](https://github.com/bevyengine/rfcs/pull/79).
- **Better Audio:** Bevy's built-in audio solution has never really hit the right notes. The [Better Audio working group](https://discord.com/channels/691052431525675048/1236113088793677888) is plotting a path forward.
- **Contributing Book:** Our documentation on how to contribute is scattered to the four corners of our repositories. By gathering this together, the [Contributing Book working group](https://discord.com/channels/691052431525675048/1236112637662724127) hopes to make it easier to discover and maintain.
- **Curve Abstraction:** Curves come up all of the time in game dev, and the mathmagicians that make up the [Curve Crew](https://discord.com/channels/691052431525675048/1236110755212820581) are [designing a trait](https://github.com/bevyengine/rfcs/pull/80) to unify and power them.
- **Better Text:** our existing text solution isn't up to the demands of modern UI. The "Lorem Ipsum" working group is [looking into](https://discord.com/channels/691052431525675048/1248074018612051978) replacing it with a better solution.
- **A Unified View on Dev Tools:** In 0.14, we've added a stub `bevy_dev_tools` crate: a place for tools and overlays that speed up game development such as performance monitors, fly cameras, or in-game commands to spawn game objects. We're working on adding more tools, and creating a [dev tool abstraction](https://github.com/bevyengine/rfcs/pull/77). This will give us a unified way to enable/disable, customize and group this grab bag of tools into toolboxes to create something like Quake console or VSCode Command Palette with tools from around the ecosystem.
- **Bevy Remote Protocol:** Communicating with actively running Bevy games is an incredibly powerful tool for building editors, debuggers and other tools. [We're developing](https://github.com/bevyengine/bevy/pull/13563) a reflection-powered protocol to create a solution that's ready to power a whole ecosystem.
- **A Modular, Maintainable Render Graph:** Bevy's existing rendering architecture is already quite good at providing reusable renderer features like `RenderPhases`, batching, and draw commands. However, the render graph interface itself is one remaining pain points. Since it's distributed across many files the control flow is hard to understand, and its heavy use of ECS resources for passing around rendering data actively works against modularity. While the exact design hasn't been finalized (and feedback is very welcome!), we've been actively working to [redesign the render graph](https://github.com/bevyengine/bevy/pull/13397) in order to build up to a larger refactor of the renderer towards modularity and ease of use.

{{ support_bevy() }}
{{ contributors(version="0.14") }}
{{ changelog(version="0.14")}}
