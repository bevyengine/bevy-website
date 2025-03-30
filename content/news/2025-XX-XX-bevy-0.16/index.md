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

Thanks to **??** contributors, **???** pull requests, community reviewers, and our [**generous donors**](/donate), we're happy to announce the **Bevy 0.16** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.16**, check out our [0.15 to 0.16 Migration Guide](/learn/migration-guides/0-15-to-0-16/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

- **Relations:** one of the hottest ECS features is finally here: allowing you to easily and robustly model and work with entity-entity connections.
- **GPU-driven rendering:** GPUs are fast: we should make sure that we keep them busy! We've done a ton of performance-oriented work that should make Bevy dramatically faster on big, complex scenes.
- **`no_std` is a reality:** `bevy` itself and a ton of our subcrates no longer rely on Rust's standard library, letting you use the same engine on everything from a modern gaming rig to a Gameboy Advance.

<!-- more -->

{{ release_notes(version="0.16") }}

## What's Next?

The features above may be great, but what else does Bevy have in flight?
Peering deep into the mists of time (predictions are _extra_ hard when your team is almost all volunteers!), we can see some exciting work taking shape:

- **A revamped observers API:** Observers are incredibly popular, but come with some weird quirks. Why is `OnAdd` not generic? What does that `B: Bundle` generic even _do_? Please, I want to add observers in my entity spawning to write little callbacks!
- **Resources-as-entities:** Sure would be nice if hooks, observers, relations and more worked with resources. Rather than duplicating all of the code, we'd like to [make them components on singleton entities](https://github.com/bevyengine/bevy/pull/17485) under the hood.
- **A .bsn file format and bsn! macro:** With the foundations laid (required components, improved spawning and relations!), it's time to build out the terse and robust Bevy-native scene format (and matching macro) described in [bevy#14437](https://github.com/bevyengine/bevy/discussions/14437).
- **More game-focused examples:** New users continue to flock to Bevy, and need up-to-date learning materials. Our API-focused approach to examples isn't enough: we need to start demonstrating how to use Bevy to do common game dev tasks like making an inventory, saving user preferences or placing structures on a map.

{{ support_bevy() }}

TODO: add  contributors
TODO: add changelog
