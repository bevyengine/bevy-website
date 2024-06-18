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

Furthermore, the `apply_state_transition` system is no longer public. The easiest way to migrate systems that depended on `apply_state_transition` for ordering is to create a custom schedule.

```rust
// 0.13
App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(StateTransition, my_system.after(apply_state_transition))
    .run()

// 0.14
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct AfterStateTransition;

let mut app = App::new();

app.add_plugins(DefaultPlugins)
    .add_systems(AfterStateTransition, my_system);

// Create a new schedule and add it to the app.
let after_state_transition = Schedule::new(AfterStateTransition);
app.add_schedule(after_state_transition);

// Modify the schedule order to make this run after `StateTransition`.
app.world_mut()
    .resource_mut::<MainScheduleOrder>()
    .insert_after(StateTransition, AfterStateTransition);

app.run()
```
