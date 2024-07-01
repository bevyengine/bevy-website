**Bevy 0.14** introduces an entirely new group of primitives: extrusions!

An extrusion is a 2D primitive (the base shape) that is *extruded* into a third dimension by some depth. The resulting shape is a prism (or in the special case of the circle, a cylinder).

```rust
// Create an ellipse with width 2 and height 1.
let my_ellipse = Ellipse::from_size(2.0, 1.0);

// Create an extrusion of this ellipse with a depth of 1.
let my_extrusion = Extrusion::new(my_ellipse, 1.);
```

All extrusions are extruded along the Z-axis. This guarantees that an extrusion of depth 0 and the corresponding base shape are identical, just as one would expect.

#### Measuring and Sampling

Since all extrusions with base shapes that implement [`Measured2d`](https://docs.rs/bevy/0.14/bevy/index.html) implement [`Measured3d`](https://docs.rs/bevy/0.14/bevy/index.html), you can easily get the surface area or volume of an extrusion.
If you have an extrusion of a custom 2D primitive, you can simply implement `Measured2d` for your primitive and `Measured3d` will be implemented automatically for the extrusion.

Likewise, you can sample the boundary and interior of any extrusion if the base shape of the extrusion implements [`ShapeSample<Output = Vec2>`](https://docs.rs/bevy/0.14/bevy/index.html) and `Measured2d`. 

```rust
// Create a 2D capsule with radius 1 and length 2, extruded to a depth of 3
let extrusion = Extrusion::new(Capsule2d::new(1.0, 2.0), 3.0);

// Get the volume of the extrusion
let volume = extrusion.volume();

// Get the surface area of the extrusion
let surface_area = extrusion.area();


// Create a random number generator
let mut rng = StdRng::seed_from_u64(4);

// Sample a random point inside the extrusion
let interior_sample = extrusion.sample_interior(&mut rng);

// Sample a random point on the surface of the extrusion
let boundary_sample = extrusion.sample_boundary(&mut rng);
```

#### Bounding

You can also get bounding spheres and Axis Aligned Bounding Boxes (AABBs) for extrusions. If you have a custom 2D primitive that implements `Bounded2d`, you can simply implement [`BoundedExtrusion`](https://docs.rs/bevy/0.14/bevy/index.html) for your primitive. The default implementation will give optimal results but may be slower than a solution fitted to your primitive. 

```rust
struct Heart {
	// ... some properties
}
impl Primitive2d for Heart {}

impl Bounded2d for Heart {
	// ... your implementation for the 2D bounding
}

// Implement bounding for extrusions of hearts 
impl BoundedExtrusion for Heart {
    // You could override the default implementation in here, if you want to
}
```

#### Meshing

Extrusions do not exist in the world of maths only though. They can also be meshed and displayed on the screen!

![selected rendered extrusions](selected_extrusions.png)

And again, adding meshing support for your own primitives is made easy by bevy! You simply need to implement meshing for your 2D primitive and then implement [`Extrudable`](https://docs.rs/bevy/0.14/bevy/index.html) for your 2D primitive's `MeshBuilder`. 

When implementing `Extrudable`, you have to provide information about whether segments of the perimeter of the base shape are to be shaded smooth or flat, and what vertices belong to each of these perimeter segments.

```rust
impl Meshable for Heart {
    type Output = HeartMeshBuilder;

    fn mesh(&self) -> Self::Output {
        Self::Output { heart: *self }
    }
}
struct HeartMeshBuilder {
    /// The heart primitive 
	heart: Heart,
	/// The number of vertices to use for each wing of the heart
	resolution: usize,
}
impl MeshBuilder for HeartMeshBuilder {
    fn build(&self) -> Mesh {
        // ... your implementation for meshing the 2D primitive
    }
}

impl Extrudable for HeartMeshBuilder {
    fn perimeter(&self) -> Vec<bevy::render::mesh::PerimeterSegment> {
        let resolution = self.resolution as u32;
        vec![
			// The left wing of the heart
            PerimeterSegment::Smooth {
				// the normals of the first and last vertices of smooth segments have to be specified manually
                first_normal: Vec2::X,
                last_normal: Vec2::new(-1.0, -1.0).normalize(),
				// These indices are used to index into the `ATTRIBUTE_POSITION` vec of your 2D mesh.
                indices: (0..resolution).collect(),
            },
			// The bottom tip of the heart
            PerimeterSegment::Flat {
                indices: vec![resolution - 1, resolution, resolution + 1],
            },
			// The right wing of the heart
            PerimeterSegment::Smooth {
                first_normal: Vec2::new(1.0, -1.0).normalize(),
                last_normal: Vec2::NEG_X,
                indices: (resolution + 1..2 * resolution).chain([0]).collect(),
            },
        ]
    }
}
```

![a 2D heart primitive and its extrusion](heart_extrusion.png)

The `Extrudable` trait allows you to easily implement meshing for extrusions of custom primitives. Of course, you could also implement meshing manually for your extrusion.

If you want to see a full implementation of this, you can check out the [custom primitives example](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/math/custom_primitives.rs).
