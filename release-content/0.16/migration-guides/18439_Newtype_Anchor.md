The anchor component has been changed from an enum to a struct newtyping a `Vec2`. The `Custom` variant has been removed, instead to construct a custom `Anchor` use its tuple constructor:

```rust
Sprite {
     anchor: Anchor(Vec2::new(0.25, 0.4)),
     ..default()
}
```

The other enum variants have been replaced with corresponding constants:

- `Anchor::BottomLeft` to `Anchor::BOTTOM_LEFT`
- `Anchor::Center` to `Anchor::CENTER`
- `Anchor::TopRight` to `Anchor::TOP_RIGHT`
- .. and so on for the remaining variants
