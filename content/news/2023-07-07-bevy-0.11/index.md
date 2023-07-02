+++
title = "Bevy 0.11"
date = 2023-07-07
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.11** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.11**, check out our [0.10 to 0.11 Migration Guide](/learn/migration-guides/0.10-0.11/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Gamepad Rumble API**: an ECS-friendly way of making controllers rumble

## Gamepad Rumble API

<div class="release-feature-authors">authors: @johanhelsing, @nicopap</div>

You can now use the `EventWriter<GamepadRumbleRequest>` system parameter to
trigger controllers force-feedback motors.

[`gilrs`], the crate bevy uses for gamepad support, allows controlling
force-feedback motors. Sadly, there were no easy way of accessing the
force-feedback API in bevy without tedious bookkeeping.

Now bevy has the `GamepadRumbleRequest` event to do just that.

```rust
fn rumble_system(
    gamepads: Res<Gamepads>,
    mut rumble_requests: EventWriter<GamepadRumbleRequest>,
) {
    for gamepad in gamepads.iter() {
        rumble_requests.send(GamepadRumbleRequest::Add {
            gamepad,
            duration: Duration::from_secs(5),
            intensity: GamepadRumbleIntensity::MAX,
        });
    }
}
```

The `GamepadRumbleRequest::Add` event triggers a force-feedback motor,
controlling how long the vibration should last, the motor to activate,
and the vibration strength. `GamepadRumbleRequest::Stop` immediately stops all
motors.

[`gilrs`]: https://crates.io/crates/gilrs

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author
