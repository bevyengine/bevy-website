+++
title = "Project Organization"
insert_anchor_links = "right"
[extra]
weight = 1
+++

As your Bevy project grows, storing it all in a single `main.rs` file will become frustrating!
But how should you split it apart, and what are your tools for doing so?

Thankfully, the Bevy community has answers to these questions:

- split your code based on the "domain" it covers
  - think about where you might want to reuse, or selectively disable bits of functionality
  - code that changes together should live together
  - splits like "world generation" and "combat" and "main menu" are great
  - don't organize your modules by e.g. "components" and "systems"!
- don't try to get it right from the very start
  - split things up gradually, as areas become complex and unwieldy
  - you'll have a much better idea of the shape of your program then
  - and it stops you from wasting time at the start of your project
- take advantage of Rust's modules, crates and visibility to organize your code
  - be mindful of [orphan rules] when architecting multi-crate projects
- one [plugin] per module, and one plugin per crate (which may nest multiple plugins) is a good default
  - placing these at the top of your files can act as a simple, consistent "table of contents"

[orphan rules]: https://doc.rust-lang.org/reference/items/implementations.html#orphan-rules
[plugin]: ./plugins.md

## Modules

Rust's [module system] is flexible and easy to use,
allowing you to split code across files.

Generally, we find that splitting files that are beyond about 1000 lines is
about the right size for Rust and Bevy projects.
Too small and you'll end up constantly swapping between files,
and too large and finding code within a file becomes frustrating.

Modules within files aren't particularly useful,
but they can be used to control visibility, enable code folding, and are commonly used for tests.

[module system]: https://doc.rust-lang.org/book/ch07-02-defining-modules-to-control-scope-and-privacy.html

## Crates

As your project grows even further,
you should consider splitting it into multiple [crates] using [workspaces].

Using crates is helpful when reusing code between projects, formalizing APIs for specific domains,
or [creating libraries] to share with other Bevy users.
Game code generally looks a lot like library code in Bevy,
so gradually pulling out stable chunks of your game's infrastructure is a sensible and natural thing to do.

But even within the context of a single game, splitting your projects into crates can be worth your time!
Because [Rust parallelizes compilation at the crate level], splitting your code into crates
can make a big difference to compile times on large projects,
above and beyond our [setup tips for faster compilations].

[`cargo build --timings`] is an incredibly helpful tool for measuring and optimizing this.
Try to reduce bottlenecks by being mindful about inter-crate dependencies,
but be mindful that when developing games incremental compilation times are much more important than clean compile times.

This is a key reason why splitting your code into crates is so helpful for working with Bevy in particular:
it allows the compiler to localize the changes to only the crates you've touched,
and use the cached compilation results for everything else.

[crates]: https://doc.rust-lang.org/book/ch07-01-packages-and-crates.html
[workspaces]: https://doc.rust-lang.org/cargo/reference/workspaces.html
[creating libraries]: ../releasing-projects/libraries-for-bevy.md
[Rust parallelizes compilation at the crate level]: https://www.feldera.com/blog/cutting-down-rust-compile-times-from-30-to-2-minutes-with-one-thousand-crates
[setup tips for faster compilations]: ../development-practices/fast-compiles.md
[`cargo build --timings`]: https://doc.rust-lang.org/cargo/reference/timings.html

## Item Visibility

Rust allows you to carefully restrict which code can see various items and structs.
This is known as [visibility], and the details are explained well in the linked reference.

For a game, thinking about visibility may feel like overkill.
We want to iterate fast! Mess about! Find the fun!
You might even be the only person on your team!

And that's largely right: game programming often warrants a more fluid, permissive approach.
But there's still a few good reasons to take advantage of Rust's visibility system:

- carefully maintaining invariants
  - simple validated getter/setter patterns are very effective
  - more complex patterns can often benefit from custom commands or system params to ensure a single blessed workflow
  - clean encapsulation ensures that you can update how something works in a single place
- dead code detection
  - if your types/methods/functions are `pub`, Rust won't known if they're dead code
  - dead code slows down compile times, confuses the reader and slowly rots
- keeping your code *reasonably* untangled
  - iterating quickly is important, and Rust will help you refactor, so you should be careful not to get too tangled up
  - forcing consumers to do things the right way makes it easier to change the internals later

At the end of the day, `pub(crate)` is a good default visibility level for items in larger projects.
Internal methods, invariants, and implementation details should be private, which is the Rust language default behavior.
Finally, fully `pub` items should be reserved for deliberate shared interfaces,
central components / resources like `Player` or `TileMapIndex`, and the plugins that
you add to your `main.rs` to actually add the final functionality.

The fields of your structs should match the visibility of the struct that holds them:
skip the getter/setter methods unless you're performing validation.

[visibility]: https://doc.rust-lang.org/reference/visibility-and-privacy.html
