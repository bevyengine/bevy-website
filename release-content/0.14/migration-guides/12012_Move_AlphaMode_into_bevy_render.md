`AlphaMode` has been moved from `bevy_pbr` to `bevy_render`. If you import them directly, you will need to update your import statements.

```rust
// 0.13
use bevy::pbr::AlphaMode;

// 0.14
use bevy::render::alpha::AlphaMode;
```
