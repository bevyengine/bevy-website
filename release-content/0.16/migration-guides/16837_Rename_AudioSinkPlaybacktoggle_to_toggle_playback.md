`AudioSinkPlayback::toggle()` has been renamed to `toggle_playback()`. This was done to create consistency with the `toggle_mute()` method added in [#16813]. Please update all references to use the new name.

```rust
// 0.15
fn pause(keyboard_input: Res<ButtonInput<KeyCode>>, sink: Single<&AudioSink>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        sink.toggle();
    }
}

// 0.16
fn pause(keyboard_input: Res<ButtonInput<KeyCode>>, sink: Single<&AudioSink>) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        sink.toggle_playback();
    }
}
```

[#16813]: https://github.com/bevyengine/bevy/pull/16813
