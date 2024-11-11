<!-- Uniform mesh sampling -->
<!-- https://github.com/bevyengine/bevy/pull/14071 -->

The surfaces of rendering meshes can be randomly sampled. This can be used, for
instance, in order to determine the placements of scenery or particle effects.

This is comprised of two individual pieces:
- The `Mesh::triangles` method allows the extraction of a `Mesh`'s list of 
  triangles (`Triangle3d`).
- The `UniformMeshSampler` type allows the creation of a [`Distribution`] that
  uniformly samples points in space (`Vec3`) from a collection of triangles.

The functionality comes from putting these together:
```rust
let mut rng = StdRng::seed_from_u64(8765309);

// Get an iterator over triangles in the mesh. This can fail if the mesh has
// the wrong format or if its vertex/index data is malformed.
let triangles = my_mesh.triangles().unwrap();

// Construct the distribution. This can fail in some cases - most notably if 
// the mesh surface has zero area.
let distribution = UniformMeshSampler::try_new(triangles).unwrap();

// Get 1000 points uniformly sampled from the surface of the mesh.
let samples: Vec<Vec3> = distribution.sample_iter(&mut rng).take(1000).collect();
```

[`Distribution`]: https://docs.rs/rand/0.8.5/rand/distributions/trait.Distribution.html
