+++
title = "Bevy 0.10"
date = 2023-03-04
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), I'm happy to announce the **Bevy 0.10** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.10**, check out our [0.9 to 0.10 Migration Guide](/learn/book/migration-guides/0.9-0.10/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Headliner Feature**: Description here.

## Section Template

<div class="release-feature-authors">authors: @Foo, @Bar</div>

Description here.

## Spatial Audio

<div class="release-feature-authors">authors: @mockersf, @DGriffin91, @harudagondi, @alice-i-cecile</div>

The library Bevy uses for audio, [`rodio`], contains support for spatial audio. In this version, we now support spatial audio (with certain caveats, like no HRTF and no first class support for `Emitter` and `Listener` components).

Interestingly, during the development of this specific feature, `@harudagondi` found a [bug][reverse-channels-bug] where the audio channels reverse when running the app in either debug or release mode. This turns out to be a `rodio` issue, and this also affects previous versions of Bevy. Thanks to `@dis-da-moe`, the bug has been [fixed upstream][rodio-pr]. See the linked PR for interesting details about audio programming quirks and performance issues.

You can now have spatial audio in your game! Clone the `bevy` repository and invoke `cargo run --example spatial_audio_3d --release` in the command line for a showcase of 3D spatial audio in Bevy.

[`rodio`]: https://crates.io/crates/rodio
[reverse-channels-bug]: https://github.com/RustAudio/rodio/issues/444
[rodio-pr]: https://github.com/RustAudio/rodio/pull/455

## Custom Audio Sources

<div class="release-feature-authors">authors: @dis-da-moe</div>

Bevy supports custom audio sources through the [`Decodable`] trait, but the way to register to the bevy app is very boilerplatey and sparsely documented. In **Bevy 0.10**, a new extension trait for `App` is added and the documentation for [`Decodable`] has vastly improved.

As such, instead of doing this:

```rust
struct MyCustomAudioSource { /* ... */ }

app.add_asset::<MyCustomAudioSource>()
    .init_resource::<Audio<MyCustomAudioSource>>()
    .init_resource::<AudioOutput<MyCustomAudioSource>>()
    .add_system(play_queued_audio_system::<MyCustomAudioSource>.in_base_set(CoreSet::PostUpdate))
```

You only have to do this:

```rust
app.add_audio_source::<MyCustomAudioSource>()
```

Much cleaner!

[`Decodable`]: https://docs.rs/bevy_audio/latest/bevy_audio/trait.Decodable.html

## `Ref<T>` Queries

<div class="release-feature-authors">authors: @Guvante, @JoJoJet</div>

Since Bevy 0.1, `Mut<T>` has been used to enable change detection (along with related types like `ResMut<T>`). It's a simple wrapper type that provides mutable access to a component alongside its change tick metadata, automatically marking a change when the value is mutated.

In **Bevy 0.10**, the change detection family has grown with `Ref<T>`, the immutable variant of `Mut<T>`. Like its mutable sibling, it allows you to react to changes made outside of the current system.

```rust
use bevy::prelude::*;

fn inspect_changes_system<T: Component + Debug>(q: Query<Ref<T>>) {
    // Iterate over each component of type `T` and log its changed status.
    for val in &q {
        if val.is_changed() {
            println!("Value `{val:?}` was last changed at tick {}.", val.last_changed());
        } else {
            println!("Value `{val:?}` is unchanged.");
        }
    }
}
```

We are also deprecating `ChangeTrackers<T>`, which is the old way of inspecting a component's change ticks. This type will be removed in the next version of Bevy.

## Android Support

<div class="release-feature-authors">authors: @mockersf, @slyedoc</div>

![Android emulator running Bevy](android%20emulator.png)

Bevy now runs out of the box on Android on more devices. This was unlocked by waiting for the [`Resumed`](https://docs.rs/winit/0.28/winit/event/enum.Event.html#variant.Resumed) event to create the window instead of doing it on startup, matching the [`onResume()`](https://developer.android.com/guide/components/activities/activity-lifecycle#onresume) callback on Android.

To follow the recommendations on the [`Suspended`](https://docs.rs/winit/0.28/winit/event/enum.Event.html#variant.Suspended) event, Bevy will now exit on receiving that event. This is a temporary solution until Bevy is able to recreate rendering resources when being resumed.

Please test on your devices and report successes or issues you may encounter! There is a known issue around touch position on some devices with software buttons, as winit doesn't expose [yet](https://github.com/rust-windowing/winit/issues/2308) the inset size, only the inner size.

As this brings Bevy closer to full support of Android, there isn't a need anymore for separated examples for Android and iOS. They have been regrouped in one ["mobile" example](https://github.com/bevyengine/bevy/tree/v0.10.0/examples/mobile), and the instructions updated ([for Android](https://github.com/bevyengine/bevy/tree/v0.10.0/examples#android) and [for iOS](https://github.com/bevyengine/bevy/tree/v0.10.0/examples#ios)).

![iOS emulator running Bevy](ios%20emulator.png)

## What's Next?

* **Some Work In Progress Feature**: Description here.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](Bevy ) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

A huge thanks to the **X contributors** that made this release (and associated docs) possible! In random order:

* @Foo

## Full Change Log
