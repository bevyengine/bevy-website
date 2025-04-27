<!-- Add `no_std` support to `bevy` -->
<!-- https://github.com/bevyengine/bevy/pull/17955 -->

Bevy now has support for `no_std` targets, allowing it to be used on a much wider range of platforms.

Early reports from users have shown Bevy working on bare-metal desktops, embedded devices, and even retro consoles such as the GameBoy Advance:

<video controls loop aria-label="A purple GameBoy Advance running a game made using Bevy"><source  src="bevy-gba.mp4" type="video/mp4"/></video>

Credit to [Chris Biscardi](https://www.youtube.com/@chrisbiscardi) for creating this awesome demo!

Bevy `no_std` support has been [discussed] going back over 4 years, but was initially dropped to avoid the added complexity managing `no_std` support can bring.
To be `no_std` compatible, your crate and _all_ of its dependencies must also be `no_std`.
Coordinating that kind of support across over a hundred dependencies was just not feasible, let alone losing access to Rust's standard library.

Since then, Rust's support for `no_std` has evolved dramatically with support for critical APIs such as [`Error`] coming in [Rust 1.81].
Starting with tracking issue [#15460] and a [`no_std` Working Group], Bevy's various crates were individually made `no_std` compatible where possible.
To aid this effort, [`bevy_platform`] was developed with the goal of providing opinionated alternatives to `std` items.

This effort reached a substantial milestone during the development of **Bevy 0.16**: support for `no_std` in our main `bevy` crate.
To use Bevy on a `no_std` platform, simply disable default features and use Bevy just like any other `no_std` dependency.

```toml
[dependencies]
bevy = { version = "0.16", default-features = false }
```

Note that not all of Bevy's features are compatible with `no_std` yet.
Rendering, audio, and assets are notable missing APIs that you will need to find an alternative for that's suitable for your platform.
But, Bevy's powerful [`Plugin`] system allows the community to build back support for that functionality for their particular platforms.

For those community members working on libraries for Bevy, we encourage you to try out `no_std` support if you can!
There's a new [`no_std` library] example which demonstrates how to make a crate that is compatible with `std` and `no_std` users, with detailed comments and advice.
During the release candidate period, quite a few libraries have successfully experimented with `no_std` support, such as [`bevy_rand`] and [`bevy_replicon`].

Determining what `no_std` targets support Bevy is still a work in progress. If you have an unusual platform you'd like to try getting Bevy working on, check out the [`#unusual-platforms`] channel on Bevy's Discord server for advice!

[`Error`]: https://doc.rust-lang.org/stable/core/error/trait.Error.html
[#15460]: https://github.com/bevyengine/bevy/issues/15460
[`no_std` Working Group]: https://discord.com/channels/691052431525675048/1303128171352293410
[`bevy_platform`]: https://crates.io/crates/bevy_platform/
[`#unusual-platforms`]: https://discord.com/channels/691052431525675048/1284885928837517432
[discussed]: https://github.com/bevyengine/bevy/discussions/705
[`Plugin`]: https://docs.rs/bevy/latest/bevy/app/trait.Plugin.html
[Rust 1.81]: https://releases.rs/docs/1.81.0/#stabilized-apis
[`no_std` library]: https://github.com/bevyengine/bevy/tree/main/examples/no_std/library
[`bevy_replicon`]: https://github.com/projectharmonia/bevy_replicon/tree/bevy-0.16-dev
[`bevy_rand`]: https://github.com/Bluefinger/bevy_rand
[`bevy_transform_interpolation`]: https://github.com/Jondolf/bevy_transform_interpolation
