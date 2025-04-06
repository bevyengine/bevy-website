When it comes to pushing hardware to support larger levels and more detailed meshes, "simply draw less stuff" is a tried-and-true strategy.
These techniques are collectively referred to as "culling": we can save work whenever we can determine "this doesn't need to be drawn" for cheaper than it would cost to draw the thing despite its pixels later being overwritten completely.

**Occlusion culling** is the idea that we don't need to draw something that's completely blocked by other opaque objects,
from the perspective of the camera.
That makes sense! We don't need to draw a car parked on the other side of a building (even if it's within the range used for fustrum culling), and we *never* need to draw all of the teeth inside of a character's closed mouth.

Before you worry too much: we're already checking for this category of wasted work in some form, via the use of a **depth prepass**.
A depth prepass renders the scene with a simplified shader, recording only the depth (distance from the camera) of each object into the 2-dimensional depth buffer.
We can check the resulting depth buffer, count how many pixels are associated with each object, and then discard objects whose pixels were completely overwritten.
As a result, we can avoid most fragment shading costs (dominated by textures and lighting) for occluded objects, but the vertex shading overhead is still present, in addition to the inherent cost of this fragment testing process.

We can do better than that.
As [PR #17413] lays out, we're adopting the modern two-phase occlusion culling (in contrast to a traditional potentially visible sets design) already used by our virtual geometry rendering, because it works well with the GPU-driven rendering architecture (cold specialization, retained bins) that we've established during this cycle!

For now, this feature is marked as experimental, due to known precision issues that can mark meshes as occluded even when they're not.
In practice, we're not convinced that this is a serious concern, so please let us know how it goes!
To try out the new mesh occlusion culling, add the [`DepthPrepass`] and [`OcclusionCulling`] components to your camera.

As we said above, the check has to be faster than simply drawing the thing: occlusion culling won't be faster on all scenes.
Small scenes, or those using simpler non-PBR rendering are particularly likely to be slower with occlusion culling turned on.
Like always: you need to measure your performance to improve it.

If you're a rendering engineer who'd like to help us resolve these precision issues and stabilize this feature, we're looking to learn from Hans-Kristian Arntzen design in [Granite].
Chime in at [issue #14062] (and read our [Contributing Guide]) and we can help you get started.

[PR #17413]: https://github.com/bevyengine/bevy/pull/17413
[`DepthPrepass`]: https://dev-docs.bevyengine.org/bevy/core_pipeline/prepass/struct.DepthPrepass.html
[`OcclusionCulling`]: https://dev-docs.bevyengine.org/bevy/render/experimental/occlusion_culling/struct.OcclusionCulling.html
[issue #14062]: https://github.com/bevyengine/bevy/issues/14062
[Granite]: https://github.com/Themaister/Granite
[Contributing Guide]: https://bevyengine.org/learn/contribute/introduction/
