Bevy has powerful support for [automatic instancing] for any entities that share the same mesh and material. However, sometimes it can still be useful to reference data that is not the same across all instances of a material. Previously, this required either writing significant amount of custom rendering code or giving up the performance benefits of automatic instancing by creating more materials.

The new `MeshTag` component allows adding a custom `u32` tag to mesh-material entities that can be referenced in the vertex shader for a material. In combination with storage textures or the [`ShaderStorageBuffer` asset] added in Bevy 0.15, this provides a flexible new mechanism to access external data on a per-instance basis or otherwise tag your mesh instances.

Spawn a mesh tag with a mesh-material entity: 
```rust
commands.spawn((
    // Clone a mesh and material handle to enable automatic instancing 
    Mesh3d(mesh_handle.clone()),
    MeshMaterial3d(material_handle.clone()),
    // The mesh tag can be any `u32` that is meaningful to your application, like
    // a particular variant of an enum or an index into some external data 
    MeshTag(1234), 
));
```

Refere
```wgsl
#import bevy_pbr::mesh_functions

@vertex
fn vertex(vertex: Vertex) -> VertexOutput {
    // Lookup the tag for the given mesh
    let tag = mesh_functions::get_tag(vertex.instance_index);

    // Index into a storage buffer, read a storage texture texel, etc...
}

```

[automatic instancing]: https://bevyengine.org/examples/shaders/automatic-instancing/
[`ShaderStorageBuffer` asset]: https://bevyengine.org/news/bevy-0-15/#shader-storage-buffer-asset