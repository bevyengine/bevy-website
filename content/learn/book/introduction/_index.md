+++
title = "Introduction"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

If you came here because you wanted to learn how to make 2D / 3D games, visualizations, user interfaces, or other graphical applications with Bevy ... you came to the right place! If not, stick around anyway. I promise it will be fun.

<h2>
    <img src="/assets/whats_a_bevy.svg" class="book-whats-a-bevy" alt="What's a BEVY?"/>
</h2>

A bevy is a group of birds!

But Bevy is also a modular game engine built in Rust, with a focus on developer productivity and performance.

It has the following design goals:
* **Capable**: Offer a complete 2D and 3D feature set
* **Simple**: Easy for newbies to pick up, but infinitely flexible for power users
* **Data Focused**: Data-oriented architecture using the Entity Component System paradigm 
* **Modular**: Use only what you need. Replace what you don't like
* **Fast**: App logic should run quickly, and when possible, in parallel
* **Productive**: Changes should compile quickly ... waiting isn't fun

Bevy is [built in the open by volunteers](/learn/book/contributing) using the <a href="https://www.rust-lang.org/" target="_blank">Rust programming language</a>. The code is free and open-source because we believe developers should fully own their tools. Games are a huge part of our culture and humanity is investing _millions_ of hours into the development of games. Why are we continuing to build up the ecosystems of closed-source monopolies that take cuts of our sales and deny us visibility into the tech we use daily? We believe that the developer community can do so much better.

<h2 class="warning">
    Stability Warning
</h2>

Bevy aims to be a general purpose game engine capable of handling any 2D or 3D workload. However Bevy is still in its infancy. <span class="warning">We are currently in the <i>prototyping</i> phase: important features are missing and APIs will change constantly.</span> If you are currently trying to pick an engine for your Next Big Project™, we recommend that you check out <a href="https://godotengine.org" target="_blank">Godot Engine</a>. It is currently much more feature-complete and stable. And it is also free, open-source, and <a href="https://github.com/GodotNativeTools/godot-rust" target="_blank">scriptable with Rust!</a>

Phew! If you haven't been scared away yet, lets move on to learning some Bevy!