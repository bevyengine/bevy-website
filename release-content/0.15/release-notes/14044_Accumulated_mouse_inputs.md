<!-- Accumulated mouse inputs -->
<!-- https://github.com/bevyengine/bevy/pull/14044 -->

"How much has the player moved their mouse this frame" is a natural question for games when the player is trying to aim or scroll a map.
Unfortunately, the operating system, and thus [`winit`], only provides us with a stream of events, in the form of individual [`MouseMotion`] events.

To get the summarized information (and the equivalent [`MouseScroll`]) information that most game systems care about, you had to sum them yourself.

```rust
pub fn accumulate_mouse_motion_system(
    mut mouse_motion_event: EventReader<MouseMotion>,
    mut accumulated_mouse_motion: ResMut<AccumulatedMouseMotion>,
) {
    let mut delta = Vec2::ZERO;
    for event in mouse_motion_event.read() {
        delta += event.delta;
    }
    accumulated_mouse_motion.delta = delta;
}
```

Bevy now does this for you, exposed in the new [`AccumulatedMouseMotion`] and [`AccumulatedMouseScroll`] resources.

[`winit`]: https://docs.rs/winit/latest/winit/
[`MouseMotion`]: https://docs.rs/bevy/0.15.0/bevy/input/mouse/struct.MouseMotion.html
[`MouseScroll`]: https://docs.rs/bevy/0.15.0/bevy/input/mouse/struct.MouseScroll.html
[`AccumulatedMouseMotion`]: https://docs.rs/bevy/0.15.0/bevy/input/mouse/struct.AccumulatedMouseMotion.html
[`AccumulatedMouseScroll`]: https://docs.rs/bevy/0.15.0/bevy/input/mouse/struct.AccumulatedMouseScroll.html
