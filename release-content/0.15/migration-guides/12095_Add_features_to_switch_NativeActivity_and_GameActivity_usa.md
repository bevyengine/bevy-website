`cargo-apk` has been replaced with `cargo-ndk`. To build shared object files for the target
architecture, your command will look like the following example:
```shell
cargo ndk -t arm64-v8a -o android_example/app/src/main/jniLibs build --package bevy_mobile_example
```
(replace target and project name as required).

Bevy may require the `libc++_shared.so` library to run on Android. This can be manually obtained
from NDK source, or NDK describes a
[`build.rs`](https://github.com/bbqsrc/cargo-ndk?tab=readme-ov-file#linking-against-and-copying-libc_sharedso-into-the-relevant-places-in-the-output-directory)
approach.

After configuration, builds can be launched using `./gradlew build`. Android Studio may also be
used. Note that build output paths have changed. APK builds can be found under
`app/build/outputs/apk`).

Applications that still require `NativeActivity` should add the `android-native-activity` feature.
