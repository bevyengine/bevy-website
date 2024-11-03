Replace all uses of `scale` with `scaling_mode`, keeping in mind that `scale` is (was) a multiplier. For example, replace

```rust
    scale: 2.0,
    scaling_mode: ScalingMode::FixedHorizontal(4.0),

```

with

```rust
    scaling_mode: ScalingMode::FixedHorizontal(8.0),
```
