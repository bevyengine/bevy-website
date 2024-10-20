<!-- Add `core` and `alloc` over `std` Lints -->
<!-- https://github.com/bevyengine/bevy/pull/15281 -->

Bevy is entirely dependent on the standard library, making it challenging to use on embedded, niche
platforms, and even certain consoles. But what if that _wasn't_ the case?

Bevy's maintainers have undertaken a new initiative to challenge the reliance on the standard
library, with the eventual goal of providing a [`no_std`](https://docs.rust-embedded.org/book/intro/no-std.html)
compatible subset of Bevy which could be used on a much wider range of platforms.

The first step of this journey is the addition of a new set of lints:

* `std_instead_of_core`
* `std_instead_of_alloc`
* `alloc_instead_of_core`

For those unfamiliar with `no_std` Rust, the standard library, `std`, gets a lot of its functionality
from two smaller crates, [`core`](https://doc.rust-lang.org/core/) and [`alloc`](https://doc.rust-lang.org/alloc/).
The `core` crate is available on every Rust target with very few exceptions, providing the
fundamental infrastructure that the Rust language relies on, such as iterators, `Result`, and many more.
While the `alloc` crate provides access to allocation-related functionality, such as `Vec`, `Box`,
and `String`. It's marginally harder to setup on a new platform, but still relatively straightforward.

These lints ensure that when a Bevy maintainer uses functionality from `std` that is _actually_ from
`core` or `alloc`, they are notified and can use the more fundamental crate instead.
Enabling these lints required a lot of small changes across the whole Bevy library, but doing so
has revealed the true extent to which Bevy relies on the standard library.

We're still a while away from true `no_std` support in Bevy, but the first few changes have already
been accepted, with many more lined up for the next release in 0.16.
Rust as an ecosystem has _excellent_ support for these atypical platforms, meaning there is a chance
that one day you Bevy project could be run on a the [GameBoy Advance](https://crates.io/crates/agb),
the [PlayStation One](https://crates.io/crates/psx), or any other number of previously unthinkable platforms.

If this work sounds interesting and you'd like to learn more, check out the [`no_std` tracking issue](https://github.com/bevyengine/bevy/issues/15460)
on GitHub, where you can find a list of pull requests, and even prototypes of Bevy running in `no_std`
environments.
