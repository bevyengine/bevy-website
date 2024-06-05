The custom UserEvent is now renamed as WakeUp, used to wake up the loop if anything happens outside the app (a new [custom_user_event](https://github.com/bevyengine/bevy/pull/13366/files#diff-2de8c0a8d3028d0059a3d80ae31b2bbc1cde2595ce2d317ea378fe3e0cf6ef2d) shows this behavior.

The internal `UpdateState` has been removed and replaced internally by the AppLifecycle. When changed, the AppLifecycle is sent as an event.

The `UpdateMode` now accepts only two values: `Continuous` and `Reactive`, but the latter exposes 3 new properties to enable reactive to device, user or window events. The previous `UpdateMode::Reactive` is now equivalent to `UpdateMode::reactive()`, while `UpdateMode::ReactiveLowPower` to `UpdateMode::reactive_low_power()`.

The `ApplicationLifecycle` has been renamed as `AppLifecycle`, and now contains the possible values of the application state inside the event loop: 

- `Idle`: the loop has not started yet
- `Running` (previously called `Started`): the loop is running
- `WillSuspend`: the loop is going to be suspended
- `Suspended`: the loop is suspended
- `WillResume`: the loop is going to be resumed

Note: the `Resumed` state has been removed since the resumed app is just running.

Finally, now that `winit` enables this, it extends the `WinitPlugin` to support custom events.
