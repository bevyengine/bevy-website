<!-- Add Immutable `Component` Support -->
<!-- https://github.com/bevyengine/bevy/pull/16372 -->

Bevy users have been idly considering the idea of "immutable components" for years:
data that users simply could not modify once it was inserted due to some kind of trait or constant on the [`Component`] trait.
While this is a nice enough tool for communicating a library author's intent, it doesn't do a whole lot beyond simply making your fields private and not providing setters (at least it helps with [`mem::swap`] shenanigans).
And so the idea languished, with too much complexity for too little benefit,
until we borrowed [hooks and observers] from [flecs].

For better or worse, immutable components have a critical loophole: even if we never hand out a `&mut MyComponent`,
users can simply overwrite the component (or remove it and then reinsert a different one).
Not so immutable now!

While this may seem like a bug, it turns out that it's actually a feature.
If the *only* way to modify a component's value is to reinsert it,
lifecycle hooks and observers *always* fire.

Because hooks and observers can modify the [`World`] in arbitrary ways,
we can combine them with immutable components to uphold critical, complex invariants.
While obviously slower than a simple mutation operation,
it's *much* harder to mess up.

Let's see how it's done:

```rust
#[derive(Component)]
#[component(immutable)]
pub struct SumMe(pub u32);

// We want to ensure that this global view of our components is *always* correct
#[derive(Resource)]
struct TotalSum(pub u32);

// We need a pair of observers to keep track of the sum
// This will trigger when spawning or inserting a SumMe component
fn add_when_inserting(mut trigger: Trigger<OnInsert, SumMe>, query: Query<&SumMe>, mut total_sum: ResMut<TotalSum>){
    let added_entity = trigger.entity();
    let sum_me = query.get(added_entity);
    total_sum.0 += sum_me.0;
}

// Hooks would work fine too: but they're intended for "inherent" constructor/destructor style logic
// This will trigger when despawning or removing a SumMe component
fn subtract_when_removing(mut trigger: Trigger<OnRemove, SumMe>, query: Query<&SumMe>, mut total_sum: ResMut<TotalSum>){
    let removed_entity = trigger.entity();
    let sum_me = query.get(removed_entity);
    total_sum.0 -= sum_me.0;
}

// Changing this to `&mut SumMe` would fail to compile!
fn modify_values(query: Query<(Entity, &SumMe)>, mut commands: Commands){
    for (entity, sum_me) in query.iter(){
        // We can read the value, but not write to it.
        let current_value = sum_me.0;
        // This will overwrite: indirectly mutating the value
        // and triggering both observers: removal first, then insertion
        commands.entity(entity).insert(sum_me.0 + 1);
    }
}
```

If you're working with direct world access rather than commands, consider using the [`World::modify_component`] method to make this pattern a bit faster amd easier.

While immutable components are a niche tool, they're a great fit for rarely mutated (or small count) components where correctness is key.
For example, Bevy 0.16 uses immutable components (and hooks!) as the foundation of our shiny new relationships to ensure that both sides of the relationship (e.g. [`ChildOf`] and [`Children`]) stay in sync.

We're keen to develop a first-class indexing solution using these new tools, and excited to hear about your ideas. Stay tuned; we've only scratched the surface here!

[`Component`]: https://dev-docs.bevyengine.org/bevy/ecs/component/trait.Component.html
[`mem::swap`]: https://doc.rust-lang.org/std/mem/fn.swap.html
[flecs]: https://github.com/SanderMertens/flecs
[`World`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.World.html
[`World::modify_component`]: https://dev-docs.bevyengine.org/bevy/prelude/struct.World.html#method.modify_component
[`ChildOf`]: https://dev-docs.bevyengine.org/bevy/ecs/hierarchy/struct.ChildOf.html
[`Children`]: https://dev-docs.bevyengine.org/bevy/ecs/hierarchy/struct.Children.html
