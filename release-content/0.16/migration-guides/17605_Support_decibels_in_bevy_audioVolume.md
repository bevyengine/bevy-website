Audio volume can now be configured using decibel values, as well as using linear scale values. To enable this, some types and functions in `bevy::audio` have changed. First, `Volume` is now an enum with `Linear` and `Decibels` variants:

```rust
// 0.15
let v = Volume(1.0);

// 0.16
let volume = Volume::Linear(1.0);

// Alternatively, you can use decibels instead.
let volume = Volume::Decibels(0.0);
```

`Volume::Linear` is equivalent to the old `f32` volume.

With this change, `AudioSinkPlayback`'s volume-related methods (`volume()` and `set_volume()`) and `GlobalVolume` now deal in `Volume`s rather than `f32`s.

Finally, `Volume::ZERO` has been renamed to the more semantically correct `Volume::SILENT`. This is because 0 decibals is equivalent to "normal volume", which could lead to confusion with the old naming.
