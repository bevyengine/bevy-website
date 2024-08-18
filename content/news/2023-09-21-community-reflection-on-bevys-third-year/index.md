+++
title = "Community Reflection on Bevy's Third Year"
date = 2023-09-21
authors = ["Carter Anderson"]
[extra]
twitter = "cart_cart"
github = "cart"
youtube = "cartdev"
+++

[@cart](https://www.twitter.com/cart_cart) here (Bevy's creator and Project Lead) with another update! A little over a month ago was Bevy's Third Birthday! As is tradition, I took that as a chance to reflect on the past year and outline my hopes and dreams for the future. You can read that in my [Bevy's Third Birthday](https://bevyengine.org/news/bevys-third-birthday/) post.

This year for the first time I also encouraged the Bevy community to write their own reflections on Bevy's third year in a similar style, and to [post them here](https://github.com/bevyengine/bevy-website/issues/728).

<!-- more -->

## What is Bevy?

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. We have a [Quick Start Guide](/learn/quick-start/introduction). You can also check out [Bevy Assets](/assets/) for a library of community-developed plugins, crates, games, and learning resources.

## Reflections

We had a number of responses! Here is a central collection of **Bevy's Third Birthday** posts:

* **Carter Anderson ([@cart](https://github.com/cart))** <span class="people-role people-role-inline people-role-project-lead">Project Lead</span><span class="people-role people-role-inline people-role-maintainer">Maintainer</span>
  * [Bevy's Third Birthday](https://bevyengine.org/news/bevys-third-birthday/)
* **Alice Cecile ([@alice-i-cecile](https://github.com/alice-i-cecile))** <span class="people-role people-role-inline people-role-maintainer">Maintainer</span>
  * [Triage-by-controversy and community review](https://www.leafwing-studios.com/blog/triage-by-controversy/)
* **Nicola Papale ([@nicopap](https://github.com/nicopap))**
  * [A Year of Bevy, Past and Future](https://devildahu.ch/devlog/bevy-3-years/)
* **Mike ([@hymm](https://github.com/hymm))**
  * [Looking Back at Stageless](https://www.hsubox.design/posts/looking-back-at-stageless/)
* **Trent ([@tbillington](https://github.com/tbillington))**
  * [Three Years of Bevy 🎉](https://trent.kiwi/bevy-three-years)
* **JMS55 ([@JMS55](https://github.com/JMS55))**
  * [Bevy's Third Birthday - Reflections on Rendering](https://jms55.github.io/posts/2023-09-12-bevy-third-birthday/)

## What are we working on right now?

Here are some things we have coming up!

* [Bevy UI and Scene system overhaul](https://github.com/bevyengine/bevy/discussions/9538): I just proposed a brand new unified Bevy UI and Bevy Scene system to give us solid foundations to build the Bevy Editor on. This will allow users to compose, nest, override, and style scenes and UI both in code and in asset files. All using the same ergonomic format supporting hot reloading, IDE autocomplete, and more! This is both a design document and an initial prototype. The community has been reacting to and iterating on the initial design.
* [Bevy Asset V2](https://github.com/bevyengine/bevy/pull/8624): We recently merged Bevy Asset V2 ... a brand new production grade asset system for Bevy. It adds asset preprocessing, as well as massive, improvements to the asset APIs and internals. We also have a [ton of additional features](https://github.com/bevyengine/bevy/issues/9714) planned that build on top of it. I'm almost finished adding "multiple asset sources", so you can load (and process) assets from "anywhere" (including mixing and matching sources).
* [Automatic batching and instancing of draw commands](https://github.com/bevyengine/bevy/pull/9685): This adds some initial "automatic batching" support, giving us some nice wins. We have big plans for continuing to optimize the renderer. This is just the beginning!
* [Rusty Shader Imports](https://github.com/bevyengine/naga_oil/pull/41): We're improving the usability and terseness of our import system by adopting rust-like syntax and functionality. Imports will feel much more natural after this.
* [Deferred Rendering](https://github.com/bevyengine/bevy/pull/9258): We're considering merging deferred rendering functionality into Bevy, and we have a working implementation! Lots of things to consider here before committing though.
* [GPU Picking](https://github.com/bevyengine/bevy/pull/8784): We have a GPU picking proposal that would add fast and accurate GPU-driven object picking to Bevy (identifying objects when they are clicked on).
* [PCF for Shadows](https://github.com/bevyengine/bevy/pull/8006): Shadows will have nice PCF filters that will drastically increase their quality.
* [Light Transmission for Materials](https://github.com/bevyengine/bevy/pull/8015): We're adding support for simulating light penetrating our PBR material, which lets us render things like glass, wax, rubber, etc with higher quality.
* [UI Node Borders and Shadows](https://github.com/bevyengine/bevy/pull/8973): This will improve the ability to express UIs by giving additional style options to users.
* [System Stepping](https://github.com/bevyengine/bevy/pull/8453): We are considering merging "system stepping", which would give users the ability to pause execution and run systems step-by step. This should make some debugging scenarios much easier!
* [And plenty more!](https://github.com/bevyengine/bevy/pulls)

**Bevy 0.12** will be released mid-October and some of these (including Bevy Asset V2) will be included!

Here is to another year of Bevy!

\- [@cart](https://github.com/cart/)

<img src="/assets/bevy_logo_dark.svg" style="height: 4.0rem; margin-top: 1.5rem" class="inverted"/>
