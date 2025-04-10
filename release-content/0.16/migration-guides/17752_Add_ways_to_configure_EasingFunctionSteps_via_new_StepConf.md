`EaseFunction::Steps` now has a second parameter, `JumpAt`, which can customize jumping behavior. `JumpAt`'s default is `JumpAt::End`, which indicates that the last steps happens when the animation ends.

```rust
// 0.15
let ease_function = EaseFunction::Steps(10);

// 0.16
let ease_function = EaseFunction::Steps(10, JumpAt::default());
```
