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

## Screen Space Ambient Occlusion

<div class="release-feature-authors">authors: @JMS55</div>

**Without SSAO**
![no_ssao](no_ssao.png)

**With SSAO**
![with_ssao](with_ssao.png)

**SSAO Only**
![ssao_only](ssao_only.png)

Bevy now supports Screen Space Ambient Occlusion (SSAO). While Bevy already supported shadows from direct lights
([`DirectionalLight`], [`PointLight`], [`SpotLight`]) via shadow mapping, Bevy now supports shadows from _indirect_ diffuse lighting such as [`AmbientLight`] or [`EnvironmentMapLight`].

These shadows give scenes a more "grounded" feel, by estimating how much surrounding geometry blocks incoming light via the screen-space depth and normal prepasses. You can try it out in the new [SSAO example](https://github.com/bevyengine/bevy/blob/v0.11.0/examples/3d/ssao.rs).

Note that using SSAO with the newly added Temporal Anti-Aliasing leads to a _large_ increase in quality and noise reduction.

Platform support is currently limited - Only Vulkan and Metal are currently supported. DirectX12 will be supported via an upcoming patch release of `wgpu`, and WebGPU support will come at a later date.

[`DirectionalLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.DirectionalLight.html
[`PointLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.PointLight.html
[`SpotLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.SpotLight.html
[`AmbientLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.AmbientLight.html
[`EnvironmentMapLight`]: https://docs.rs/bevy/0.11.0/bevy/pbr/struct.EnvironmentMapLight.html

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author
