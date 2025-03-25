- The `AudioSinkPlayback` trait now has 4 new methods to allow you to mute audio sinks: `is_muted`, `mute`, `unmute` and `toggle_mute`. You can use these methods on `bevy_audio`’s `AudioSink` and `SpatialAudioSink` components to manage the sink’s mute state.
- `AudioSinkPlayback`’s `set_volume` method now takes a mutable reference instead of an immutable one. Update your code which calls `set_volume` on `AudioSink` and `SpatialAudioSink` components to take a mutable reference. E.g.:

Before:

```rust
fn increase_volume(sink: Single<&AudioSink>) {
    sink.set_volume(sink.volume() + 0.1);
}
```

After:

```rust
fn increase_volume(mut sink: Single<&mut AudioSink>) {
    let current_volume = sink.volume();
    sink.set_volume(current_volume + 0.1);
}
```

- The `PlaybackSettings` component now has a `muted` field which you can use to spawn your audio in a muted state. `PlaybackSettings` also now has a helper method `muted` which you can use when building the component. E.g.: 

```rust
commands.spawn((
    // ...
    AudioPlayer::new(asset_server.load("sounds/Windless Slopes.ogg")),
    PlaybackSettings::LOOP.with_spatial(true).muted(),
));
```
