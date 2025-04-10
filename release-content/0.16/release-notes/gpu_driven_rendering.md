## GPU-Driven Rendering

Over the years, the trend in real-time rendering has increasingly been to move work from the CPU to the GPU. One of the latest developments in this area has been *GPU-driven rendering*, in which the GPU takes a representation of the scene and essentially works out what to draw on its own. While Bevy 0.15 had some support for GPU-driven rendering for meshlets only, Bevy 0.16 features comprehensive support for this technique, including for skinned meshes. This dramatically reduces the amount of CPU time that the renderer needs for larger scenes. It's automatically enabled on platforms that support it; unless your application hooks into the rendering pipeline, upgrading to Bevy 0.16 will automatically enable GPU-driven rendering for your meshes.

To explain how GPU-driven rendering operates, it's easiest to first describe how CPU-driven rendering works:

1. The CPU determines the objects that are visible, via frustum culling and perhaps occlusion culling.

2. For each such object:

   a. The CPU sends the object's transform to the GPU, possibly in addition to other data such as joint weights.

   b. The CPU tells the GPU where the mesh data is.

   c. The CPU writes the material data to the GPU.

   d. The CPU tells the GPU where textures and other buffers needed to render the objects are (light data, etc.)

   e. The CPU issues a drawcall.

   f. The GPU renders the object.

In contrast, GPU-driven rendering in Bevy works like this:

1. The CPU uploads transform information for all objects at once to the GPU.

2. If new objects have been spawned since the last frame, the CPU fills out tables specifying where the mesh data for the new objects are.

3. If materials have been modified since the last frame, the CPU uploads information about those materials to the GPU.

4. The CPU creates lists of objects to be rendered this frame. Each object is simply referenced by an integer ID, so these lists are small. The number of lists depends on the size and complexity of the scene, but there are rarely more than 15 such lists even for large scenes.

5. For each such list:

    a. The CPU issues a *single* drawcall.

    b. The GPU processes all objects in the list, determining which ones are truly visible.

    c. The GPU renders each such visible object.

For large scenes that may have tens of thousands of objects, GPU-driven rendering frequently results in reduction in CPU rendering overhead of 3× or more. It's also necessary for occlusion culling, because of the GPU transform step (5(b) above).

Internally, GPU-driven rendering is less a single technique than a collection of techniques. These include:

* *Multi-draw indirect* (MDI), a GPU API that allows multiple meshes to be drawn in a single drawcall, the details of which the GPU provides by filling out tables in GPU memory. In order to use MDI effectively, Bevy uses a new subsystem, the *mesh allocator*, which manages the details of packing meshes together in GPU memory.

  - *Multi-draw indirect count* (MDIC), an extension to multi-draw indirect that allows the GPU to determine the *number* of meshes to draw with minimal overhead.

* *Bindless resources*, which allow Bevy to supply the textures (and other resources) for many objects as a group, instead of having to bind textures one-by-one on the CPU. These resources are managed by a new subsystem known as the *material allocator*.

* *GPU transform and cull*, which allows Bevy to compute the position and visibility of every object from the camera's perspective on the GPU instead of on the GPU.

* The *retained render world*, which allows the CPU to avoid processing and uploading data that hasn't changed since the last frame. Bevy 0.16 not only retains entities in the render world from frame to frame but also retains many internal rendering data structures, allowing it to avoid most CPU overhead associated with meshes that didn't change from the previous frame.

Not all platforms offer full support for GPU-driven rendering; for example, Metal (macOS and iOS) currently only has partial support, and WebGL 2 (the browser) has no support for GPU-driven rendering at all. Some platforms, such as Android phones with certain Mali chips, theoretically support GPU-driven rendering, but in practice driver issues prevent it from working reliably. In these cases, Bevy 0.16 automatically works around the missing functionality, or falls back to CPU-driven rendering if that isn't feasible. This is possible because the Bevy renderer is a unified codebase that supports both CPU- and GPU-driven rendering techniques. That is, there is just one renderer, not a CPU-driven renderer and a GPU-driven renderer.

In most cases, you don't need to do anything special in order for your application to support GPU-driven rendering. There are two main exceptions:

1. Materials with custom WGSL shaders will continue to use CPU-driven rendering by default. In order for your materials to use GPU-driven rendering, you'll want to use the new `#[bindless]` feature on `AsBindGroup`. See the `AsBindGroup` documentation and the `shader_material_bindless` example for more details. If you're using `ExtendedMaterial`, check out the new `extended_material_bindless` example.

2. Applications and plugins that hook into the renderer at a low level will need to be updated to support GPU-driven rendering. The newly-updated `custom_phase_item` and `specialized_mesh_pipeline` examples may prove useful as a guide to do this.

Bevy's current GPU-driven rendering isn't the end of the story. There's a sizable amount of potential future work to be done:

* Bevy 0.16 only supports GPU-driven rendering for the 3D pipeline, but the techniques are equally applicable to the 2D pipeline. Future versions of Bevy could support 2D GPU-driven rendering as well.

* Morph targets currently force objects to CPU-driven rendering. This is something we plan to address in the future.

* We're watching new API features, such as [Vulkan device generated commands] and [Direct3D 12 work graphs], with interest. These would allow future versions of Bevy to offload even more work to the GPU, such as sorting of transparent objects. While figuring out how to unify these disparate APIs in a single renderer will be a challenge, the future possibilities in this space are exciting.

If you're interested in any of these tasks, feel free to ask in Discord or via GitHub issues.

[Vulkan device generated commands]: https://www.supergoodcode.com/device-generated-commands/

[Direct3D 12 work graphs]: https://devblogs.microsoft.com/directx/d3d12-work-graphs/