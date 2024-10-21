<!-- Add `core` and `alloc` over `std` Lints -->
<!-- https://github.com/bevyengine/bevy/pull/15281 -->

Bevy relies heavily on Rust's [standard library](https://doc.rust-lang.org/std/), making it challenging to use on embedded, niche
platforms, and even certain consoles. But what if that _wasn't_ the case?

Bevy's maintainers have undertaken a new initiative to challenge the reliance on the standard
library, with the eventual goal of providing a [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html)
compatible subset of Bevy which could be used on a much wider range of platforms.

The first step of this journey is the addition of a new set of lints:

* [`std_instead_of_core`](https://rust-lang.github.io/rust-clippy/master/index.html#std_instead_of_core)
* [`std_instead_of_alloc`](https://rust-lang.github.io/rust-clippy/master/index.html#std_instead_of_alloc)
* [`alloc_instead_of_core`](https://rust-lang.github.io/rust-clippy/master/index.html#alloc_instead_of_core)

For those unfamiliar with `no_std` Rust, the standard library, `std`, gets a lot of its functionality
from two smaller crates, [`core`](https://doc.rust-lang.org/core/) and [`alloc`](https://doc.rust-lang.org/alloc/).
The `core` crate is available on every Rust target with very few exceptions, providing the
fundamental infrastructure that the Rust language relies on, such as iterators, `Result`, and many more.
While the `alloc` crate provides access to allocation-related functionality, such as `Vec`, `Box`,
and `String`.

Rust has built-in support for platforms with a [three tiered policy](https://doc.rust-lang.org/rustc/platform-support.html),
where tier 1 is guaranteed to work and will always provide the `std` crate, and tiers 2 and 3 _may_
have the `std` crate, but often do not.
The reason for this is some platforms simply don't support the features the `std` crate requires,
such as a filesystem, networking, or threading.

But why should Bevy care about these platforms? The reason is when a new platform is added to Rust,
it is often lacking tier 1 support.
Even modern consoles such as the Nintendo Switch, PlayStation 5, or Xbox Series don't have tier 1
support due to non-disclosure agreements and platform specifics.
Adding `no_std` support to Bevy will make it easier for commercial teams developing for these platforms to get started and stay up to date.

Beyond just the current generation of consoles, there is a vibrant community of embedded and retro
enthusiasts developing for platforms that may never support the standard library.
Crates such as [`agb`](https://crates.io/crates/agb) and [`psx`](https://crates.io/crates/psx) provide
support for developing games on the GameBoy Advance and PlayStation One respectively.
With `no_std` support in Bevy, users may be able to leverage the wider Rust ecosystem to run their
software on whole new platforms on the frontiers of support.

These lints ensure that when a Bevy maintainer uses functionality from `std` that is _actually_ from
`core` or `alloc`, they are notified and can use the more fundamental crate instead.
Enabling these lints required a lot of small changes across the whole Bevy library, but doing so
has revealed the true extent to which Bevy relies on the standard library.

We're still a while away from true `no_std` support in Bevy, but the first few changes have already
been accepted, with many more lined up for the next release in 0.16.

If this work sounds interesting and you'd like to learn more, check out the
[`no_std` tracking issue](https://github.com/bevyengine/bevy/issues/15460) on GitHub, where you can
find a list of pull requests, and even prototypes of Bevy running in `no_std` environments.
