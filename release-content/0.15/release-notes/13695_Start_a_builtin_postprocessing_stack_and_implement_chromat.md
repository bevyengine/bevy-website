<!-- Start a built-in postprocessing stack, and implement chromatic aberration in it. -->
<!-- https://github.com/bevyengine/bevy/pull/13695 -->

We've added [chromatic aberration](https://en.wikipedia.org/wiki/Chromatic_aberration), which is a common postprocessing effect that simulates lenses that fail to focus all colors of light to a single point. It's often used for impact effects and/or horror games. Our implementation uses the technique from Inside (Gjøl & Svendsen 2016), which allows the developer to customize the particular color pattern to achieve different effects.

![chromatic aberration](chromatic_aberration.png)

To use it, add the [`ChromaticAberration`](https://docs.rs/bevy/0.15/bevy/core_pipeline/post_process/struct.ChromaticAberration.html) component to your camera:

```rust
commands.spawn((Camera3d::default(), ChromaticAberration));
```