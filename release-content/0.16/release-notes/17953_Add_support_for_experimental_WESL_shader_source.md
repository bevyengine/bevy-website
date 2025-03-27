<!-- Add support for experimental WESL shader source -->
<!-- https://github.com/bevyengine/bevy/pull/17953 -->

Bevy continues to live life on the edge, the bleeding edge, of graphics technology. Today bevy officially supports [WESL](https://wesl-lang.dev/) shaders!

Most Bevy shaders are written in [WGSL](https://www.w3.org/TR/WGSL/), a modern shader language built for simplicity. But while WGSL is pretty simple as far as shader languages go, it also leaves a lot to be desired in terms of features.

Currently, Bevy putts extra work into it's WGSL shaders to support conditional compiling, importing between files, and other useful features. WESL is a brand new shader language built on WGSL that aims to bring common language conveniences to the GPU. WESL not only includes conditional compiling and importing between files, but it also includes support for generics, package managers, and more.

It's important to note that WESL is still relatively early in development, and not all of its features are fully functional yet, nor are all its features supported in Bevy (yet). For that reason, WESL support is gated behind a feature, `shader_format_wesl`, which is disabled by default.

If you're interested in trying WESL, check out the new "Material - WESL" example. Before using this in a production environment, be sure to check out the original [PR](https://github.com/bevyengine/bevy/pull/17953) for a full list of caviots.

WESL is an exciting frontier of shader languages, and as it grows, Bevy intends to grow with it. You can track their progress and plans [here](https://github.com/wgsl-tooling-wg/wesl-rs).
