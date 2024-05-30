[`NextState`](https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.NextState.html) is now an enum. If you were constructing it manually or matching on its value, you will need to use the equivalent enum variants.

|0.13|0.14|
|-|-|
|`NextState(Some(S))`|`NextState::Pending(S)`|
|`NextState(None)`|`NextState::Unchanged`|

If you were manually adding `apply_state_transition` to your app, add an exclusive system that runs the `StateTransition` schedule instead.
