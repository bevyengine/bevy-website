<!-- Add support for experimental WESL shader source -->
<!-- https://github.com/bevyengine/bevy/pull/17953 -->

Bevy continues to live life on the edge, the bleeding edge, of graphics technology. Today, Bevy supports [WESL](https://wesl-lang.dev/) shaders!

Most Bevy shaders are written in [WGSL](https://www.w3.org/TR/WGSL/), a modern shader language built for simplicity. But while WGSL is pretty simple as far as shader languages go, it also leaves a lot to be desired in terms of higher level features. Currently, Bevy has its own extended version of WGSL, which adds support for conditional compiling, importing between files, and other useful features.

WESL is a brand new shader language that extends WGSL (often in ways similar to Bevy's approach) and aims to bring common language conveniences to the GPU. WESL not only includes conditional compiling and importing between files, but it is also growing to support generics, package managers, and more.

It's important to note that WESL is still relatively early in development, and not all of its features are fully functional yet, nor are all its features supported in Bevy (yet). For that reason, WESL support is gated behind the cargo feature `shader_format_wesl`, which is disabled by default.

Despite the additional features, WESL is easy to layer on top of existing WGSL shaders. This is because it is a superset of WGSL (WGSL is valid WESL). That makes it easy to migrate existing WGSL to WESL, though it's worth mentioning that the Bevy's own "extended WGSL syntax" will need to be ported to its WESL counterparts. The WESL team (who helped write these notes!) is actively listening to feedback, and so is Bevy. If you do choose to use WESL in addition to or in replacement of WGSL, your thoughts, feature requests, and any pain points you encounter can be shared [here](https://github.com/wgsl-tooling-wg/wesl-rs).

If you're interested in trying WESL, check out the new [Material - WESL](https://bevyengine.org/examples/shaders/shader-material-wesl/) example. Before using this in a production environment, be sure to check out the original [PR](https://github.com/bevyengine/bevy/pull/17953) for a full list of caveats.

WESL is an exciting frontier for shader languages. You can track their progress and plans [here](https://wesl-lang.dev/).
