**If you have been building your application for iOS:**

Previously, the `#[bevy_main]` attribute created a `main_rs` entry point that most Xcode templates were using to run your Rust code from C. This was found to be unnecessary, as you can simply let Rust build your application as a binary, and run that directly.

You have two options for dealing with this.

#### New, suggested approach

Preferred option is to remove your “compile” and “link” build phases, and instead replace it with a “run script” phase that invokes `cargo build --bin ...`, and moves the built binary to the Xcode path `$TARGET_BUILD_DIR/$EXECUTABLE_PATH`. An example of how to do this can be viewed in [mobile example](https://github.com/bevyengine/bevy/tree/main/examples/mobile).

If you are not sure how to do this, consider one of two ways:

- replace local mobile `game` crate with the one in repo and reapply your changes.
- replicate the changes from [pull request](https://github.com/bevyengine/bevy/pull/14780) in your `mobile` crate.

To make the debugging experience in Xcode nicer after this, you might also want to consider either enabling `panic = "abort"` or to set a breakpoint on the `rust_panic` symbol.

#### Restoring old behaviour

If you’re using additional ObjC code, Swift packages, Xcode customizations, or if it otherwise it makes sense for your use-case to continue link with Xcode, you can revert to the old behavior by adding code below to your `main.rs` file:

```rust
#[cfg(target_os = "ios")]
#[unsafe(no_mangle)]
extern "C" fn main_rs() {
    main()
}
```

Note that the old approach of linking a static library prevents the Rust standard library from doing runtime initialization, so certain functionality provided by `std` might be unavailable (stack overflow handlers, stdout/stderr flushing and other such functionality provided by the initialization routines).
