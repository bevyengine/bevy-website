Manually clamp the various colour components yourself if this behaviour is still required.

```rust
fn clamped_srgba(color: Srgba) -> Srgba {
    Srgba {
        red: color.red.clamp(0., 1.),
        green: color.green.clamp(0., 1.),
        blue: color.blue.clamp(0., 1.),
        alpha: color.alpha.clamp(0., 1.),
    }
}
```
