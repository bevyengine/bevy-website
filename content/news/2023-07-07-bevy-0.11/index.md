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

## Improved Shader Imports

<div class="release-feature-authors">authors: @robtfm</div>

Bevy's rendering engine has a lot of great options and features. For example, the PBR `StandardMaterial` pipeline supports desktop/webgpu and webgl, 6 optional mesh attributes, 4 optional textures, and a plethora of optional features like fog, skinning, and alpha blending modes, with more coming in every release.

Many feature combos need specialized shader variants, and with over 3000 lines of shader code split over 50 files in total, the text-substitution-based shader processor was beginning to creak at the seams.

This release we've switched to using [naga_oil](https://github.com/bevyengine/naga_oil), which gives us a module-based shader framework. It compiles each file individually to naga's IR and then combines them into a final shader on demand. This doesn't have much visible impact yet, but it does give a few immediate benefits:

* The engine's shader code is easier to navigate and less magical. Previously there was only a single global scope, so items could be referenced even if they were only imported indirectly. This sometimes made it hard to locate the actual code behind the reference. Now items must be explicitly imported, so you can always tell where a variable or function originated just by looking at the current file:

<kbd><img src="imported items.png" /></kbd>

* Shaders now have codespan reporting, an error will point you to the shader file and line number, preventing a lot of hair pulling in complex shader codebases:

<kbd><img src="codespan.png" /></kbd>

* naga_oil's preprocessor supports a few more conditional directives, you can use `#else if` and `#else ifndef` as well as `#else ifdef` which was previously supported

* Functions, variables and structs are all properly scoped so a shader file doesn't need to use globally unique names to avoid conflicts

* Shader defs can be added to modules directly. For example, any shader that imports `bevy_pbr::mesh_view_types` now has `MAX_DIRECTIONAL_LIGHTS` automatically defined, there's no longer a need to remember to add it for every new pipeline that uses the module.

The future possibilities are more exciting. Using naga IR opens the door to a bunch of nice features that we hope to bring in future releases:

* Automatic bind slot allocation will let plugins extend the core view bindgroup, which means self-contained plugins for features like lighting and shadow methods, common material properties, etc become viable. This will allow us to modularise the core pipelines to make growing the codebase - while keeping support for multiple targets - more sustainable

* "Virtual" shader functions will allow user modifications to core functions (like lighting), and potentially lead to a template-style material system, where users can provide "hooks" that will be called at the right point in the pipeline

* Language interop: mix and match glsl and wgsl, so bevy's pbr pipeline features could be accessed from your glsl material shader, or utils written for glsl could be used in wgsl code. we're hopeful that this can extend to spirv (and rust-gpu) as well

* More cool stuff we haven't thought of yet. Being able to inspect and modify shaders at runtime is very powerful and makes a lot of things possible!

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author
