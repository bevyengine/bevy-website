The `AppExit` event is now an enum that represents whether the code exited successfully or not. If you construct it, you must now specify `Success` or `Error`:

```rust
// Before
fn send_exit(mut writer: EventWriter<AppExit>) {
    writer.send(AppExit);
}

// After
fn send_exit(mut writer: EventWriter<AppExit>) {
    writer.send(AppExit::Success);
    // Or...
    writer.send(AppExit::Error(NonZeroU8::new(1).unwrap()));
}
```

If you subscribed to this event in a system, consider `match`ing whether it was a success or an error:

```rust
// Before
fn handle_exit(mut reader: EventReader<AppExit>) {
    for app_exit in reader.read() {
        // Something interesting here...
    }
}

// After
fn handle_exit(mut reader: EventReader<AppExit>) {
    for app_exit in reader.read() {
        match *app_exit {
            AppExit::Success => {
                // Something interesting here...
            },
            AppExit::Error(exit_code) => panic!("App exiting with an error! (Code: {exit_code})"),
        }
    }
}
```

Furthermore, `App::run` now returns `AppExit` instead of the unit type `()`. Since `AppExit` implements [`Termination`](https://doc.rust-lang.org/stable/std/process/trait.Termination.html), you can now return it from the main function.

```rust
// Before
fn main() {
    App::new().run()
}

// After
fn main() -> AppExit {
    App::new().run()
}

// After (alternative)
fn main() {
    // If you want to ignore `AppExit`, you can add a semicolon instead. :)
    App::new().run();
}
```
