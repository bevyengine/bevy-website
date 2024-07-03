**Bevy 0.14** adds a new [`Transform::align`] function, which is a more general form of [`Transform::look_to`], which allows you to specify any local axis you want to use for the main and secondary axes.

This allows you to do things like point the front of a spaceship at a planet you're heading toward while keeping the right wing pointed in the direction of another ship. or point the top of a ship in the direction of the tractor beam pulling it in, while the front rotates to match the bigger ship's direction.

Lets consider a ship where we're going to use the front of the ship and the right wing as local axes:

![before calling Transform::align](align-before-move.png)

```rust
// point the local negative-z axis in the global Y axis direction
// point the local x-axis in the global Z axis direction
transform.align(Vec3::NEG_Z, Vec3::Y, Vec3::X, Vec3::Z)
```

`align` will move it to match the desired positions as closely as possible:

![after calling Transform::align](align-after-move.png)

 Note that not all rotations can be constructed and the [documentation](https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.align) explains what happens in such scenarios.

[`Transform::look_to`]: https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.look_to
[`Transform::align`]: https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.align
