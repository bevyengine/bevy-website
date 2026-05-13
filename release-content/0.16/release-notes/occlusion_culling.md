**Occlusion culling** is the idea that we don't need to draw something that's completely blocked by other opaque objects, from the perspective of the camera. For example: we don't need to draw a person hidden behind a wall, even if they're within the range used for frustum culling.

Bevy already has an optional [Depth Prepass](/news/bevy-0-10/#depth-and-normal-prepass), which renders a simple version of the scene and captures a 2D depth buffer. This can then be used to skip hidden objects in the more expensive main pass. However this doesn't skip the vertex shading overhead, and the depth checks in the fragment shader also add overhead.

In **Bevy 0.16**, we've added modern [two-phase occlusion culling](https://medium.com/@mil_kru/two-pass-occlusion-culling-4100edcad501) (in contrast to a traditional "potentially visible sets" design). This approach was already used by our [virtual geometry](/news/bevy-0-14/#virtual-geometry-experimental) rendering system, and works quite well with the GPU-driven rendering architecture that we've established during this cycle! For more details on our implementation, [check out this PR](https://github.com/bevyengine/bevy/pull/17413).

For now, this feature is marked as experimental, due to known precision issues that can mark meshes as occluded even when they're not.
In practice, we're not convinced that this is a serious concern, so please let us know how it goes!
To try out the new mesh occlusion culling, add the [`DepthPrepass`] and [`OcclusionCulling`] components to your camera.

An important note: occlusion culling won't be faster on all scenes. Small scenes, or those using simpler non-PBR rendering are particularly likely to be slower with occlusion culling turned on. Enabling occlusion culling incurs overhead ... the work that it skips must be more expensive than the cost of running the checks for it to be worth it!

Like always: you need to measure your performance to improve it.

If you're a rendering engineer who'd like to help us resolve these precision issues and stabilize this feature, we're looking to borrow from Hans-Kristian Arntzen's design in [Granite].
Chime in at [issue #14062] (and read our [Contributing Guide]) and we can help you get started.

[`DepthPrepass`]: https://docs.rs/bevy/0.16/bevy/core_pipeline/prepass/struct.DepthPrepass.html
[`OcclusionCulling`]: https://docs.rs/bevy/0.16/bevy/render/experimental/occlusion_culling/struct.OcclusionCulling.html
[issue #14062]: https://github.com/bevyengine/bevy/issues/14062
[Granite]: https://github.com/Themaister/Granite
[Contributing Guide]: https://bevy.org/learn/contribute/introduction/
