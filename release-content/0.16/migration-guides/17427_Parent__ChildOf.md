The `Parent` component has been renamed to `ChildOf` to make it more clear that entities with a `ChildOf` component are children, not parents.

Furthermore, it is now only possible to access the parent `Entity` from `ChildOf::parent()`. The `Deref` implementation has been removed and the `get()` method deprecated.

```rust
// 0.15
let parent = *child_of
// 0.16
let parent = child_of.parent()

// 0.15
let parent = child_of.get()
// 0.16
let parent = child_of.parent()
```
