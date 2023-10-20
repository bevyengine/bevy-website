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

## Suspend and Resume on Android

<div class="release-feature-authors">authors: @mockersf</div>

On Android, applications are no longer crashing on suspend. Instead, they are paused, and no system
are running until the application is resumed.

<video controls><source src="suspend-resume.mp4" type="video/mp4"/></video>

Background tasks working in other threads, like playing audio, won't be stopped. When the
application will be suspended, a [`Lifetime`] event `Lifetime::Suspended` is sent, corresponding to
the [`onStop()`] callback. You should take care to pause tasks that shouldn't run in the background,
and resume them when receiving the `Lifetime::Resumed` event (corresponding to the [`onRestart()`]
callback).

```rust
fn handle_lifetime_events(
    mut lifetime_events: EventReader<Lifetime>,
    music_controller: Query<&AudioSink>,
) {
    for event in lifetime_events.read() {
        match event {
            // Upon receiving the `Lifetime::Suspended` event, the application has 1 frame before it is paused
            Lifetime::Suspended => music_controller.single().pause(),
            Lifetime::Resumed => music_controller.single().play(),
            // `Lifetime::Started` is the only other event for now, more to come in the next Bevy version
            _ => (),
        }
    }
}
```

[`Lifetime`]: https://docs.rs/bevy/0.12.0/bevy/window/enum.Lifetime.html
[`onStop()`]: https://developer.android.com/reference/android/app/Activity#onStop()
[`onRestart()`]: https://developer.android.com/reference/android/app/Activity#onRestart()

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:
