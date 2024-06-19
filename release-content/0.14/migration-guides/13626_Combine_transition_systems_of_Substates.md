The `apply_state_transition` system is no longer public. The easiest way to migrate your systems that depended on it for ordering is to create a custom schedule.

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
