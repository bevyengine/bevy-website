+++
title = "What is Bevy?"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

Bevy is a modern, ECS-first game engine written in Rust. We aim to be:

- **Capable:** Offer a complete 2D and 3D feature set
- **Simple:** Easy for newbies to pick up, but infinitely flexible for power users
- **Modular:** Use only what you need. Replace what you don't like
- **Fast:** App logic should run quickly, and when possible, in parallel
- **Productive:** Changes should compile quickly... waiting isn't fun

Bevy is [MIT](https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT) + [Apache v2](https://github.com/bevyengine/bevy/blob/main/LICENSE-APACHE) licensed and controlled by the [Bevy Foundation](https://bevyengine.org/foundation/), a 501c3 charity run by the maintainers who do the day-to-day work to make Bevy what it is.
It's powered by hundreds of community contributors, and funded by the [generous donations](https://bevyengine.org/donate/) of the hobbyists and businesses that love and rely on it.

## Why use Bevy?

Bevy is great! You should use Bevy if:

- You like Rust: it's a modern, safe, high-performance language with incredible tooling and a welcoming community
- You like ECS: it's fast, elegant, extremely parallelizable and handles extreme levels of complexity well
- You prefer a code-first approach to game logic: it's all just plain Rust
- You're making something unusual (like CAD software, an art installation or a scientific simulation), and value Bevy's extreme flexibility
- You care about open source and feel more comfortable when you can read/understand/hack your tools from top-to-bottom
- You never want to have to pay licensing fees or worry about vendor lock-in

## Why not use Bevy?

Choose the right tool for your project. You should not use Bevy if:

- You need a stable, mature tool
  - Bevy iterates quickly, and ships breaking changes approximately once every four months
  - Many serious users upgrade Bevy mid-project with the help of our [migration guides](https://bevyengine.org/learn/migration-guides/introduction/), but this can be a serious time sink on large projects
- You want to ship (relatively traditional) games quickly
  - Bevy is still not finished, stable software: many important features are missing!
  - While you can build these features yourself, or work with others in the community, doing so will take valuable development time and add risk
  - [Godot](https://godotengine.org/) is a fantastic choice for many common game genres, and it's even [scriptable with Rust](https://github.com/godot-rust/gdext)!
- You need to make a lot of game content: Bevy does not currently have a graphical editor
- You want scripting language support out-of-the-box
  - Unlike other game engines, gameplay logic is written in the same language (and style!) as engine logic
  - Bevy and Rust make sure that users have the tools needed to support Lua, Python etc integration
  - Take a look at [Bevy Assets](https://bevyengine.org/assets) to see the options that the community have provided
- You want to ship to consoles: Rust, and therefore Bevy, is currently not supported on Sony, Nintendo or Microsoft consoles
