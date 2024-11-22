<!-- Pack multiple vertex and index arrays together into growable buffers. -->
<!-- https://github.com/bevyengine/bevy/pull/14257 -->

<!-- TODO -->

**Bevy 0.15** changes the way meshes are uploaded to the GPU to greatly improve performance. Instead of submitting vertex and index buffers for every mesh individually as in Bevy 0.14, now they are coalesced into 'slabs' of configurable size. This cuts down on wgpu per-buffer overhead, winning us up to 2x speedups!

The `MeshAllocatorSettings` resource allows tuning slab sizes, growth rate, and cut-offs to best fit your application's needs. The defaults should already be a significant win for most scenes.

WebGL 2 does not support packing vertex buffers together, so only index buffers get combined on this platform.

Some measurements on the [Bistro] scene:

Overall frame time improves from 8.74 ms to 5.53 ms (1.58x speedup)
Render system time improves from 6.57 ms to 3.54 ms (1.86x speedup)
Opaque pass time improves from 4.64 ms to 2.33 ms (1.99x speedup)

[Bistro]: https://github.com/DGriffin91/bevy_bistro_scene