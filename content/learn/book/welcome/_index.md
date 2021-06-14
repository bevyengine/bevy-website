+++
title = "Welcome to Bevy"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Welcome to the Bevy game engine! While Bevy is still very early in development, it's already:

- Rust-first: with all the safety, tooling and power that comes with a modern systems language
- completely free, open source and hackable with an architecture designed for modularity
- powered by an ergonomic and innovative  ECS that makes writing gameplay and engine logic a joy
- support all major desktop platforms with a single code-base, with limited mobile and web support
- high-performance enough that it won't get in your way

This book is designed to provide a more thoughtful, explanation-first to the engine's essential features.
If you just want to try it out hands-on, jump into one of our quick-start guides instead!

## Why not Bevy?

Of course, Bevy is *extremely* immature, and there are very good reasons not to choose it for your next project.
It is currently completely missing first-party solutions for:

- animation
- networking
- console support
- physics

The following areas are very immature:

- audio
- user interface
- web support
- mobile support

While Bevy's modular architecture makes it relatively easy to integrate your own (or third-party) solutions, be mindful that it won't Just Work out of the box.
Contributions are extremely welcome, but if you want something fully featured for a game *today*, Bevy is not the right engine for you yet.

Unsurprisingly, Bevy's rapid pace of development means that our APIs will break, repeatedly, in both deep and pervasive ways.
This allows us to refine the engine now, adding new features and fixing problems fast, rather than being tied to a first attempt.

While we'll do our best to provide [migration guides, updating versions of Bevy is largely an exercise in letting the excellent Rust tooling guide you and chasing the errors until everything compiles again.