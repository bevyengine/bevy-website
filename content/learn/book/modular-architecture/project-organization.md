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
  - don't try to group together objects by "kind", like "components" and "systems"
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

## Project organization by example

Now that we have a handle on the tools available, let's talk over how a hypothetical project might change and grow.
Our small studio is building a Wild West single player first-person shooter!

The very first step is to create a new project.
Our folder structure will look something like:

- src/
  - main.rs
- Cargo.toml

Simple enough! We get to Hello World with some [`DefaultPlugins`], and add some third-party plugins that we know we'll want,
and stick it all in `main.rs`.

Next up, we're going to need a first-person shooter character controller.
We hunt around online, find a permissively licensed template or example, and copy-paste it into our project under a new `player.rs` file,
declaring this file as a module under `main.rs`.
We create a `PlayerPlugin`, add it to our `fn main`, and make sure we can hop around and aim.
We *don't* pull in a library for this, because we know that we're going to need to heavily customize the exact behavior.

We also need some placeholder assets, so we look on [itch.io] and grab something with about the right proportions.
Our project now looks like this:

- assets/
  - main_character.gltf
- src/
  - main.rs
  - player.rs
- Cargo.toml

With a bit of basic gameplay working, it's time to tackle some infrastructure that we know we'll need: menus and loading screens.
We look at what we've done in previous projects and copy-paste liberally into a new `ui` folder, with multiple sub-files.
We setup a basic `GameState` here, to allow us to pause and restart the game and handle opening and closing menus,
sticking it in our `main.rs` because it's used throughout the project.
Now, our project looks like this:

- assets/
  - main_character.gltf
- src/
  - ui/
    - mod.rs
    - main_menu.rs
    - pause_menu.rs
    - settings_menu.rs
  - main.rs
  - player.rs
- Cargo.toml

We're off to a good start, and it's time to add some more features!
Let's get some basic enemies going. Wait, hang on, we want to share our character controller between our players and our enemies.
After a quick refactor:

- assets/
  - main_character.gltf
- src/
  - characters/
    - mod.rs
    - controller.rs
    - player.rs
  - ui/
    - mod.rs
    - main_menu.rs
    - pause_menu.rs
    - settings_menu.rs
  - main.rs
- Cargo.toml

With the help of Rust's compiler, that wasn't too bad. And now, we can finally add enemies!

- assets/
  - main_character.gltf
  - enemy.gltf
- src/
  - characters/
    - mod.rs
    - controller.rs
    - enemies.rs
    - player.rs
  - ui/
    - mod.rs
    - main_menu.rs
    - pause_menu.rs
    - settings_menu.rs
  - main.rs
- Cargo.toml

Let's add some combat mechanics.
After a few commits, our project has a brand new folder with plenty of game logic,
and our enemies have some actual behavior:

- assets/
  - main_character.gltf
  - enemy.gltf
- src/
  - characters/
    - mod.rs
    - ai.rs
    - controller.rs
    - enemies.rs
    - player.rs
  - combat/
    - mod.rs
    - life_and_damage.rs
    - weapons.rs
  - ui/
    - mod.rs
    - main_menu.rs
    - pause_menu.rs
    - settings_menu.rs
  - main.rs
- Cargo.toml

Our enemy logic and behavior keeps growing in a messy way.
Let's do some research, and swap over to a more sophisticated [Goal Oriented Action Planning] solution.
Now:

- assets/
  - main_character.gltf
  - enemy.gltf
- src/
  - characters/
    - ai/
      - mod.rs
      - goals.rs
      - actions.rs
      - planning.rs
    - mod.rs
    - controller.rs
    - enemies.rs
    - player.rs
  - combat/
    - mod.rs
    - life_and_damage.rs
    - weapons.rs
  - ui/
    - mod.rs
    - main_menu.rs
    - pause_menu.rs
    - settings_menu.rs
  - main.rs
- Cargo.toml

Ugh, the compile times are really starting to slow down, and things are getting complicated.
Let's refactor this to be a workspace.
One step at a time though: get the simple setup right first, then we'll spin out subcrates.
We're just going to create a binary application and a single library,
and dump everything but our main.rs contents into the library, which we'll depend on in our binary.

- assets/
  - main_character.gltf
  - enemy.gltf
- wild_west_game/
  - src/
    - main.rs
  - Cargo.toml
- wild_west_lib/
  - src/
    - characters/
      - ai/
        - mod.rs
        - goals.rs
        - actions.rs
        - planning.rs
      - mod.rs
      - controller.rs
      - enemies.rs
      - player.rs
    - combat/
      - mod.rs
      - life_and_damage.rs
      - weapons.rs
    - ui/
      - mod.rs
      - main_menu.rs
      - pause_menu.rs
      - settings_menu.rs
    - lib.rs
    - Cargo.toml
- Cargo.toml

We've had to move our `GameState` into our `lib.rs` so we can rely on it in our plugins, but that wasn't too bad.
Now that everything is working again, let's start splitting things up!
We can split out our new AI library for sure though: that's something we might want to publish one day!

- assets/
  - main_character.gltf
  - enemy.gltf
- wild_west_game/
  - src/
    - main.rs
  - Cargo.toml
- wild_west_lib/
  - src/
    - characters/
      - mod.rs
      - controller.rs
      - enemies.rs
      - player.rs
    - combat/
      - mod.rs
      - life_and_damage.rs
      - weapons.rs
    - ui/
      - mod.rs
      - main_menu.rs
      - pause_menu.rs
      - settings_menu.rs
    - lib.rs
- better_goap/
  - src/
    - lib.rs
    - goals.rs
    - actions.rs
    - planning.rs
  - Cargo.toml
- Cargo.toml

We're still not sure exactly how our combat is going to work,
and it's going to need to interface closely with our character code.
Let's leave that alone for now.
However, we've been bitten before by spaghettti code with our UI:
let's spin that out and reduce the temptation.
We'll define the various settings for the UI in our `wild_west_lib` using resources,
and have our new `wild_west_ui` simply read and write those resource values.

- assets/
  - main_character.gltf
  - enemy.gltf
- wild_west_game/
  - src/
    - main.rs
  - Cargo.toml
- wild_west_lib/
  - src/
    - characters/
      - mod.rs
      - controller.rs
      - enemies.rs
      - player.rs
    - combat/
      - mod.rs
      - life_and_damage.rs
      - weapons.rs
    - lib.rs
- wild_west_ui/
  - src/
    - lib.rs
    - main_menu.rs
    - pause_menu.rs
    - settings_menu.rs
  - Cargo.toml
- better_goap/
  - src/
    - lib.rs
    - goals.rs
    - actions.rs
    - planning.rs
  - Cargo.toml
- Cargo.toml

Watching this hypothetical studio develop their hypothetical game,
we can come to appreciate where the advice about "don't try and get it right from the start",
and "organize your code by functionality" comes from.

As you build your project, your needs will shift, and over-architecting a beautiful file structure
will waste time and slow you down.
But that doesn't mean you should be completely blind to potential future needs!
By grouping related code together, we can quickly and reliably split apart files,
and spin off folders into their own crates as the need arises.

If you keep each of your refactors small, use version control and make sure your project always keeps running,
experimenting with project structure and architecture is easy and safe!

[`DefaultPlugins`]: https://docs.rs/bevy/latest/bevy/struct.DefaultPlugins.html
[itch.io]: https://itch.io/
[Goal Oriented Action Planning]: https://goap.crashkonijn.com/readme/theory
