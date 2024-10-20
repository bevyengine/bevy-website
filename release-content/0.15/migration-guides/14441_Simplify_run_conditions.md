Some run conditions have been simplified.

```rust
// Before:
app.add_systems(Update, (
    system_0.run_if(run_once()),
    system_1.run_if(resource_changed_or_removed::<T>()),
    system_2.run_if(resource_removed::<T>()),
    system_3.run_if(on_event::<T>()),
    system_4.run_if(any_component_removed::<T>()),
));

// After:
app.add_systems(Update, (
    system_0.run_if(run_once),
    system_1.run_if(resource_changed_or_removed::<T>),
    system_2.run_if(resource_removed::<T>),
    system_3.run_if(on_event::<T>),
    system_4.run_if(any_component_removed::<T>),
));
```
