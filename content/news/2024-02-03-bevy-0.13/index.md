+++
title = "Bevy 0.13"
date = 2024-02-03
[extra]
author = "Bevy Contributors"
image = "TODO.gif"
show_image = true
image_subtitle = "TODO"
image_subtitle_link = "TODO"

+++

Thanks to **TODO** contributors, **TODO** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.13** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.
And to see what the engine has to offer hands-on, check out the entries in the [latest Bevy Jam](https://itch.io/jam/bevy-jam-4/entries), including the winner [That's a lot of beeeeees](https://andrewb330.itch.io/thats-a-lot-of-beeeeees)

To update an existing Bevy App or Plugin to **Bevy 0.13**, check out our [0.12 to 0.13 Migration Guide](/learn/migration-guides/0.12-0.13/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **First-party primitive shapes:** basic shapes are a core building block of both game engines and video games: we've added a polished collection of them for you to use!
* **System stepping:** completely pause and advance through your game frame-by-frame or system-by-system to interactively debug game logic, all while rendering continues to update.
* **Dynamic queries:** refining queries from within systems is extremely expressive, and is the last big puzzle piece for runtime-defined types and third-party modding and scripting integration.
* **Automatically inferred command flush points:** tired of reasoning about where to put `apply_deferred` and confused about why your commands weren't being applied? Us too! Now, Bevy's scheduler uses ordinary `.before` and `.after` constraints and inspects the system parameters to automatically infer (and deduplicate) synchronization points.
* **Slicing, tiling and nine-patch sprites:** ninepatch layout is a popular tool for smoothly scaling stylized tilesets and UIs. Now in Bevy!
* **Lightmaps:** the first step towards baked global illumination: a fast, popular and pretty lighting technique.
* **Animation interpolation modes:** Bevy now supports non-linear interpolation modes in exported glTF animations.

## Primitive shapes

<div class="release-feature-authors">authors: @Jondolf, @NiseVoid</div>

Geometric shapes are used all across game development, from primitive mesh shapes and debug gizmos to physics colliders and raycasting. Despite being so commonly used across several domains, Bevy hasn't really had reusable shape representations other than the rendering-specific mesh shapes like the [`UVSphere`] in [`bevy_render`].

This is changing in Bevy 0.13 with the introduction of first-party **primitive shapes**! They are lightweight geometric primitives designed for maximal interoperability and reusability, allowing Bevy and third-party plugins to use the same set of basic shapes and increase cohesion within the ecosystem. See the original [RFC][Primitive RFC] for more details.

The built-in [collection of primitives] is already quite sizeable:

| 2D                              | 3D                              |
| ------------------------------- | ------------------------------- |
| `Rectangle`                     | `Cuboid`                        |
| `Circle`                        | `Sphere`                        |
| `Ellipse`                       | -                               |
| `Triangle2d`                    | -                               |
| `Plane2d`                       | `Plane3d`                       |
| `Line2d`                        | `Line3d`                        |
| `Segment2d`                     | `Segment3d`                     |
| `Polyline2d`, `BoxedPolyline2d` | `Polyline3d`, `BoxedPolyline3d` |
| `Polygon`, `BoxedPolygon`       | -                               |
| `RegularPolygon`                | -                               |
| `Capsule2d`                     | `Capsule3d`                     |
| -                               | `Cylinder`                      |
| -                               | `Cone`                          |
| -                               | `ConicalFrustum`                |
| -                               | `Torus`                         |

More primitives will be added in future releases.

Some use cases for primitive shapes include meshing, gizmos, bounding volumes, colliders, and raycasting functionality. Several of these have landed in 0.13 already!

[`UVSphere`]: https://dev-docs.bevyengine.org/bevy/prelude/shape/struct.UVSphere.html
[`bevy_render`]: https://dev-docs.bevyengine.org/bevy/render/index.html
[Primitive RFC]: https://github.com/bevyengine/rfcs/blob/main/rfcs/12-primitive-shapes.md
[collection of primitives]: https://dev-docs.bevyengine.org/bevy/math/primitives/index.html

### Meshing

<div class="release-feature-authors">authors: @Jondolf</div>

TODO

* Explain current meshing
* Show primitive meshing API
* Screenshot of `2d_shapes` example
* Mention 2D polygon meshing and 3D meshing not being implemented yet

### Gizmos

<div class="release-feature-authors">authors: @RobWalt</div>

TODO

* Show primitive gizmo API
* Screenshot or video of primitive shapes drawn using gizmos

### Bounding Volumes

<div class="release-feature-authors">authors: @Jondolf, @NiseVoid</div>

In game development there are many use cases for fast spatial checks. For example: getting all entities that are in the camera's view frustum, getting all entities near the player, or finding pairs of physics objects that might collide. To speed up such checks, bounding volumes are used to approximate more complex shapes. This version of bevy adds a public API for bounding volumes. The volumes can be created manually, or generated from primitives shapes.

There are two traits for working with bounding volumes: `BoundingVolume` and `IntersectsVolume`. These traits can be used by crates to generalize over multiple bounding volume types and intersection tests. The `BoundingVolume` trait has two implementations per dimension, one for axis-aligned bounding boxes and one for bounding balls. The `IntersectsVolume` trait takes a `BoundingVolume` as argument, and allows you to test for intersections against your bounding volumes.

Here is an example of how bounding volumes are constructed, and how an intersection test is performed:
```rust
// We create an axis-aligned bounding box that is centered at position
let position = Vec2::new(100., 50.);
let half_size = Vec2::splat(20.);
let aabb = Aabb2d::new(position, half_size);

// We create a bounding circle that is centered at position
let position = Vec2::new(80., 70.);
let radius = 30.;
let bounding_circle = BoundingCircle::new(position, radius);

// `BoundingCircle` and `Aabb2d` implement `IntersectsVolume` against both theirselves and eachother
let intersects = bounding_circle.intersects(&aabb);
```

There are also two traits for the generation of bounding volumes: `Bounded2d` and `Bounded3d`. An example of these in action:

```rust
// We create a primitive, a hexagon in this case
let hexagon = RegularPolygon::new(50., 6);

let translation = Vec2::new(50., 200.);
let rotation = PI / 2.; // Rotation in radians

// Now we can get an Aabb2d or BoundingCircle from this primitive.
// These methods are part of the `Bounded2d` trait.
let aabb = hexagon.aabb_2d(translation, rotation);
let circle = hexagon.bounding_circle(translation, rotation);
```

[TODO: video of intersection tests]
<div style="font-size: 1.0rem" class="release-feature-authors">
    The new example (bounding_2d) that showcases various intersection tests.
</div>

#### Ray casting and volume casting

The bounding volumes also supports ray casting and volume casting. Ray casting tests if the bounding volumes intersect with the ray. Rays are cast from an origin, towards a direction until a maximum distance. Volume casts work in a similar way, but function as if moving a volume along the ray. This support is provided trough the new `RayCast2d`, `RayCast3d`, `AabbCast2d`, `AabbCast3d`, `CircleCast`, and `SphereCast` types.

Some newly introduced types can be very useful when reasoning about ray casts. The old `Ray` type has been split into `Ray2d` and `Ray3d`, these types are defined by an origin and a direction. New `Direction2d` and `Direction3d` types have been added, these are normalized vectors pointing in a direction.


## System Stepping

<div class="release-feature-authors">authors: @TODO</div>

## Dynamic queries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Entity optimizations

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WorldQuery trait split

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Automatically inserted sync points

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Input for one-shot systems

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WGPU upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Texture atlas rework

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Sprite slicing and tiling

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Exposure settings

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Minimal reflection probes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light maps

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light RenderLayers

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Approximate indirect specular occlusion

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Unload render assets from RAM

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Bind group layout entries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Type-safe labels for the `RenderGraph`

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Camera-driven UI

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Winit upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Animation interpolation

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## `Animatible` trait

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## gltF extensions

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Extensionless asset support

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Gizmo configuration

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## <a name="what-s-next"></a>What's Next?

We have plenty of work in progress! Some of this will likely land in **Bevy 0.14**.

Check out the [**Bevy 0.14 Milestone**](https://github.com/bevyengine/bevy/milestone/20) for an up-to-date list of current work that contributors are focusing on for **Bevy 0.14**.

* **More editor experimentation:** TODO
* **bevy_dev_tools:** TODO
* **A revised scene format:** TODO
* **bevy_ui improvements:** TODO
* **The steady march towards relations:** TODO
* **Animation blending:** TODO
* **Irradiance volumes:** TODO

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the 185 contributors that made this release (and associated docs) possible! In random order:

TODO: add contributors

## Full Changelog

The changes mentioned above are only the most appealing, highest impact changes that we've made this cycle.
Innumerable bug fixes, documentation changes and API usability tweaks made it in too.
For a complete list of changes, check out the PRs listed below.

TODO: add full changelog, sorting by area.
