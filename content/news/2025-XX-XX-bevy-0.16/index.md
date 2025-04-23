+++
title = "Bevy 0.16"
date = 2024-12-31 # TODO: fix date
[extra]
image = "cover.png"
show_image = true
image_subtitle = "TODO"
image_subtitle_link = "https://todo.example.com"
public_draft = 2008
status = 'hidden'
+++

Thanks to **261** contributors, **1244** pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.16** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.16**, check out our [0.15 to 0.16 Migration Guide](/learn/migration-guides/0-15-to-0-16/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **Unified error handling:** Sick of impossible to debug panics in your Bevy code? Us too! We've dramatically improved debuggability with the new `track_location` feature flag, and we've made it easy to return and handle errors across the entire ECS.
- **GPU-driven rendering:** GPUs are fast: we should make sure that we keep them busy! We've done a ton of performance-oriented work that should make Bevy dramatically faster on big, complex scenes.
- **Relations:** one of the hottest ECS features is finally here: allowing you to easily and robustly model and work with entity-entity connections. Some caveats apply, but we're excited to get a simple and robust solution to users today.
- **`no_std` is a reality:** `bevy` itself and a ton of our subcrates no longer rely on Rust's standard library, letting you use the same engine on everything from a modern gaming rig to a Gameboy Advance.
- **Procedural atmosphere scattering system:** allows for simulating realistic physically-based Earth-like sky at any time of day at a low performance cost. It is a multi pass advanced rendering technique and it's achieved using WebGPU compute shaders that run on both web and native platforms.
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
- **Unified volumetrics system:** God rays, fogs, cascading shadow maps, and atmospheric scattering: there's a huge a number of rendering features that fundamentally care about the optical properties of volumes of open air (or water!). We're hoping to unify and extend these features for easier to use, more beautiful physically-based rendering.
- **More game-focused examples:** New users continue to flock to Bevy, and need up-to-date learning materials. Our API-focused approach to examples isn't enough: we need to start demonstrating how to use Bevy to do common game dev tasks like making an inventory, saving user preferences or placing structures on a map.

{{ support_bevy() }}

{{ contributors(version="0.16") }}

For those interested in a complete changelog, you can see the entire log (and linked pull requests) via the [relevant commit history](https://github.com/bevyengine/bevy/compare/v0.15.0...v0.16.0-rc.5).
