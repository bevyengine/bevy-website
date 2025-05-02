It is now possible to mute audio sinks. Several breaking changes have been introduced to implement this feature.

First, `AudioSinkPlayback::set_volume()` now takes a mutable `&mut AudioSinkPlayback` argument instead of an immutable one. This may require you to update your system parameters:

```rust
// 0.15
fn increase_volume(sink: Single<&AudioSink, With<Music>>) {
    sink.set_volume(sink.volume() + 0.1);
}

// 0.16
fn increase_volume(mut sink: Single<&mut AudioSink, With<Music>>) {
    let current_volume = sink.volume();
    sink.set_volume(current_volume + 0.1);
}
```

Secondly, `PlaybackSettings` has a new `muted` field to specify whether an entity should start muted. You may need to set this field when creating `PlaybackSettings` if you do not use function update syntax (`..default()`).

Finally, if you manually implemented audio muting using an audio sink's volume, you can switch over to using the new `AudioSinkPlayback` methods: `is_muted()`, `mute()`, `unmute()` and `toggle_mute()`.
