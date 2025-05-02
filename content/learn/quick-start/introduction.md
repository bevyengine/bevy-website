+++
title = "Introduction"
insert_anchor_links = "right"
aliases = ["learn/book/introduction"]
[extra]
weight = 1
+++

If you came here because you wanted to learn how to make 2D / 3D games, visualizations, user interfaces, or other graphical applications with Bevy... you came to the right place! If not, stick around anyway. I promise it will be fun.

<h2>
    <img src="/assets/whats_a_bevy.svg" class="docs-whats-a-bevy" alt="What's a BEVY?"/>
</h2>

A bevy is a group of birds!

But Bevy is also a refreshingly simple data-driven game engine built in Rust. It is [free and open-source](https://github.com/bevyengine/bevy) forever under your choice of the MIT or Apache 2.0 licenses.

Bevy has the following design goals:

* **Capable**: Offer a complete 2D and 3D feature set
* **Simple**: Easy for newbies to pick up, but infinitely flexible for power users
* **Data Focused**: Data-oriented architecture using the Entity Component System paradigm
* **Modular**: Use only what you need. Replace what you don't like
* **Fast**: App logic should run quickly, and when possible, in parallel
* **Productive**: Changes should compile quickly... waiting isn't fun

Bevy is [built in the open by volunteers](https://bevyengine.org/learn/contribute/introduction) using the [Rust programming language](https://www.rust-lang.org/). The code is free and open-source because we believe developers should fully own their tools. Games are a huge part of our culture and humanity is investing _millions_ of hours into the development of games. Why are we continuing to build up the ecosystems of closed-source monopolies that take cuts of our sales and deny us visibility into the tech we use daily? We believe that the developer community can do so much better.

For a more in-depth introduction, check out the [Introducing Bevy](/news/introducing-bevy/) blog post.

{% callout(type="warning") %}
## Stability Warning

Bevy is still in the early stages of development. Important features are missing. Documentation is sparse. A new version of Bevy containing breaking changes to the API is released [approximately once every 3 months](https://bevyengine.org/news/bevy-0-6/#the-train-release-schedule). We provide [migration guides](https://bevyengine.org/learn/book/migration-guides/), but we can't guarantee migrations will always be easy. Use only if you are willing to work in this environment.

If you are currently trying to pick an engine for your Next Big Project™, we recommend that you check out [Godot Engine](https://godotengine.org). It is currently much more feature-complete and stable. And it is also free, open-source, and [scriptable with Rust](https://github.com/godot-rust/gdext)!
{% end %}

The Quick Start Guide is not a comprehensive guide to Bevy and the next section [Getting Started](/learn/quick-start/getting-started/) will help you with the setup of Bevy and learning the basics, but it does not cover most of Bevy's features. In the future you can use the Bevy Book to gain a better understanding, until then see the last page [Next Steps](/learn/quick-start/next-steps) for more exhaustive and complex resources on Bevy.

Phew! If you haven't been scared away yet, let's move on to learning some Bevy!
