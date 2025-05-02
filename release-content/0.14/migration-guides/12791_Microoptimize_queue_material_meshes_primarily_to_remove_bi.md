The `primitive_topology` field on `GpuMesh` is now an getter method: `GpuMesh::primitive_topology()`.

For performance reasons, `MeshPipelineKey` has been split into `BaseMeshPipelineKey`, which lives in `bevy::render`, and `MeshPipelineKey`, which lives in `bevy::pbr`. These two may be combined with bitwise-or to produce the final `MeshPipelineKey`.

```rust
let base_pipeline_key = BaseMeshPipelineKey::all();
let pbr_pipeline_key = MeshPipelineKey::all();

let pipeline_key: u64 = base_pipeline_key.bits() | pbr_pipeline_key.bits();
```
