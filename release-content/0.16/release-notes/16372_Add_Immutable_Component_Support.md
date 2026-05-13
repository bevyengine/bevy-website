<!-- Add Immutable `Component` Support -->
<!-- https://github.com/bevyengine/bevy/pull/16372 -->

**Bevy 0.16** adds the concept of **immutable components**: data that cannot be mutated once it is inserted (ex: you cannot query for `&mut MyComponent`). The only way to modify an immutable component is to insert a new instance on top.

A component can be made immutable by adding the `#[component(immutable)]` attribute:

```rust
#[derive(Component)]
#[component(immutable)]
pub struct MyComponent(pub u32);
```

By embracing this restriction, we can ensure that component lifecycle hooks and observers (add/remove/insert/replace) capture _every_ change that occurs, enabling them to uphold complex invariants.

To illustrate this, consider the following example, where we track the global sum of all `SumMe` components:

```rust
#[derive(Component)]
#[component(immutable)]
pub struct SumMe(pub u32);

// This will always hold the sum of all `SumMe` components on entities.
#[derive(Resource)]
struct TotalSum(u32);

// This observer will trigger when spawning or inserting a SumMe component
fn add_when_inserting(
    trigger: Trigger<OnInsert, SumMe>,
    query: Query<&SumMe>,
    mut total_sum: ResMut<TotalSum>,
) {
    if let Ok(sum_me) = query.get(trigger.target()) {
        total_sum.0 += sum_me.0;
    }
}

// This observer will trigger when despawning or removing a SumMe component
fn subtract_when_removing(
    trigger: Trigger<OnRemove, SumMe>,
    query: Query<&SumMe>,
    mut total_sum: ResMut<TotalSum>,
) {
    if let Ok(sum_me) = query.get(trigger.target()) {
        total_sum.0 -= sum_me.0;
    }
}

// Changing this to `&mut SumMe` would fail to compile!
fn modify_values(mut commands: Commands, query: Query<(Entity, &SumMe)>) {
    for (entity, sum_me) in query.iter() {
        // We can read the value, but not write to it.
        let current_value = sum_me.0;
        // This will overwrite: indirectly mutating the value
        // and triggering both observers: removal first, then insertion
        commands.entity(entity).insert(SumMe(current_value + 1));
    }
}
```

While immutable components are a niche tool, they're a great fit for rarely mutated (or small count) components where correctness is key.
For example, **Bevy 0.16** uses immutable components (and hooks!) as the foundation of our shiny new **relationships** to ensure that both sides of the relationship (e.g. [`ChildOf`] and [`Children`]) stay in sync.

We're keen to develop a first-class indexing solution using these new tools, and excited to hear about your ideas. Stay tuned; we've only scratched the surface here!

[`ChildOf`]: https://docs.rs/bevy/0.16/bevy/ecs/hierarchy/struct.ChildOf.html
[`Children`]: https://docs.rs/bevy/0.16/bevy/ecs/hierarchy/struct.Children.html
