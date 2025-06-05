Geometric shapes find a variety of applications in game development, ranging from rendering simple items to the screen for display / debugging to use in
colliders, physics, raycasting, and more.

For this, geometric shape primitives were [introduced in Bevy 0.13](https://bevy.org/news/bevy-0-13/#primitive-shapes), and work on this area has continued with Bevy 0.14, which brings the addition of 
[`Triangle3d`] and [`Tetrahedron`] 3D primitives, along with [`Rhombus`], [`Annulus`], [`Arc2d`], [`CircularSegment`], and [`CircularSector`] 2D 
primitives. As usual, these each have methods for querying geometric information like perimeter, area, and volume, and they all support meshing (where 
applicable) as well as gizmo display. 

[`Triangle3d`]: https://docs.rs/bevy/0.14.0/bevy/math/primitives/struct.Triangle3d.html
[`Tetrahedron`]: https://docs.rs/bevy/0.14.0/bevy/math/primitives/struct.Tetrahedron.html
[`Rhombus`]: https://docs.rs/bevy/0.14.0/bevy/math/primitives/struct.Rhombus.html
[`Annulus`]: https://docs.rs/bevy/0.14.0/bevy/math/primitives/struct.Annulus.html
[`Arc2d`]: https://docs.rs/bevy/0.14.0/bevy/math/primitives/struct.Arc2d.html
[`CircularSegment`]: https://docs.rs/bevy/0.14.0/bevy/math/primitives/struct.CircularSegment.html
[`CircularSector`]: https://docs.rs/bevy/0.14.0/bevy/math/primitives/struct.CircularSector.html
