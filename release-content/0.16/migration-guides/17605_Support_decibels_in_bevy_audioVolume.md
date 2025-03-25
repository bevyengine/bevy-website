Audio volume can now be configured using decibel values, as well as using linear scale values. To enable this, some types and functions in `bevy_audio` have changed.

- `Volume` is now an enum with `Linear` and `Decibels` variants.

Before:

```rust
let v = Volume(1.0);
```

After:

```rust
let volume = Volume::Linear(1.0);
let volume = Volume::Decibels(0.0); // or now you can deal with decibels if you prefer
```

- `Volume::ZERO` has been renamed to the more semantically correct `Volume::SILENT` because `Volume` now supports decibels and “zero volume” in decibels actually means “normal volume”.
- The `AudioSinkPlayback` trait’s volume-related methods now deal with `Volume` types rather than `f32`s. `AudioSinkPlayback::volume()` now returns a `Volume` rather than an `f32`. `AudioSinkPlayback::set_volume` now receives a `Volume` rather than an `f32`. This affects the `AudioSink` and `SpatialAudioSink` implementations of the trait. The previous `f32` values are equivalent to the volume converted to linear scale so the  `Volume:: Linear` variant should be used to migrate between `f32`s and `Volume`.
- The `GlobalVolume::new` function now receives a `Volume` instead of an `f32`.
