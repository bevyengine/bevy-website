Replace all insertions of `AudioSoucreBundle`, `AudioBundle`, and `PitchBundle` with the `AudioPlayer` component. The other components required by it will now be inserted automatically.

In cases where the generics cannot be inferred, you may need to specify them explicitly. For example:

```rust
commands.spawn(AudioPlayer::<AudioSource>(asset_server.load("sounds/sick_beats.ogg")));
```
