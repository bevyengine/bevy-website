Bevy's `Transform` represents the translation, rotation, and scale of an `Entity` and is widely used for the purpose of positioning entities in the world. The `Transform` type includes a number of associated functions for constructing, rotating, and otherwise manipulating a `Transform` value. If your `Entity` has a "front" side and a "top" side, then these functions are one way to determine which direction the front and top are facing.

One of these functions is [`look_to`](https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.look_to). `look_to` tries to match the [`Transform::forward`](https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.forward) and [`Transform::up`](https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.up) values for your `Entity` to the additional forward and up directions you give the function as arguments. This results in a potential rotation to match your entity's local `forward` axis (your "front") to the direction you've chosen, and your entity's local `up` axis (your "top") to the second direction you've chosen.

```rust
// point the "front" in the global X axis direction
// point the "top" in the global Y axis direction
transform.look_to(Vec3::X, Vec3::Y);
```

If you spawned in a shape (such as a cube) programmatically, it is very likely that you used the default `forward` and `up` axes, which are `-local_z()` and `local_y` respectively. This is both not necessarily true, for example if you imported an art asset from Blender and a human chose a different "front" and "top" when modelling, and also sometimes you want to operate on different axis anyway, like when you're operating a spaceship.

For that more generic case, new in 0.14, there is [`Transform::align`](https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.align).

`Transform::align` is like `look_to` but it is more generic and allows you to specify any local axis you want to use for the main and secondary axes. This allows you to do things like point the front of a spaceship at a planet you're heading toward while keeping the right wing pointed in the direction of another ship. or point the top of a ship in the direction of the tractor beam pulling it in, while the front rotates to match the bigger ship's direction.

Given a ship where we're going to use the front of the ship and the right wing local axes.

```rust
// point the local negative-z axis in the global Y axis direction
// point the local x-axis in the global Z axis direction
transform.align(Vec3::NEG_Z, Vec3::Y, Vec3::X, Vec3::Z)
```

![before calling Transform::align](align-before-move.png)

`align` will move it to match the desired positions as closely as possible. Note that not all rotations can be constructed and the [documentation](https://docs.rs/bevy/0.14/bevy/transform/components/struct.Transform.html#method.align) explains what happens in such scenarios.

![after calling Transform::align](align-after-move.png)
