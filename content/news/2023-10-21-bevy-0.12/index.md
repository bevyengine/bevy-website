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

## Feature Name

<div class="release-feature-authors">authors: @author</div>

## AnimationPlayer API Improvements

<div class="release-feature-authors">authors: @devinleamy</div>

The `AnimationPlayer` now has new methods for controlling playback, and utilities for checking
if an animation is playing or completed, and getting its `AnimationClip` handle.

`set_elapsed` and has been removed in favor of `seek_to`. `elapsed` now
returns the actual elapsed time and is not affected by the animation speed. `stop_repeating` have been removed
in favor of `set_repeat(RepeatAnimation::Never)`.

```rust
let mut player = q_animation_player.single_mut();
// Check if an animation is complete.
if player.is_finished() {
    // Set the playback mode.
    player.set_repeat(RepeatAnimation::Forever);
    player.set_repeat(RepeatAnimation::Never);
    player.set_repeat(RepeatAnimation::Count(4));
}
// Get a handle to the playing AnimationClip.
let clip_handle = player.animation_clip();
// Seek to 1s in the current clip.
player.seek_to(1.0);
```

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:
