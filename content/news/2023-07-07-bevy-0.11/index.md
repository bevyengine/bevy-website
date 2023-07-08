+++
title = "Bevy 0.11"
date = 2023-07-07
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.11** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.11**, check out our [0.10 to 0.11 Migration Guide](/learn/migration-guides/0.10-0.11/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Feature**: description

## Temporal Antialiasing

<div class="release-feature-authors">authors: @JMS55, @DGriffin91</div>

Alongside MSAA and FXAA, Bevy now supports Temporal Antialiasing (TAA) as an antialiasing option. Try it out in
Bevy's improved antialiasing example.

TAA works by blending the newly rendered frame with past frames in order to smooth out aliasing artifacts in the image,
particularly specular aliasing of light on reflective surfaces.

Here's a quick rundown of the following advantages and disadvantages of each antialiasing method that Bevy supports:

* Mutli-Sample Antialiasing (MSAA)
  * Pro: Does a good job at smoothing the edges of meshes (anti geometric aliasing)
  * Con: Does not help with specular aliasing
  * Con: Performance cost scales with triangle count, and performs very poorly on scenes with many triangles
![msaa](msaa.png)
* Fast Approximate Antialiasing (FXAA)
  * Pro: Does a decent job of dealing with both geometric and specular aliasing
  * Pro: Very little performance cost in all scenes
  * Con: Somewhat blurry and low quality results
* Temporal Antialiasing (TAA)
  * Pro: Does a very good job at dealing with both geometric and specular aliasing
  * Pro: Does a good job at dealing with temporal aliasing, where high-frequency details flicker
  over time or as you move the camera around or as things animate
  * Pro/Con: Performance cost is moderate, and scales only with screen resolution
  * Con: Chance of "ghosting" where meshes or lighting effects may leave trails behind them that fade over time
  * Con: Although TAA helps with reducing temporal aliasing, it may also introduce additional temporal aliasing,
  especially on thin geometry or texture detail rendered at a distance
  * Con: Requires 2 view's worth of additional GPU memory, as well as enabling the motion vector and depth prepasses
  * Con: Requires accurate motion vector and depth prepasses, which complicates custom materials
![taa](taa.png)

TAA implementations are a series of tradeoffs and rely on heuristics that are easy to get wrong. Currently, these heuristics are not
user-configurable. For this reason, along with the below limitations in Bevy 0.11, TAA is marked as an experimental feature. We will
continue to improve quality, compatibility, and performance in future releases. Please report any bugs you encounter!

  * Current Bevy limitation: Does not currently work with all bevy features, namely skinning, morph targets, and parallax mapping
  * Current Bevy limitation: Currently tends to soften the image a bit, which can be worked around via post-process sharpening

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author
