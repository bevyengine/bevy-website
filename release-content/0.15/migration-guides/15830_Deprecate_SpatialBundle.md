`SpatialBundle` is now deprecated, insert `Transform` and `Visibility` instead which will automatically insert all other components that were in the bundle. If you do not specify these values and any other components in your `spawn`/`insert` call already requires either of these components you can leave that one out.

before:

```rust
commands.spawn(SpatialBundle::default());
```

after:

```rust
commands.spawn((Transform::default(), Visibility::default());
```
