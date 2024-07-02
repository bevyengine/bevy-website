[Winit v0.30] changed its API to support a trait based architecture instead of a plain event-based one. Bevy 0.14 now implements that new architecture, making the event loop handling easier to follow.

[Winit v0.30]: https://docs.rs/winit/0.30.0/winit/changelog/v0_30/index.html

It's now possible to define a custom `winit` user event, that can be used to trigger App updates,
and that can be read inside systems to trigger specific behaviors. This is particularly useful to
send events from outside the `winit` event loop and manage them inside Bevy systems
(see the `window/custom_winit_event.rs` example).

[custom_user_event]: https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/window/custom_user_event.rs

The `UpdateMode` enum now accepts only two values: `Continuous` and `Reactive`. The latter exposes 3 new properties to enable reactivity to device, user, or window events. The previous `UpdateMode::Reactive` is now equivalent to `UpdateMode::reactive()`, while `UpdateMode::ReactiveLowPower` maps to `UpdateMode::reactive_low_power()`.

* `Idle`: the loop has not started yet
* `Running` (previously called `Started`): the loop is running
* `WillSuspend`: the loop is going to be suspended
* `Suspended`: the loop is suspended
* `WillResume`: the loop is going to be resumed

Note: the `Resumed` state has been removed since the resumed app is just `Running`.
