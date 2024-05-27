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

<!-- TODO What's next -->

{{ support_bevy() }}
{{ contributors(version="0.14") }}
{{ changelog(version="0.14")}}
