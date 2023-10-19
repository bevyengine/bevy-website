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

## Unified `Time`

<div class="release-feature-authors">authors: @nakedible @maniwani @alice-i-cecile</div>

Bevy 0.12 brings two major QoL improvements to `FixedUpdate`.

- `Time` now returns the contextually correct values for systems running in `FixedUpdate`. (As such, `FixedTime` has been removed.)
- `FixedUpdate` can no longer snowball into a "death spiral" (where the app freezes because `FixedUpdate` steps are enqueued faster than it can run them).

The `FixedUpdate` schedule and its companion `FixedTime` resource were introduced in Bevy 0.10, and it soon became apparent that `FixedTime` was lacking. It had completely different methods and it didn't track "total time elapsed" like `Time` did, to name a few examples. Having two different APIs also meant you had to write systems to specifically support "fixed timestep" or "variable timestep" and not both. It was desirable to not have this split as it can lead to incompatibilities between plugins down the road (which is sometimes the case with plugins in other game engines).

Now, you can just write systems that read `Time` and schedule them in either context.

```rust
// This system will see a constant delta time if scheduled in `FixedUpdate` or
// a variable delta time if scheduled in `Update`.
fn integrate_velocity(
    mut query: Query<(&mut Transfrom, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
```

Most systems should continue to use `Time`, but behind the scenes, the methods from previous APIs have been refactored into four clocks:

- `Time<Real>`
- `Time<Virtual>`
- `Time<Fixed>`
- `Time<()>`

`Time<Real>` measures the true, unedited frame and app durations. For diagnostics/profiling, use that one. It's also used to derive the others. `Time<Virtual>` can be sped up, slowed down, and paused, and `Time<Fixed>` chases `Time<Virtual>` in fixed increments. Lastly, `Time<()>` is automatically overwritten with the current value of `Time<Fixed>` or `Time<Virtual>` upon entering or exiting `FixedUpdate` (and will update their value on the next swap). When a system borrows `Time`, it actually borrows `Time<()>`.

Try the new `time` example to get a better feel for these resources.

The fix for the windup problem was limiting how much `Time<Virtual>` can advance from a single frame. This then limits how many times `FixedUpdate` can be queued for the next frame, and so things like frame lag or your computer waking up from a long sleep can no longer cause a death spiral. So now, the app won't freeze, but things happening in `FixedUpdate` will appear to slow down since it'll be running at a temporarily reduced rate.

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:
