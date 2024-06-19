`NextState` has been converted from a unit struct to an enum. If you accessed the internal `Option` directly, whether through `NextState::0` or matching, you will have to update your code to handle this change.

```rust
// 0.13
let state = next_state.0.unwrap();

// 0.14
let NextState::Pending(state) = next_state else { panic!("No pending next state!") };
```

|0.13|0.14|
|-|-|
|`NextState(Some(S))`|`NextState::Pending(S)`|
|`NextState(None)`|`NextState::Unchanged`|
