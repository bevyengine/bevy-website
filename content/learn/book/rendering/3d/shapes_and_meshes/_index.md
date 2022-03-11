+++
title = "Shapes and meshes"
weight = 6
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

When working with 3d scenes the first thing you will need are basic shapes.

Currently bevy has these builtin shapes:

TODO: Maybe add a screenshot of a 3d scene with each shape???

* Box: An axis-aligned box defined by its minimum and maximum point.
* Cube: A rectangle on the XY plane centered at the origin.
* Quad: A rectangle on the XY plane centered at the origin.
* Plane: A square on the XZ plane centered at the origin.
* Torus: A torus (donut) shape.
* Capsule: A cylinder with hemispheres at the top and bottom
* UV sphere: A sphere made of sectors and stacks.
* Icosphere: A sphere made from a subdivided Icosahedron.

There will eventually be more basic shapes added, but this is what we have access to for now. Don't worry you can easily add more shapes if you need too. In fact, this is exactly what we will do later in this chapter.

## PBR Bundle

To spawn a shape, we can use the `PbrBundle` that will spawn everything we need to have a shape using all the pbr pipeline. This is how it looked in the example.

```rust
commands.spawn_bundle(PbrBundle {
    mesh: meshes.add(Mesh::from(shape::Cube { size: 1.0 })),
    material: materials.add(Color::rgb(0.8, 0.7, 0.6).into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
});
```

### Mesh property

There's a few things here, first let's cover the mesh property.

```rust
// First, we define the shape we want. In this case it's just a cube with a size of 1.0.
let cube_shape = shape::Cube { size: 1.0 };
// We then need to create a `Mesh` from that shape definition
let cube_mesh = Mesh::from(cube_mesh)
// Finally, we need to add the mesh to the mesh asset collection
let mesh_handle = meshes.add(cube_mesh);
// This handle can then be used by the PbrBundle
```

If you want to spawn multiple cubes you can reuse the same mesh without needing to recreate it everytime.

```rust
// This will spawn 3 cubes of different color but all using the same mesh we defined earlier.

// Red cube
commands.spawn_bundle(PbrBundle {
    mesh: mesh_handle.clone_weak(),
    material: materials.add(Color::RED.into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
});
// Green cube
commands.spawn_bundle(PbrBundle {
    mesh: mesh_handle.clone_weak(),
    material: materials.add(Color::GREEN.into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
});
// Blue cube
commands.spawn_bundle(PbrBundle {
    mesh: mesh_handle.clone_weak(),
    material: materials.add(Color::BLUE.into()),
    transform: Transform::from_xyz(0.0, 0.5, 0.0),
    ..default()
});
```

### Material property

TODO use StandardMaterial::from
TODO explain that this will be covered in a future chapter

## Custom Shape

TODO show how to define a plane with a variable number of vertices
