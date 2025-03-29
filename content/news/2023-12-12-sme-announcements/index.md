+++
title = "SME Announcements"
date = 2023-12-12
authors = ["Carter Anderson"]
[extra]
github = "cart"
youtube = "cartdev"
+++

[@cart](https://www.twitter.com/cart_cart) here (Bevy's creator and Project Lead) with a quick update! We have some new SMEs (Subject Matter Experts) to announce.

<!-- more -->

## What is an SME?

[At the start of this year](/news/scaling-bevy-development/) we rolled out the [new structure for The Bevy Organization](https://github.com/bevyengine/bevy/blob/main/docs/the_bevy_organization.md), including the Subject Matter Expert system. SMEs are Bevy community members that have distinguished themselves as:

1. An expert in a given area (ex: Rendering, Audio, ECS, etc)
2. Aligned with our vision for Bevy
3. Easy to work with and welcoming to new contributors

An SME approval counts as a "vote" for controversial changes (ex: a [pull request](https://github.com/bevyengine/bevy/pulls) or [RFC](https://github.com/bevyengine/rfcs) that is opinionated, highly technical, impactful, and/or large). If a controversial change receives 2 votes, it can be merged.

For a list of all current SMEs (and a full view of the Bevy Organization), check out the [Bevy People](/community/people/) page.

## The New SMEs

Please join me in welcoming our new SMEs!

* **ECS**
  * [@JoJoJet](https://github.com/JoJoJet) (Joseph Giordano): A highly technical ECS contributor with a prolific number of contributions, and a focus on low level internals, quality, and correctness.
  * [@alice-i-cecile](https://github.com/alice-i-cecile/) (Alice Cecile): Has helped design and implement a ton of ECS features, including ECS Schedule V3. A long-time ECS developer with a vision for where we should be headed.
* **Rendering**
  * [@IceSentry](https://github.com/IceSentry): Has contributed foundational improvements to Bevy's renderer, such as depth and normal prepasses and render graph improvements. Builds on Bevy's renderer tech professionally at Foresight Spatial Labs. A truly prolific reviewer possessing a firm understanding of the internals.
* **Input**
  * [@alice-i-cecile](https://github.com/alice-i-cecile/) (Alice Cecile): Authored leafwing-input-manager, a solid 3rd party action-driven input plugin for Bevy. Interested in bringing similar features upstream!
  * [@mockersf](https://github.com/mockersf) (François Mockers): Contributed a variety of input improvements and brings mobile expertise to the space.
* **Mobile**
  * [@mockersf](https://github.com/mockersf) (François Mockers): Brought Bevy's Android support across the finish line, built out a "real phone" CI mobile test suite, and regularly ensures Bevy continues to work across platforms.
* **Docs**
  * [@alice-i-cecile](https://github.com/alice-i-cecile/) (Alice Cecile): Has been our lead doc-wrangler for years, which includes code docs, examples, and driving efforts like the new Bevy Book.

As always, these appointments shouldn't be particularly surprising. These people have been building fantastic things in their areas for awhile now.

## Quick Updates

I'll also take this as a chance to share some other quick updates with you:

* **Bevy Jam #4**: [Bevy Jam #4](https://itch.io/jam/bevy-jam-4) just wrapped up! The theme was **That's a LOT of Entities**! We are now in the voting period. You can now play and rate the entries.
* **A Legal Bevy Entity**: We are in the process of designing and implementing a "legal Bevy entity". This will be a centralized entity that will do things like accept and distribute sponsorships, as well as hold on to things like "ownership of the Bevy logo". Expect an announcement about this soon.
* **A New Bevy Scene and UI System**: We are deep in the prototyping phase of defining the new system for composing Bevy scenes (including UIs). This is a critical first step to unlock Bevy Editor development. It will also massively improve the experience of composing Bevy scenes, both in code and in asset files (composed by hand _or_ in the upcoming editor). We should have updates on this soon.

This is an exciting time for Bevy. The next year is going to be big for us!

\- [@cart](https://github.com/cart/)
