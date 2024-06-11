
The gltf file format allows passing additional user defined metadata in the *extras* properties, and
in addition to the gltf extras at the primitive/node level , Bevy now has specific GltfExtras for:
- scenes: **SceneGltfExtras** injected at the scene level if any
- meshes: **MeshGltfExtras**, injected at the mesh level if any
- materials: **MaterialGltfExtras**, injected at the mesh level if any: ie if a mesh has a material that has gltf extras, the component will be injected there.

You can now easily query for these specific extras

```rust
fn check_for_gltf_extras(
    gltf_extras_per_entity: Query<(
        Entity,
        Option<&Name>,
        Option<&GltfSceneExtras>,
        Option<&GltfExtras>,
        Option<&GltfMeshExtras>,
        Option<&GltfMaterialExtras>,
    )>,
) {
    // use the extras' data 
    for (id, name, scene_extras, extras, mesh_extras, material_extras) in
        gltf_extras_per_entity.iter()
    {

    }
}

```

This makes passing information from programs such as Blender to Bevy via gltf files more spec compliant, and more practical !

