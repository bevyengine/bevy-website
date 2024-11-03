__Changed__

- Vertex and index buffers for meshes may now be packed alongside other buffers, for performance.
- `GpuMesh` has been renamed to `RenderMesh`, to reflect the fact that it no longer directly stores handles to GPU objects.
- Because meshes no longer have their own vertex and index buffers, the responsibility for the buffers has moved from `GpuMesh` (now called `RenderMesh`) to the `MeshAllocator` resource. To access the vertex data for a mesh, use `MeshAllocator::mesh_vertex_slice`. To access the index data for a mesh, use `MeshAllocator::mesh_index_slice`.
