States were moved to a separate crate which is gated behind the `bevy_state` feature. Projects that use state but don't use Bevy's `default-features` will need to add this feature to their `Cargo.toml`.

Projects that use `bevy_ecs` directly and use states will need to add the `bevy_state` **crate** as a dependency.

Projects that use `bevy_app` directly and use states will need to add the `bevy_state` **feature**.

If you do not use `DefaultPlugins`, you will need to add the `StatesPlugin` manually to your app.

Users should update imports that referenced the old location.

```rust
// 0.13
use bevy::ecs::schedule::{NextState, OnEnter, OnExit, OnTransition, State, States};
use bevy::ecs::schedule::common_conditions::in_state;

// 0.14
use bevy::state::state::{NextState, OnEnter, OnExit, OnTransition, State, States}
use bevy::state::condition::in_state;
```
