The glTF 3D model file format allows single mesh to be associated with multiple materials. For example, a teapot may consist of a single mesh, yet each part can have a different material. When a single mesh is assigned multiple materials, it is divided into several primitive nodes, with each primitive assigned a unique material.

```json
{
  "meshes": [
    {
      "name": "Cube",
      "primitives": [
        {
          "attributes": { "POSITION": 0, "NORMAL": 1, "TEXCOORD_0": 2 },
          "indices": 3,
          "material": 0
        },
        {
          "attributes": { "POSITION": 4, "NORMAL": 5, "TEXCOORD_0": 6 },
          "indices": 7,
          "material": 1
        },
        {
          "attributes": { "POSITION": 8, "NORMAL": 9, "TEXCOORD_0": 10 },
          "indices": 11,
          "material": 2
        },
        {
          "attributes": { "POSITION": 12, "NORMAL": 13, "TEXCOORD_0": 14 },
          "indices": 15,
          "material": 3
        }
      ]
    }
  ]
}
```

In Bevy 0.14 and before, these primitives are named using the format "Mesh.Index", which complicates querying. A new component [GltfMaterialName](https://docs.rs/bevy/0.15.0-rc.1/bevy/gltf/struct.GltfMaterialName.html) is now added to each primitive node that has a material, letting you quickly look up the primitive by using the this component with the material name.

```rust
fn find_top_material_and_mesh(
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
    time: Res<Time>,
    mat_query: Query<(
        &MeshMaterial3d<StandardMaterial>,
        &Mesh3d,
        &GltfMaterialName,
    )>,
) {
    for (mat_handle, mesh_handle, name) in mat_query.iter() {
        // locate the material and associated submesh by name
        if name.0 == "Top" {
            if let Some(material) = materials.get_mut(mat_handle) {
                // ...
            }

            if let Some(mesh) = meshes.get_mut(mesh_handle) {
                // ...
            }
        }
    }
}
```