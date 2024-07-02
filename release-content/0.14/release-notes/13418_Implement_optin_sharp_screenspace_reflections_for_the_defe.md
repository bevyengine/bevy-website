<!-- Implement opt-in sharp screen-space reflections for the deferred renderer, with improved raymarching code. -->
<!-- https://github.com/bevyengine/bevy/pull/13418 -->

{{ compare_slider(
    path="POST_PATH",
    left_title="No SSR",
    left_image="no_ssr.png",
    right_title="SSR",
    right_image="ssr.png"
) }}

[Screen-space reflections](https://lettier.github.io/3d-game-shaders-for-beginners/screen-space-reflection.html) (SSR) approximate real-time reflections by raymarching through the depth buffer and copying samples from the final rendered frame.
Our initial implementation is relatively minimal, to provide a flexible base to build on, but is based on the production-quality [raymarching code by Tomasz Stachowiak](https://gist.github.com/h3r2tic/9c8356bdaefbe80b1a22ae0aaee192db), one of the creators of the indie darling Bevy game [Tiny Glade](https://store.steampowered.com/app/2198150/Tiny_Glade/).
As a result, there are a few caveats to bear in mind:

1. Currently, this feature is built on top of the deferred renderer and is currently only supported in that mode. Forward screen-space reflections are possible albeit uncommon (though e.g. Doom Eternal uses them); however, they require tracing from the previous frame, which would add complexity. This patch leaves the door open to implementing SSR in the forward rendering path but doesn't itself have such an implementation.
2. Screen-space reflections aren't supported in WebGL 2, because they require sampling from the depth buffer, which `naga` can't do because of a bug (`sampler2DShadow` is incorrectly generated instead of `sampler2D`; this is the same reason why depth of field is disabled on that platform).
3. No temporal filtering or blurring is performed at all. For this reason, SSR currently only operates on very low-roughness / smooth surfaces.
4. We don't perform acceleration via the hierarchical Z-buffer and reflections are traced at full resolution. As a result, you may notice performance issues depending on your scene and hardware.

To add screen-space reflections to a camera, insert the [`ScreenSpaceReflectionsSettings`] component.
In addition to [`ScreenSpaceReflectionsSettings`], [`DepthPrepass`], and [`DeferredPrepass`] must also be present for the reflections to show up.
Conveniently, the [`ScreenSpaceReflectionsBundle`] bundles these all up for you!
While the [`ScreenSpaceReflectionsSettings`] comes with sensible defaults, it also contains several settings that artists can tweak.

[`ScreenSpaceReflectionsBundle`]: https://docs.rs/bevy/0.14/bevy/pbr/struct.ScreenSpaceReflectionsBundle.html
[`ScreenSpaceReflectionsSettings`]:https://docs.rs/bevy/0.14/bevy/pbr/struct.ScreenSpaceReflectionsSettings.html
[`DepthPrepass`]: https://docs.rs/bevy/0.14/bevy/core_pipeline/prepass/struct.DepthPrepass.html
[`DeferredPrepass`]: https://docs.rs/bevy/0.14/bevy/core_pipeline/prepass/struct.DeferredPrepass.html
