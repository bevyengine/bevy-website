The `AppExit` event is now an enum that represents whether the code exited successfully or not. If you construct it, you must now specify `Success` or `Error`:

```rust
// 0.13
fn send_exit(mut writer: EventWriter<AppExit>) {
    writer.send(AppExit);
}

// 0.14
fn send_exit(mut writer: EventWriter<AppExit>) {
    writer.send(AppExit::Success);
    // Or...
    writer.send(AppExit::Error(NonZeroU8::new(1).unwrap()));
}
```

If you subscribed to this event in a system, consider `match`ing whether it was a success or an error:

```rust
// 0.13
fn handle_exit(mut reader: EventReader<AppExit>) {
    for app_exit in reader.read() {
        // Something interesting here...
    }
}

// 0.14
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

Furthermore, `App::run()` now returns `AppExit` instead of the unit type `()`. Since `AppExit` implements [`Termination`](https://doc.rust-lang.org/stable/std/process/trait.Termination.html), you can now return it from the main function.

```rust
// 0.13
fn main() {
    App::new().run()
}

// 0.14
fn main() -> AppExit {
    App::new().run()
}

// 0.14 (alternative)
fn main() {
    // If you want to ignore `AppExit`, you can add a semicolon instead. :)
    App::new().run();
}
```

Finally, if you configured a custom `App` runner function, it will now have to return an `AppExit`.

```rust
let mut app = App::new();

app.set_runner(|_app| {
    // ...

    // Return success by default, though you may also return an error code.
    AppExit::Success
});
```
