- Replace all usages of [`AppExit`](https://docs.rs/bevy/latest/bevy/app/struct.AppExit.html) with `AppExit::Success` or `AppExit::Failure`.
- Any custom app runners now need to return a `AppExit`. We suggest you return a `AppExit::Error` if any `AppExit` raised was a Error. You can use the new [`App::should_exit`](https://example.org/) method.
- If not exiting from `main` any other way. You should return the `AppExit` from `App::run()` so the app correctly returns a error code if anything fails e.g.

```rust
fn main() -> AppExit {
    App::new()
        //Your setup here...
        .run()
}
```
