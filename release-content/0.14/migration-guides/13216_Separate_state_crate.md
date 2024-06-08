States were moved to a separate crate which is gated behind the `bevy_state` feature. Projects that use state but don't use Bevy's `default-features` will need to add that feature to their `Cargo.toml`.

Projects that use `bevy_ecs` directly and use states will need to add the `bevy_state` crate as a dependency.

Projects that use `bevy_app` directly and use states will need to add the `bevy_state` feature.

If you do not use `DefaultPlugins`, you will need to add the `StatesPlugin` manually to your app.
