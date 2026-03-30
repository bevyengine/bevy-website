+++
title = "Introduction"
insert_anchor_links = "right"
aliases = ["learn/book/introduction"]
[extra]
weight = 1
+++

If you came here because you wanted to learn how to make 2D / 3D games, visualizations, user interfaces, or other graphical applications with Bevy, you came to the right place!
If not, stick around anyway.
We promise it will be fun!

<h2>
    <img src="/assets/whats_a_bevy.svg" class="docs-whats-a-bevy" alt="What's a BEVY?"/>
</h2>

A bevy is a group of birds!

But Bevy is also a refreshingly simple data-driven game engine built in Rust.
It is [free and open-source](https://github.com/bevyengine/bevy) forever under your choice of the MIT or Apache 2.0 licenses.

Bevy has the following design goals:

* **Capable**: Offer a complete 2D and 3D feature set.
* **Simple**: Easy for newbies to pick up, but infinitely flexible for power users.
* **Data Focused**: Data-oriented architecture using the Entity Component System (ECS) paradigm.
* **Modular**: Use only what you need. Replace what you don't like.
* **Fast**: App logic should run quickly, and when possible, in parallel.
* **Productive**: Changes should compile quickly... waiting isn't fun!

Bevy is [built in the open by volunteers](https://bevy.org/learn/contribute/introduction) using the [Rust programming language](https://www.rust-lang.org/).
The code is free and open-source because we believe developers should fully own their tools.
Games are a huge part of our culture and humanity is investing _millions_ of hours into the development of games.
Why are we continuing to build up the ecosystems of closed-source monopolies that take cuts of our sales and deny us visibility into the tech we use daily?
We believe that the developer community can do so much better.

For a more in-depth introduction, check out the [Introducing Bevy](/news/introducing-bevy/) blog post.

{% callout(type="warning") %}

## Stability Warning

Bevy is still in development.
Important features are missing, and documentation can be sparse.
A new version of Bevy containing breaking changes to the API is released [approximately once every 3 months](https://bevy.org/news/bevy-0-6/#the-train-release-schedule).
We provide [migration guides](https://bevy.org/learn/book/migration-guides/), but we can't guarantee that migrations will always be easy.

**Our recommendation is to only use Bevy if you are willing to work in this environment.**

If you are currently trying to pick an engine for your _Next Big Project™_, we recommend that you check out [Godot Engine](https://godotengine.org).
It is currently much more feature-complete and stable.
Plus, it is also free, open-source, and [scriptable with Rust](https://github.com/godot-rust/gdext)!
{% end %}

## Bevy Learning Paths

Bevy provides two resources for learning and getting familiar with Bevy: the Quick Start Guide, and the Bevy Book.

Currently you're looking at the introduction of the Quick Start Guide!
This is _not_ meant to be a comprehensive guide for learning how Bevy works, instead it will help you get Bevy setup and ready for development.
Don't worry if you aren't familiar with Bevy though, we will also provide a quick introduction to the fundamentals of some core concepts in the following sections as well.
Head over to the next section [Getting Started] to continue with the Quick Start Guide.

If instead you're looking for a more in-depth learning resource to see how Bevy works, the [Bevy Book] is where you'll want to head to.
Each book chapter provides a detailed look at different aspects of Bevy, including [how Bevy stores data], [how to construct game systems], and even advice for [structuring your projects] and [implementing productive development practices].

{% callout(type="info") %}
Please be aware that the Bevy Book is still a work-in-progress.

While the Bevy Book is continuously being expanded and improved alongside the Bevy Engine, there are still sections that are missing and core aspects that need to be documented.
If you'd like to help us get these missing sections written out, please check out the [Contribute](/learn/contribute) page on the website!
{% end %}

Phew! If you haven't been scared away yet, let's move on to learning some Bevy!

[Getting Started]: /learn/quick-start/getting-started/
[Bevy Book]: /learn/book/intro
[how Bevy stores data]: /learn/book/storing-data
[how to construct game systems]: /learn/book/control-flow
[structuring your projects]: /learn/book/modular-architecture
[implementing productive development practices]: /learn/book/development-practices
