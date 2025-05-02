Over the years, the trend in real-time rendering has increasingly been to move work from the CPU to the GPU. One of the latest developments in this area has been *GPU-driven rendering*, in which the GPU takes a representation of the scene and essentially works out what to draw on its own.

**Bevy 0.16** adds GPU-driven rendering support for most "standard" 3D mesh rendering, including skinned meshes. This dramatically reduces the amount of CPU time that the renderer needs for larger scenes. It's automatically enabled on platforms that support it; unless your application hooks into the rendering pipeline, upgrading to **Bevy 0.16** will automatically enable GPU-driven rendering for your meshes. This joins the support that **Bevy 0.14** and **0.15** added for GPU-driven rendering of [Virtual Geometry](/news/bevy-0-14/#virtual-geometry-experimental).

On Activision's "heavy" hotel section of the [Caldera scene](https://github.com/Activision/caldera) from Call of Duty Warzone, **Bevy 0.16** with GPU-driven rendering performs roughly 3x better than **Bevy 0.15**! (this includes *all* optimizations between these releases)

![Caldera scene rendered in Bevy](caldera.jpg)

On a mobile Nvidia 4090 with Vulkan / Linux, **Bevy 0.16** runs the scene at 10.16ms (~101 FPS), compared to 33.55ms (~30 FPS) on **Bevy 0.15**. Massive wins!

### Overview: CPU-Driven Rendering

To explain how GPU-driven rendering operates, it's easiest to first describe how CPU-driven rendering works:

1. The CPU determines the objects that are visible, via frustum culling and perhaps occlusion culling.
2. For each such object:
    * The CPU sends the object's transform to the GPU, possibly in addition to other data such as joint weights.
    * The CPU tells the GPU where the mesh data is.
    * The CPU writes the material data to the GPU.
    * The CPU tells the GPU where textures and other buffers needed to render the objects are (light data, etc.)
    * The CPU issues a drawcall.
    * The GPU renders the object.

### Overview: GPU-Driven Rendering

In contrast, GPU-driven rendering in Bevy works like this:

1. The CPU supplies a single buffer containing transform information for all objects to the GPU, so that shaders can process many objects at once.
2. If new objects have been spawned since the last frame, the CPU fills out tables specifying where the mesh data for the new objects are.
3. If materials have been modified since the last frame, the CPU uploads information about those materials to the GPU.
4. The CPU creates lists of objects to be rendered this frame. Each object is simply referenced by an integer ID, so these lists are small. The number of lists depends on the size and complexity of the scene, but there are rarely more than 15 such lists even for large scenes.
5. For each such list:
    * The CPU issues a *single* drawcall.
    * The GPU processes all objects in the list, determining which ones are truly visible.
    * The GPU renders each such visible object.

For large scenes that may have tens of thousands of objects, GPU-driven rendering frequently results in a reduction in CPU rendering overhead of 3× or more. It's also necessary for occlusion culling, because of the GPU transform step (5(b) above).

### GPU-Driven Rendering Techniques

Internally, GPU-driven rendering is less a single technique than a combination of several techniques. These include:

* *Multi-draw indirect* (MDI), a GPU API that allows multiple meshes to be drawn in a single drawcall, the details of which the GPU provides by filling out tables in GPU memory. In order to use MDI effectively, Bevy uses a new subsystem, the *mesh allocator*, which manages the details of packing meshes together in GPU memory.
  * *Multi-draw indirect count* (MDIC), an extension to multi-draw indirect that allows the GPU to determine the *number* of meshes to draw with minimal overhead.
* *Bindless resources*, which allow Bevy to supply the textures (and other resources) for many objects as a group, instead of having to bind textures one-by-one on the CPU. These resources are managed by a new subsystem known as the *material allocator*.
* *GPU transform and cull*, which allows Bevy to compute the position and visibility of every object from the camera's perspective on the GPU instead of on the CPU.
* The *retained render world*, which allows the CPU to avoid processing and uploading data that hasn't changed since the last frame.
* *Cached pipeline specialization*, which leverages Bevy's component-level change detection to more quickly determine when the rendering state for meshes is unchanged from the previous frame.

### GPU-Driven Platform Compatibility

At the moment, not all platforms offer full support for this feature. The following table summarizes the platform support for the various parts of GPU-driven rendering:

| OS      | Graphics API | GPU transform | Multi-draw & GPU cull | Bindless resources |
|---------|--------------|---------------|-----------------------|--------------------|
| Windows | Vulkan       | ✅            | ✅                   | ✅                |
| Windows | Direct3D 12  | ✅            | ❌                   |❌                 |
| Windows | OpenGL       | ✅            |❌                    |❌                 |
| Linux   | Vulkan       | ✅            | ✅                   | ✅                |
| Linux   | OpenGL       | ✅            |❌                    |❌                 |
| macOS   | Metal        | ✅            |❌                    |➖¹                |
| iOS     | Metal        | ✅            |❌                    |➖¹                |
| Android | Vulkan       | ➖²            |➖²                    |➖²               |
| Web     | WebGPU       | ✅            |❌                    |❌                 |
| Web     | WebGL 2       | ❌            |❌                    |❌                 |

¹ Bevy does support bindless resources on Metal, but the limits are currently significantly lower, potentially resulting in more drawcalls.

² Some Android drivers that are known to exhibit bugs in Bevy's workloads are denylisted and will cause Bevy to fall back to CPU-driven rendering.

In most cases, you don't need to do anything special in order for your application to support GPU-driven rendering. There are two main exceptions:

1. Materials with custom WGSL shaders will continue to use CPU-driven rendering by default. In order for your materials to use GPU-driven rendering, you'll want to use the new `#[bindless]` feature on `AsBindGroup`. See the `AsBindGroup` documentation and the `shader_material_bindless` example for more details. If you're using `ExtendedMaterial`, check out the new `extended_material_bindless` example.
2. Applications and plugins that hook into the renderer at a low level will need to be updated to support GPU-driven rendering. The newly-updated `custom_phase_item` and `specialized_mesh_pipeline` examples may prove useful as a guide to do this.

### What's Next for GPU-Driven Rendering?

Bevy's current GPU-driven rendering isn't the end of the story. There's a sizable amount of potential future work to be done:

* **Bevy 0.16** only supports GPU-driven rendering for the 3D pipeline, but the techniques are equally applicable to the 2D pipeline. Future versions of Bevy should support GPU-driven rendering for 2D mesh rendering, sprites, UI, and so on.
* Bevy currently draws objects with morph targets using CPU-driven rendering. This is something we plan to address in the future. Note that the presence of objects with morph targets doesn't prevent objects that don't have morph targets from being drawn with GPU-driven rendering.
* In the future, a portion of the GPU-driven rendering infrastructure could be ported to platforms that don't support the full set of features, offering some performance improvements on those platforms. For example, even on WebGL 2 the renderer could make use of the material allocator to pack data more efficiently.
* We're watching new API features, such as [Vulkan device generated commands] and [Direct3D 12 work graphs], with interest. These would allow future versions of Bevy to offload even more work to the GPU, such as sorting of transparent objects. While figuring out how to unify these disparate APIs in a single renderer will be a challenge, the future possibilities in this space are exciting.

If you're interested in any of these tasks, feel free to ask [in our Discord](https://discord.gg/bevy) or via [GitHub issues](https://github.com/bevyengine/bevy/issues).

[Vulkan device generated commands]: https://www.supergoodcode.com/device-generated-commands/

[Direct3D 12 work graphs]: https://devblogs.microsoft.com/directx/d3d12-work-graphs/