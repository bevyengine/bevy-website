You can configure the behavior of spawned audio with the `PlaybackMode` enum. One of its variants, `PlaybackMode::Despawn`, would despawn the entity when the audio finished playing.

There was previously a bug where this would only despawn the entity and not its children. This has been fixed, so now `despawn_recursive()` is called when the audio finishes.

If you relied on this behavior, consider using `PlaybackMode::Remove` to just remove the audio components from the entity or `AudioSink::empty()` to check whether any audio is finished and manually `despawn()` it.
