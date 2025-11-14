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
- **Productive:** Changes should be visible quickly... waiting isn't fun

Bevy is [MIT](https://github.com/bevyengine/bevy/blob/main/LICENSE-MIT) + [Apache v2](https://github.com/bevyengine/bevy/blob/main/LICENSE-APACHE) licensed and controlled by the [Bevy Foundation](https://bevyengine.org/foundation/), a 501c3 charity run by the maintainers who do the day-to-day work to make Bevy what it is.
It's built in the open, powered by hundreds of [community contributors](https://bevyengine.org/learn/contribute/introduction) and funded by the [generous donations](https://bevyengine.org/donate/) of the hobbyists and businesses that love and rely on it.

## Why use Bevy?

Bevy is delightful! You should use Bevy if:

- You like Rust: it's a modern, safe, high-performance language with incredible tooling and a welcoming community
- You like ECS: it's fast, elegant, extremely parallelizable and handles extreme levels of complexity well
- You prefer a code-first approach to game logic: it's all just plain Rust
- You're making something unusual (like CAD software, an art installation, or a scientific simulation), and value Bevy's extreme flexibility
- You care about open source and feel more comfortable when you can read/understand/hack your tools from top-to-bottom
- You want an engine with a lively [community](/community) of users, engine devs and ecosystem creators
- You never want to have to pay licensing fees or worry about vendor lock-in

## Why not use Bevy?

We love Bevy, but it's not the right tool for every project. You should not use Bevy if:

- You need a stable, mature tool
  - Bevy iterates quickly, and ships breaking changes approximately once every four months
  - Many serious users upgrade Bevy mid-project with the help of our [migration guides](https://bevyengine.org/learn/migration-guides/introduction/), but this can be a serious time sink on large projects
- You want to ship (relatively traditional) games very quickly
  - Bevy is still not finished, stable software: many important features are missing!
  - While you can build these features yourself, or work with others in the community, doing so will take valuable development time and add risk
  - [Godot](https://godotengine.org/) is a fantastic choice for many common game genres, and it's even [scriptable with Rust](https://github.com/godot-rust/gdext)!
- You need to make a lot of game content: Bevy does not currently have a graphical editor
- You want scripting language support out-of-the-box
  - Unlike other game engines, gameplay logic is written in the same language (and style!) as engine logic
  - That said, it is possible. Bevy and Rust make sure that users have the tools needed to integrate Lua, Python and more
  - Take a look at [Bevy Assets](https://bevyengine.org/assets) to see the options that the community have provided
- You want to ship to consoles: Rust, and therefore Bevy, is currently not supported on Sony, Nintendo, or Microsoft consoles

## Bevy beyond games

Game engines are, at their core, high-performance, real-time simulators coupled with a framework for producing high-fidelity graphics and immersive sound. Game engines are designed to squeeze every last drop of performance out of the available hardware (both CPU and GPU). The fruits of this labor are just as applicable to data visualization and computer-aided design as they are to building entertainment products.

Bevy's modular, flexible nature makes it particularly suited to these weirder applications, and there are multiple successful
companies and large projects using Bevy to build everything from CAD to art installations.
Take what you want; ignore or rewrite the rest.

While our documentation and development priorities place games in the foreground, we care about all of our non-game users too:
no matter what you're building.
