`GameActivity` is now the default activity for Android projects, replacing `NativeActivity`.
`cargo-apk` has been replaced with `cargo-ndk` since the former is not compatible with
`GameActivity`.

Before:

```sh
rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-apk
```

After:

```sh
rustup target add aarch64-linux-android
cargo install cargo-ndk
```

Shared object files must be now built for the target architecture before launching package builds
with the Gradle wrapper.

Before:

```sh
cargo apk build --package bevy_mobile_example
```

After:

```sh
cargo ndk -t arm64-v8a -o android_example/app/src/main/jniLibs build --package bevy_mobile_example
./android_example/gradlew build
```

(replace target and project name as required). Note that build output paths have changed. APK builds
can be found under `app/build/outputs/apk`).

Android Studio may also be used.

Bevy may require the `libc++_shared.so` library to run on Android. This can be manually obtained
from NDK source, or NDK describes a
[`build.rs`](https://github.com/bbqsrc/cargo-ndk?tab=readme-ov-file#linking-against-and-copying-libc_sharedso-into-the-relevant-places-in-the-output-directory)
approach. A suggested solution is also presented in the Bevy mobile example.

Applications that still require `NativeActivity` should:
1. disable default features in `Cargo.toml`
2. re-enable all default features _except_ `android-game-activity`
3. enable the `android-native-activity` feature
