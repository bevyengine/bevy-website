The `UpdateAssets` schedule has been removed. If you add systems to this schedule, move them to run on `PreUpdate`. (You may need to configure the ordering with `system.before(...)` and `system.after(...)`.)

```rust
// 0.13
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(UpdateAssets, my_system)
    .run();

// 0.14
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(PreUpdate, my_system)
    .run();
```

Furthermore, `AssetEvents` has been changed from a `ScheduleLabel` to a `SystemSet` within the `First` schedule.

```rust
// 0.13
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(AssetEvents, my_system)
    .run();

// 0.14
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(First, my_system.in_set(AssetEvents))
    .run();
```
