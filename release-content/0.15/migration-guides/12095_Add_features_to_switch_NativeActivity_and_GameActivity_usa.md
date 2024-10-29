Bevy now uses `GameActivity` as the default `Activity` for  for Android projects, replacing
`NativeActivity`. `NativeActivity` is still available, but has been placed behind a feature flag.

This change updates Bevy to a more modern Android stack, and includes an SDK minimum version bump to
[PlayStore's current version
requirement](https://developer.android.com/distribute/best-practices/develop/target-sdk). We've also
switched to an [NDK](https://developer.android.com/ndk)-based build using
[`cargo-ndk`](https://docs.rs/crate/cargo-ndk/3.5.4). Gradle projects for both `GameActivity` and
`NativeActivity` are provided.

`GameActivity` brings with it improvements to game interaction (`SurfaceView` rendering, improved
touch and input handling), more frequent updates, and access to other parts of the
[JetPack](https://developer.android.com/jetpack) ecosystem. It is better placed to integrate with
Rust code without excessive JNI wrangling. You can read more about `GameActivity`
[here](https://developer.android.com/games/agdk/game-activity).

