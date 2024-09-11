+++
title = "Community Reflection on Bevy's Fourth Year"
date = 2024-09-11
authors = ["Carter Anderson"]
[extra]
twitter = "cart_cart"
github = "cart"
youtube = "cartdev"
+++

[@cart](https://www.twitter.com/cart_cart) here (Bevy's creator and Project Lead) with another update! A month ago was Bevy's Fourth Birthday! As is tradition, I took that as a chance to reflect on the past year and outline my hopes and dreams for the future. You can read that in my [Bevy's Fourth Birthday](/news/2024-08-10-bevys-fourth-birthday) post.

I also encouraged the Bevy community to write their own reflections on Bevy's fourth year in a similar style, and to [post them here](https://github.com/bevyengine/bevy-website/issues/1592).

<!-- more -->

## What is Bevy?

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. We have a [Quick Start Guide](/learn/quick-start). You can also check out [Bevy Assets](/assets) for a library of community-developed plugins, crates, games, and learning resources.

## Reflections

We had plenty of responses this year! Here is a central collection of **Bevy's Fourth Birthday** posts:

* **Carter Anderson ([@cart](https://github.com/cart))** <span class="people-role people-role-inline people-role-project-lead">Project Lead</span><span class="people-role people-role-inline people-role-maintainer">Maintainer</span>
  * [Bevy's Fourth Birthday](/news/2024-08-10-bevys-fourth-birthday)
* **Alice Cecile ([@alice-i-cecile](https://github.com/alice-i-cecile))** <span class="people-role people-role-inline people-role-maintainer">Maintainer</span>
  * [I landed my dream job making a Rust game engine. Now what?](/news/2024-09-10-dream-job)
* **Thierry Berger ([@Vrixyz](https://github.com/Vrixyz))**
  * [Happy Bevybirthday](https://thierryberger.com/blog/bevy-4th-birthday/)
* **BD103 ([@BD103](https://github.com/BD103))**
  * [4 Years of Bevy](https://bd103.github.io/blog/2024-08-18-4-years-of-bevy)
* **JMS55 ([@JMS55](https://github.com/JMS55))**
  * [Bevy's Fourth Birthday - A Year of Meshlets](https://jms55.github.io/posts/2024-08-30-bevy-fourth-birthday/)
* **Mikael Matveinen ([@mmatvein](https://github.com/mmatvein))**
  * [Bevy's 4th birthday: Reflections in the context of our own project](https://www.reddit.com/r/bevy/comments/1f712hl/bevys_4th_birthday_reflections_in_the_context_of/)
* **Peter Hayman ([@mrchantey](https://github.com/mrchantey))**
  * [Bevy Turns Four](https://beetmash.com/blog/bevy-turns-four)
* **doomy ([@piedoom](https://github.com/piedoom))**
  * [birthday_system.system()](https://doomy.org/bevys-fourth-birthday/)
* **Chris Biscardi ([@ChristopherBiscardi](https://github.com/ChristopherBiscardi))**
  * [Bevy's Fourth Birthday](https://gist.github.com/ChristopherBiscardi/ab3fc92c3ba83e46cb5577bda2d287bc)

## What are we working on right now?

Here are some things we have coming up!

* [Next Generation Scene / UI](https://github.com/bevyengine/bevy/discussions/14437): We're working on a brand new modern scene system that will make defining Bevy scenes in code, asset files, _and_ the upcoming Bevy Editor _much_ nicer. This will also be the foundation for the Bevy Editor itself, which will be built _on top_ of the new Scene / UI system. In general this will be a massive step forward for Bevy's capabilities and user experience. Read the whole proposal for details!
* [Required Components](https://github.com/bevyengine/bevy/pull/14791): This is the first step in our plan for Bevy's Next Generation Scene / UI system, and it has already been merged into our main branch! It _significantly_ improves the legibility and composability of defining and spawning components. We're in the process of porting Bevy's built-in components / bundles to the new system, and if all goes well, this will be included in the next Bevy release.
* [Retained Render World](https://github.com/bevyengine/bevy/pull/14449): To facilitate parallel pipelined rendering, Bevy has both an "app world" and a "render world". This requires synchronizing the two worlds. Currently, Bevy clears the render world every frame and then repopulates it, which incurs overhead. By switching to a "retained render world", we can avoid redundant work across frames by caching entity data on the renderer side across frames.
* [Function Reflection](https://github.com/bevyengine/bevy/pull/13152): We're making it possible to dynamically call functions via Bevy Reflect, which unlocks scenarios like calling Rust functions referenced in assets such as scenes! We'll use this feature as part of the Next Generation Scene / UI effort.
* [Order Independent Transparency](https://github.com/bevyengine/bevy/pull/14876): A new alternative to sorted alpha blending that does not require sorting draw objects before drawing them. This can both save CPU cycles and improve transparency behavior in some cases.
* [Improved Text API](https://github.com/bevyengine/bevy/discussions/15014): In preparation for Next Generation Scene / UI, we're reworking our `Text` component APIs to be significantly more straightforward to use.
* [Cosmic Text Rendering](https://github.com/bevyengine/bevy/pull/10193): We've switched to [cosmic-text](https://github.com/pop-os/cosmic-text) for our text rendering, which improves our ability to handle scripts and ligatures (among other improvements).
* [Upstreaming bevy_mod_picking](https://github.com/bevyengine/bevy/pull/13677): We're upstreaming aspects of [bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking/) and porting it to Bevy's new Observer system. This provides APIs for picking (identifying / clicking on / selecting) entities in 2D and 3D space.
* [Bevy CLI](https://hackmd.io/cCHAfbtaSviU_MDnbNHKxg): We're building a Bevy command line interface, which will be the one-stop-shop for doing things like generating new Bevy projects from templates, running the asset preprocessor, running Bevy-specific lints, and more!

**Bevy 0.15** will be coming out in a month or so, and some of the items above will land in it!

Here is to another year of Bevy!

\- [@cart](https://github.com/cart/)

<img src="/assets/bevy_logo_dark.svg" style="height: 4.0rem; margin-top: 1.5rem" />
