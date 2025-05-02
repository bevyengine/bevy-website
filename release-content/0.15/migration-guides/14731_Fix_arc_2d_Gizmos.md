- users have to adjust their usages of `arc_2d`:
  - before: 

```rust
arc_2d(
  pos,
  angle,
  arc_angle,
  radius,
  color
)
```

  - after: 

```rust
arc_2d(
  // this `+ arc_angle * 0.5` quirk is only if you want to preserve the previous behavior 
  // with the new API.
  // feel free to try to fix this though since your current calls to this function most likely
  // involve some computations to counter-act that quirk in the first place
  Isometry2d::new(pos, Rot2::radians(angle + arc_angle * 0.5),
  arc_angle,
  radius,
  color
)
```
