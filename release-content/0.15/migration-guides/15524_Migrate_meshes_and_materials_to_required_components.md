Asset handles for meshes and mesh materials must now be wrapped in the `Mesh2d` and `MeshMaterial2d` or `Mesh3d` and `MeshMaterial3d` components for 2D and 3D respectively. Raw handles as components no longer render meshes.

Additionally, `MaterialMesh2dBundle`, `MaterialMeshBundle`, and `PbrBundle` have been deprecated. Instead, use the mesh and material components directly.

Previously:

```rust
commands.spawn(MaterialMesh2dBundle {
    mesh: meshes.add(Circle::new(100.0)).into(),
    material: materials.add(Color::srgb(7.5, 0.0, 7.5)),
    transform: Transform::from_translation(Vec3::new(-200., 0., 0.)),
    ..default()
});
```

Now:

```rust
commands.spawn((
    Mesh2d(meshes.add(Circle::new(100.0))),
    MeshMaterial2d(materials.add(Color::srgb(7.5, 0.0, 7.5))),
    Transform::from_translation(Vec3::new(-200., 0., 0.)),
));
```

If the mesh material is missing, a white default material is now used. Previously, nothing was rendered if the material was missing.

The `WithMesh2d` and `WithMesh3d` query filter type aliases have also been removed. Simply use `With<Mesh2d>` or `With<Mesh3d>`.
