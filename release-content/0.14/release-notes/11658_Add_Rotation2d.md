Ever wanted to work with rotations in 2D and get frustrated with having to choose between quaternions and a raw `f32`?
Us too!

We've added a convenient [`Rotation2d`](https://dev-docs.bevyengine.org/bevy/math/struct.Rotation2d.html) type for you, with plenty of helper methods.
Feel free to replace that helper type you wrote, and submit little PRs for any useful functionality we're missing.

`Rotation2d` is a great complement to the [`Dir2`](https://dev-docs.bevyengine.org/bevy/math/struct.Dir2.html) type (formerly `Direction2d`).
The former represents an angle, while the latter is a unit vector.
These types are similar but not interchangeable, and the choice of representation depends heavily on the task at hand. You can rotate a direction using `direction = rotation * Dir2::X`. To recover the rotation, use `Rotation2d::radians(Dir2::X.angle_between(direction))`.

While these types aren't used widely within the engine yet, we *are* aware of your pain
and are evaluating [proposals](https://github.com/bevyengine/rfcs/pull/82) for how we can make working with transforms in 2D more straightforward and pleasant.
