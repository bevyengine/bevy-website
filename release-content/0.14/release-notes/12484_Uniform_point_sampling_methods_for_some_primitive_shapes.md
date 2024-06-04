The ability to perform a variety of random sampling operations has been added to `bevy_math`, gated behind the `rand` feature. These come in a couple of flavors.

First, one can sample random points from the boundaries and interiors of a variety of mathematical primitives:
![Image of several primitives side-by-side with points randomly sampled from their interiors][sampling-primitives]

In code, these can be sampled in a couple different ways, using either the `sample_interior`/`sample_boundary` or `interior_dist`/`boundary_dist` APIs:
```rust
use bevy::math::prelude::*;
use rand::{Rng, SeedableRng, rngs::StdRng};

// Make a Sphere of radius 1.5:
let sphere = Sphere::new(1.5);

// Instantiate an Rng:
let rng = &mut StdRng::from_entropy();

// Using these, sample a random point from the interior of this sphere:
let interior_pt: Vec3 = sphere.sample_interior(rng);
// or from the boundary:
let boundary_pt: Vec3 = sphere.sample_boundary(rng);

// Or, if we want a lot of points, we can use a Distribution instead...
// to sample 100000 random points from the interior:
let interior_pts: Vec<Vec3> = sphere.interior_dist().sample_iter(rng).take(100000).collect();
// or 100000 random points from the boundary:
let boundary_pts: Vec<Vec3> = sphere.boundary_dist().sample_iter(rng).take(100000).collect();
```
(Note that these methods explicitly require an [`Rng`](https://docs.rs/rand/0.8.5/rand/trait.Rng.html).)

The currently supported shapes are as follows:

2D: `Circle`, `Rectangle`, `Triangle2d`, `Annulus`, `Capsule2d`.

3D: `Sphere`, `Cuboid`, `Triangle3d`, `Tetrahedron`, `Cylinder`, `Capsule3d`, and extrusions of sampleable 2D shapes (`Extrusion`). 

---

Separately, the direction types (`Dir2`, `Dir3`, `Dir3A`) and quaternions (`Quat`) can now be constructed randomly using `from_rng`:
```rust
use bevy::math::prelude::*;
use rand::{random, Rng, SeedableRng, rngs::StdRng, distributions::Standard};

// Intantiate an Rng:
let rng = &mut StdRng::from_entropy();

// Get a random direction:
let direction = Dir3::from_rng(rng);

// Similar, but requires left-hand type annotations or inference:
let another_direction: Dir3 = rng.gen();

// Using `random` to grab a value using implicit thread-local rng:
let yet_another_direction: Dir3 = random();
```

[sampling-primitives]: sampling_primitives.png
