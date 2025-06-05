+++
title = "Bevy 0.16"
date = 2025-04-24 
[extra]
image = "planet.jpg"
show_image = true
image_subtitle = "A planet from EmbersArc's in-development spaceflight simulation game, rendered with custom shaders in Bevy"
image_subtitle_link = "https://bsky.app/profile/embersarc.bsky.social"
+++

Thanks to **261** contributors, **1244** pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.16** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevy.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.16**, check out our [0.15 to 0.16 Migration Guide](/learn/migration-guides/0-15-to-0-16/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **GPU-Driven Rendering:** Bevy now does even more rendering work on the GPU (where possible), making Bevy dramatically faster on big, complex scenes.
- **Procedural Atmospheric Scattering:** Simulate realistic physically-based Earth-like sky at any time of day at a low performance cost.
- **Decals**: Dynamically layer textures onto rendered meshes.
- **Occlusion Culling**: Improve performance by not rendering objects that are obscured by other objects.
- **ECS Relationships:** One of the hottest ECS features is finally here: allowing you to easily and robustly model and work with entity-entity connections. Some caveats apply, but we're excited to get a simple and robust solution to users today.
- **Improved Spawn API:** Spawning entity hierarchies is now significantly easier!
- **Unified Error Handling:** Bevy now has first class error handling support, making it easy, flexible, and ergonomic, while also making debugging easier!
- **`no_std` Support:** `bevy` itself and a ton of our subcrates no longer rely on Rust's standard library, letting you use the same engine on everything from a modern gaming rig to a Gameboy Advance.
- **Faster Transform Propagation:** We've dramatically improved the performance of transform propagation for more objects at once, especially if they are static.
<!-- more -->

{{ release_notes(version="0.16") }}

## What's Next?

The features above may be great, but what else does Bevy have in flight?
Peering deep into the mists of time (predictions are _extra_ hard when your team is almost all volunteers!), we can see some exciting work taking shape:

- **A revamped observers API:** Observers are incredibly popular, but come with some weird quirks. We're looking to smooth those out, and make them the easiest way to write one-off logic for UI.
- **Resources-as-entities:** Sure would be nice if hooks, observers, relations and more worked with resources. Rather than duplicating all of the code, we'd like to [make them components on singleton entities](https://github.com/bevyengine/bevy/pull/17485) under the hood.
- **A .bsn file format and bsn! macro:** With the foundations laid (required components, improved spawning and relations!), it's time to build out the terse and robust Bevy-native scene format (and matching macro) described in [bevy#14437](https://github.com/bevyengine/bevy/discussions/14437).
- **Light textures:** Also known as "light cookies", these are great for everything from dappled sunlight to shadow boxing.
- **NVIDIA Deep Learning Super Sampling:** DLSS is a neural-net powered approach to temporal anti-aliasing and upscaling for NVIDIA RTX GPUs. We're working on integrating DLSS into Bevy to provide a cheaper and higher quality anti-aliasing solution than Bevy's current TAA (on supported platforms).
- **Unified volumetrics system:** God rays, fogs, cascading shadow maps, and atmospheric scattering: there's a huge number of rendering features that fundamentally care about the optical properties of volumes of open air (or water!). We're hoping to unify and extend these features for easier to use, more beautiful physically-based rendering.
- **Ray-tracing foundations:** Hardware-accelerated ray-tracing is all the rage, and with `wgpu`'s help we're ready to start making the first steps, walking towards a world of dynamic ray-traced global illumination.
- **More game-focused examples:** New users continue to flock to Bevy, and need up-to-date learning materials. Our API-focused approach to examples isn't enough: we need to start demonstrating how to use Bevy to do common game dev tasks like making an inventory, saving user preferences or placing structures on a map.

{{ support_bevy() }}

{{ contributors(version="0.16") }}

For those interested in a complete changelog, you can see the entire log (and linked pull requests) via the [relevant commit history](https://github.com/bevyengine/bevy/compare/v0.15.0...v0.16.0-rc.5).
