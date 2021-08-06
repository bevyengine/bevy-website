+++
title = "Welcome"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Welcome to the Bevy game engine! A bevy may be a group of birds, but our engine is:

- **Rust-first:** all the safety, tooling and power that come with a modern systems language
- **Completely free:** [free-of-charge](https://github.com/sponsors/cart), [open source](https://github.com/bevyengine/bevy/blob/main/LICENSE), and hackable with an architecture designed for modularity
- **Innovative ECS:** powered by an ergonomic, full-native entity-component-system architecture that makes writing clear, performant gameplay and engine logic a joy
- **Cross-platform:** support Windows, MacOS, Linux, web*, Android* and iOS with a single code base
- **Effortless parallelism:** a data-oriented design and automatic parallel scheduler provide a high-performance foundation for your game

This book is designed for new and experienced users looking for a thoughtful, [explanation-first](https://diataxis.fr/explanation/) guide to the engine's essential features.
If you just want to try it out hands-on, jump into one of our [quick-start guides](TODO: add link) instead!

*: support for these platforms is still limited; you may encounter some missing features or a more involved setup process

## Stability warning

Bevy aims to be a general-purpose game engine capable of handling arbitrary 2D or 3D games.
However, Bevy is still in its infancy, and there are very good reasons not to choose it for your next project.
No mature solutions currently exist for:

- animation
- networking
- console support

The following areas are part of the engine itself, but are very immature:

- audio
- user interfaces

Solid third-party solutions currently exist for:

- [advanced audio](https://crates.io/crates/bevy_kira_audio)
- [realistic physics](https://github.com/dimforge/bevy_rapier)

While Bevy's modular architecture makes it relatively easy to integrate your own (or third-party) solutions, be mindful that Bevy does not provide a complete game solution out of the box *yet*.
[Contributions to the engine](https://github.com/bevyengine/bevy/) are extremely welcome, but if you want something fully-featured to make your dream game *today*, check out [Godot](https://godotengine.org/) instead!

Unsurprisingly, Bevy's rapid pace of development means that our APIs will break, repeatedly, in both deep and pervasive ways.
This allows us to refine the engine now, adding new features and fixing problems fast, rather than being tied to a first attempt.
That said, updating versions of Bevy is surprisingly painless; we provide migration guides and Rust's excellent tooling will guide you.
Chase the errors until everything compiles again and you should be basically done.
