[Winit v0.30] changed its API to support a trait based architecture instead of a plain event-based one. Bevy 0.14 now implements that new architecture, making the event loop handling easier to follow.

[Winit v0.30]: https://docs.rs/winit/0.30.0/winit/changelog/v0_30/index.html

It's now possible to define a custom `winit` user event that can be used to trigger `App` updates, and that can be read inside systems to trigger specific behaviours. This is particularly useful when sending events from outside the `winit` event loop and manage them inside Bevy systems (see the `window/custom_winit_event.rs` example).

The `UpdateMode` enum now accepts only two values: `Continuous` and `Reactive`. The latter exposes 3 new properties to enable reactivity to device, user, or window events. The previous `UpdateMode::Reactive` is now equivalent to `UpdateMode::reactive()`, while `UpdateMode::ReactiveLowPower` maps to `UpdateMode::reactive_low_power()`.

The `ApplicationLifecycle` has been renamed to `AppLifecycle`, and it now contains the possible values of the application state inside the event loop:

- `Idle`: the loop has not started yet.
- `Running` (previously `Started`): the loop is running.
- `WillSuspend`: the loop is going to be suspended.
- `Suspended`: the loop is suspended.
- `WillResume`: the loop is going to be resumed.

Note that `ApplicationLifetime::Resumed` has been removed in favor of just `Running`.
