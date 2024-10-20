[run_fixed_main_schedule](https://docs.rs/bevy/latest/bevy/time/fn.run_fixed_main_schedule.html) is no longer public. If you used to order against it, use the new dedicated `RunFixedMainLoopSystem` system set instead. You can replace your usage of `run_fixed_main_schedule` one for one by `RunFixedMainLoopSystem::FixedMainLoop`, but it is now more idiomatic to place your systems in either `RunFixedMainLoopSystem::BeforeFixedMainLoop` or `RunFixedMainLoopSystem::AfterFixedMainLoop`

Old:

```rust
app.add_systems(
    RunFixedMainLoop,
    some_system.before(run_fixed_main_schedule)
);
```

New:

```rust
app.add_systems(
    RunFixedMainLoop,
    some_system.in_set(RunFixedMainLoopSystem::BeforeFixedMainLoop)
);
```
