+++
title = "Bevy 0.12"
date = 2023-10-21
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.12** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.12**, check out our [0.11 to 0.12 Migration Guide](/learn/migration-guides/0.11-0.12/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

## AccessKit Integration Improvements

<div class="release-feature-authors">authors: @ndarilek</div>

Bevy 0.10's [AccessKit](https://accesskit.dev) integration made it incredibly easy for the engine to take the lead and push updates to the accessibility tree. But as any good dance partner knows, sometimes it's best not to lead but to follow.

This release adds the `ManageAccessibilityUpdates` resource which, when set to `false`, stops the engine from updating the tree on its own. This paves the way for third-party UIs with Bevy and AccessKit integration to send updates directly to Bevy. When the UI is ready to return control, `ManageAccessibilityUpdates` is set to `true` Bevy picks up where it left off and starts sending updates again.

AccessKit itself was also simplified, and this release capitalizes on that to shrink the surface area of our integration. If you're curious about how things work internally or want to help, the `bevy_a11y` crate is now more approachable than ever.

## Feature Name

<div class="release-feature-authors">authors: @author</div>

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:
