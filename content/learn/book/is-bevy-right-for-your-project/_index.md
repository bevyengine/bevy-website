+++
title = "Is Bevy Right for Your Project?"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 50
status = 'hidden'
+++

We think Bevy's great, but it's not the right tool for every project.
Use this page to make an informed decision.

## Why Use Bevy?

Bevy is delightful! You should use Bevy if:

- _You like Rust:_
  - It's a modern, safe, high-performance language with incredible tooling and a welcoming community.
- _You like ECS:_
  - Fast, elegant, and extremely parallelizable, ECS handles extreme levels of complexity well.
- _You prefer a code-first approach to game logic:_
  - Bevy is all just plain Rust.
- _You're making something unusual:_
  - Whether you're making CAD software, art installations, or scientific simulations, Bevy can be extremely flexible.
- _You care about open source software:_
  - Read, understand, and hack your tools from top-to-bottom.
- _You want an engine with a lively [community](@/community/_index.md):_
  - Users, engine devs, and ecosystem creators join forces from across the world.
- _You never want to have to pay licensing fees or worry about vendor lock-in._
  - Bevy is free and open source from now until the end of time.

## Why Not Use Bevy?

We love Bevy, but it's not the right tool for every project. You should not use Bevy if:

- _You need a stable, mature tool:_
  - Bevy iterates quickly, and ships breaking changes approximately once every four months.
  - Many serious users upgrade Bevy mid-project with the help of our [migration guides](https://bevy.org/learn/migration-guides/introduction/), but this can be a serious time sink on large projects.
- _You want to ship (relatively traditional) games very quickly:_
  - Bevy is still not finished, stable software: many important features are missing.
  - While you can build these features yourself, or work with others in the community, doing so will take valuable development time and add risk.
  - [Godot](https://godotengine.org/) is a fantastic choice for many common game genres, and it's even [scriptable with Rust](https://github.com/godot-rust/gdext).
- _You need to make a lot of game content:_
  - Bevy does not currently have a graphical editor.
- _You want scripting language support out-of-the-box:_
  - Unlike other game engines, gameplay logic is written in the same language (and style) as engine logic.
  - That said, it is possible. Bevy and Rust provide the tools needed to integrate Lua, Python, and more.
  - Take a look at [Bevy Assets](https://bevy.org/assets) to see the options that the community has provided.
- _You want to ship to consoles:_
  - Rust (and therefore Bevy) is currently not supported on Sony or Nintendo consoles.

## Bevy Beyond Games

Game engines are, at their core, high-performance, real-time simulators coupled with a framework for producing high-fidelity graphics and immersive sound. Game engines are designed to squeeze every last drop of performance out of the available hardware (both CPU and GPU). The fruits of this labor are just as applicable to data visualization and computer-aided design as they are to building entertainment products.

Bevy's modular, flexible nature makes it particularly suited to these weirder applications, and there are multiple successful companies and large projects using Bevy to build everything from CAD to art installations. Take what you want; ignore or rewrite the rest.

While our documentation and development priorities place games in the foreground, we care about all of our non-game users too, no matter what you're building.
