`State` has been moved to `bevy::state`. With it, `App::init_state` has been moved from a normal method to an extension trait. You may now need to import `AppExtStates` in order to use this method. (This trait is behind the `bevy_app` feature flag, which you may need to enable.)

```rust
// 0.13
App::new()
    .init_state::<MyState>()
    .run();

// 0.14
use bevy::state::app::AppExtStates as _;

App::new()
    .init_state::<MyState>()
    .run();
```

If you import the prelude (such as with `use bevy::prelude::*`), you do not need to do this.
