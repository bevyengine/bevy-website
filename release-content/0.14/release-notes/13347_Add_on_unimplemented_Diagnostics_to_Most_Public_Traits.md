Bevy takes full advantage of the powerful type system Rust provides, but with that power can often come confusion when even minor mistakes are made.

```rust
use bevy::prelude::*;

struct MyResource;

fn main() {
    App::new()
        .insert_resource(MyResource)
        .run();
}
```

Running the above will produce a compiler error, let's see why...

<details>
<summary>Click to expand...</summary>

```
error[E0277]: the trait bound `MyResource: Resource` is not satisfied
   --> example.rs:6:32
    |
6   |     App::new().insert_resource(MyResource).run();
    |                --------------- ^^^^^^^^^^ the trait `Resource` is not implemented for `MyResource`
    |                |
    |                required by a bound introduced by this call
    |
    = help: the following other types implement trait `Resource`:
              AccessibilityRequested
              ManageAccessibilityUpdates
              bevy::a11y::Focus
              DiagnosticsStore
              FrameCount
              bevy::prelude::Axis<T>
              WinitActionHandlers
              ButtonInput<T>
            and 127 others
note: required by a bound in `bevy::prelude::App::insert_resource`
   --> /bevy/crates/bevy_app/src/app.rs:537:31
    |
537 |     pub fn insert_resource<R: Resource>(&mut self, resource: R) -> &mut Self {
    |                               ^^^^^^^^ required by this bound in `App::insert_resource`
```

</details>

The compiler suggests we use a different type that implements `Resource`, or that we implement the trait on `MyResource`. The former doesn't help us at all, and the latter fails to mention the available derive macro.

With the release of Rust 1.78, Bevy can now provide more direct messages for certain types of errors during compilation using [diagnostic attributes](https://blog.rust-lang.org/2024/05/02/Rust-1.78.0.html#diagnostic-attributes).

```
error[E0277]: `MyResource` is not a `Resource`
   --> example.rs:6:32
    |
6   |     App::new().insert_resource(MyResource).run();
    |                --------------- ^^^^^^^^^^ invalid `Resource`
    |                |
    |                required by a bound introduced by this call
    |
    = help: the trait `Resource` is not implemented for `MyResource`
    = note: consider annotating `MyResource` with `#[derive(Resource)]`
    = help: the following other types implement trait `Resource`:
              AccessibilityRequested
...
```

Now, the error message has a more approachable entry point, and a new `note` section pointing to the derive macro for resources. If Bevy's suggestions _aren't_ the solution to your problem, the rest of the compiler error is still included just in case.

These diagnostics have been implemented for various traits across Bevy, and we hope to improve this experience as new features are added to Rust. For example, we'd really like to improve the experience of working with tuples of `Component`'s, but we're not quite there yet. You can read more about this change in the [pull request](https://github.com/bevyengine/bevy/pull/13347) and associated [issue](https://github.com/bevyengine/bevy/issues/12377).
