`Command` and `CommandQueue` have been moved from `bevy::ecs::system` to `bevy::ecs::world`. If you import them directly, you will need to update your import statements. (This does not affect you if you just import the prelude.)

```rust
// Before
use bevy::ecs::system::{Command, CommandQueue};

// After
use bevy::ecs::world::{Command, CommandQueue};
```
