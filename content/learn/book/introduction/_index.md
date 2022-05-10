+++
title = "Introduction"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
+++

If one came here to learn how to make 2D / 3D games, visualizations, user interfaces, or other graphical applications with Bevy, they came to the right place.

<h2>
    <img src="/assets/whats_a_bevy.svg" class="book-whats-a-bevy" alt="What's a BEVY?"/>
</h2>

bevy
- a large group of people or things of a particular kind
- **a group of birds**, particularly quail, but it could be whatever

But Bevy is also a refreshingly simple data-driven game engine built in Rust. It is [free and open-source](https://github.com/bevyengine/bevy) forever.

Bevy has the following design goals:
* **Capable**: Offer a complete 2D and 3D feature set
* **Simple**: Easy for newbies to pick up, but infinitely flexible for power users
* **Data Focused**: Data-oriented architecture using the Entity Component System paradigm 
* **Modular**: Use only what you need. Replace what you don't like
* **Fast**: App logic should run quickly, and when possible, in parallel
* **Productive**: Changes should compile quickly ... waiting isn't fun

Bevy is [built in the open by volunteers](/learn/book/contributing) using free and open-source code because we believe developers should fully own their tools. Games are a huge part of our culture and humanity is investing _millions_ of hours into the development of games. Bevy is rewriting the game engine marketplace, producing a WebGPU rendering using the [Rust programming language](https://www.rust-lang.org/).

For a more in-depth introduction, check out the [Introducing Bevy](/news/introducing-bevy/) blog post.

<h2 class="warning">
    Stability Warning
</h2>

Bevy aims to be a general purpose game engine capable of handling any 2D or 3D workload. <span class="warning">Bevy is currently in development: expect APIs changes with every update.</span> If you are currently trying to pick an engine for your game, there is also [Godot Engine](https://godotengine.org) for those looking for an open source game engine built without Rust, yet is [scriptable with Rust](https://github.com/GodotNativeTools/godot-rust).

This official book will help you get started with the setup and learning the basics, but it does not yet cover most of Bevy's features. See the [Next Steps](/learn/book/next-steps/) page for links to other, more exhaustive, learning resources you can use.
