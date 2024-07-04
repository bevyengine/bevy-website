After several months of hard work, we're super excited to bring you the experimental release of a new virtual geometry feature!

This new rendering feature works much like Unreal Engine 5's Nanite renderer. You can take a very high-poly mesh, preprocess it to generate a [`MeshletMesh`] during build time, and then at runtime render huge amounts of geometry - much more than Bevy's standard renderer can support. No explicit LODs are needed - it's all automatic, and near seamless.

This feature is still a WIP, and comes with several constraints compared to Bevy's standard renderer, so be sure to read the docs and report any bugs you encounter. We still have a lot left to do, so look forward to more performance improvements (and associated breaking changes) in future releases!

Note that this feature does not use GPU "mesh shaders", so older GPUs are compatible for now. However, they are not recommended, and are likely to become unsupported in the near future.

In addition to the below user guide, checkout:

* [The Bevy example for this feature](https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/3d/meshlet.rs)
* [The technical deep dive article by the main author of this feature](https://jms55.github.io/posts/2024-06-09-virtual-geometry-bevy-0-14)

<video controls loop><source src="many_bunnies.mp4" type="video/mp4"/></video>

Users wanting to use virtual geometry should compile with the `meshlet` cargo feature at runtime, and `meshlet_processor` cargo feature at build time for preprocessing meshes into the special meshlet-specific format ([`MeshletMesh`]) the meshlet renderer uses.

Enabling the meshlet feature unlocks a new module: [`bevy::pbr::experimental::meshlet`].

First step, add [`MeshletPlugin`] to your app:

```rust
app.add_plugins(MeshletPlugin);
```

Next, preprocess your [`Mesh`] into a [`MeshletMesh`]. Currently, this needs to be done manually via `MeshletMesh::from_mesh()`(again, you need the `meshlet_processor` feature enabled). This step is fairly slow, and should be done once ahead of time, and then saved to an asset file. Note that there are limitations on the types of meshes and materials supported, make sure to read the docs.

Automatic GLTF/scene conversions via Bevy's asset preprocessing system is planned, but unfortunately did not make the cut in time for this release. For now, you'll have to come up with your own asset conversion and management system. If you come up with a good system, let us know!

Now, spawn your entities. In the same vein as `MeshMaterialBundle`, there's a `MeshletMeshMaterialBundle`, which uses a [`MeshletMesh`] instead of the typical [`Mesh`].

```rust
commands.spawn(MaterialMeshletMeshBundle {
    meshlet_mesh: meshlet_mesh_handle.clone(),
    material: material_handle.clone(),
    transform,
    ..default()
});
```

Lastly, a note on materials. Meshlet entities use the same [`Material`] trait as regular mesh entities, however, the standard material methods are not used. Instead there are 3 new methods: `meshlet_mesh_fragment_shader`, `meshlet_mesh_prepass_fragment_shader`, and `meshlet_mesh_deferred_fragment_shader`. All 3 methods of forward, forward with prepasses, and deferred rendering are supported.

Notice however that there is no access to vertex shaders. Meshlet rendering uses a hardcoded vertex shader that cannot be changed.

The actual fragment shader code for meshlet materials are mostly the same as fragment shaders for regular mesh entities. The key difference is that instead of this:

```rust
@fragment
fn fragment(vertex_output: VertexOutput) -> @location(0) vec4<f32> {
    // ...
}
```

You should use this:

```rust
#import bevy_pbr::meshlet_visibility_buffer_resolve::resolve_vertex_output

@fragment
fn fragment(@builtin(position) frag_coord: vec4<f32>) -> @location(0) vec4<f32> {
    let vertex_output = resolve_vertex_output(frag_coord);
    // ...
}
```

[`MeshletMesh`]: https://docs.rs/bevy/0.14/bevy/pbr/experimental/meshlet/struct.MeshletMesh.html
[`Mesh`]: https://docs.rs/bevy/0.14/bevy/prelude/struct.Mesh.html
[`bevy::pbr::experimental::meshlet`]: https://docs.rs/bevy/0.14/bevy/pbr/experimental/meshlet/index.html
[`Material`]: https://docs.rs/bevy/0.14/bevy/pbr/trait.Material.html
[`MeshletPlugin`]: https://docs.rs/bevy/0.14/bevy/pbr/experimental/meshlet/struct.MeshletPlugin.html
