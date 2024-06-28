<!-- Hooks: https://github.com/bevyengine/bevy/pull/10756 -->
<!-- Observers: https://github.com/bevyengine/bevy/pull/10839 -->

As much as we love churning through homogeneous blocks of data in a tight loop here at Bevy; not every task is a perfect fit for the straightforward ECS model.
Responding to changes and/or processing events are vital tasks in any application, and games are no exception.

Bevy has accumulated a number of subtly distinct tools to handle this over the years, and in 0.14 is picking up two more.
Let's quickly go over the existing tools to contextualize why adding more is useful:

- Buffered [`Event`]s: multiple-producer, multiple-consumer queues. Flexible and efficient, but requires regular polling as part of a schedule. Events are dropped after two frames.
- Change detection via [`Added`] and [`Changed`]: Granular, automatic, great integration with queries. Linear time checks, no added data.
- [`RemovedComponents`]: A special form of event that is triggered when a component is removed from an entity, or an entity with that component is despawned.

All of these (and systems themselves!) use a ["pull"-style mechanism]: metaphorical events are sent regardless of whether or not anyone is listening, and listeners must periodically poll to ask if anything has changed.
This is a useful pattern, and one we intend to keep around!
By polling, we can process events in batch, getting more context and improving data locality (which makes the CPU go brr).

But it comes with some limitations:

- polling has a small but non-zero overhead every frame
- there is an unavoidable delay between an event being triggered and the response being processed

This delay is the critical problem:

- data (like indexes or hierarchies) exists, even momentarily, in an invalid state
- we can't process arbitrary chains of recursive logic within a single cycle, no matter how carefully we order our event writers and readers

To overcome these limitations, Bevy 0.14 introduces two complementary "push"-style mechanisms inspired by the ever-wonderful C ECS, [flecs]:

1. [**Component Lifecycle Hooks:**](https://dev-docs.bevyengine.org/bevy/ecs/component/struct.ComponentHooks.html) Mutations to the world that automatically occur in response to **lifecycle events**: whenever a component of the given type is added, overwritten or removed.
2. [**Observers:**](https://dev-docs.bevyengine.org/bevy/ecs/observer/struct.Observer.html) On-demand systems that listen to [`Trigger`] events, which commonly target specific entities. These systems are run sequentially and recursively at the next command flush point in response to a trigger being sent.

While hooks are most comparable to constructors and destructors, observers should be used in a manner closer to flexible callbacks.
Hooks are about fundamental behaviors of a type, while observers are for the business logic layered on top.
Let's take a look at the details.

Only one hook per component type can be registered, and it cannot be overwritten.
These are intended for enforcing lower level ECS invariants required to use components (e.g. hierarchy correctness).
Hooks always run before observers and cannot be removed and so are more suitable for maintaining critical safety or correctness invariants.
Additionally, hooks are somewhat faster than observers, as their reduced flexibility means that fewer lookups are involved.

Let's examine a simple example where we care about maintaining invariants: trying to target a specific `Enemy`.

```rust
#[derive(Component)]
struct Target(Option<Entity>);

#[derive(Component)]
struct Targetable {
    targeted_by: Vec<Entity>
};
```

Suppose, through custom commands, [cleverness] or sheer diligence, that `Target` only ever contains entities with the `Targetable` component (at the time of const).
We want to automatically clear the target when it's despawned (or made untargetable): how do we do this?

If we use a pull-based approach (`RemovedComponents` is the most natural here), there can be gaps (within a single frame) between the entity being despawned and the `Target` component being updated.
This can lead to all sorts of bizarre bugs: fun for speedrunners, but not for game developers.

Let's see what this looks like with a hook on `Targetable`:

```rust
// Rather than a derive, let's configure the hooks with a custom implementation of Component
impl Component for Targetable {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        // Whenever this component is removed, or an entity with this component is despawned...
        hooks.on_remove(|mut world, targeted_entity, _component_id|{
            // Grab the data that's about to be removed
            let targetable = world.get::<Targetable>(targeted_entity).unwrap();
            for targeting_entity in targetable.targeted_by {
                // Track down the entity that's targeting us
                let mut targeting = world.get::<Target>(targeting_entity).unwrap();
                // And clear its target, cleaning up any dangling references
                targeting.0 = None;
            }
        })
    }
}
```

Because adding and removing components can only be done in the context of exclusive world access, hooks are always run *immediately*, leaving no opportunity for desynchronization.

By contrast, observers are a flexible tool intended for higher level application logic.
They can listen to the same lifecycle events as hooks, but can also respond to custom, user-defined triggers.
Their advantages over buffered events are clearest when you're targeting a specific entity,
when combined with commands that emit triggers (to avoid ever entering a bad state),
or when you're taking advantage of observers' ability to emit triggers which are then immediately processed, chaining recursively.

Let's examine the API through a simple gameplay-flavored example:

```rust
// Any event type can be used as a trigger
#[derive(Event)]
struct DealDamage {
    damage: u8
}

#[derive(Event)]
struct LoseLife {
    life_lost: u8,
}

#[derive(Event)]
struct PlayerDeath;

// Observers are stored as components on entities,
// and can be set up to watch specific entities.
fn spawn_player(mut commands: Commands) {
    let player_entity = commands
        // Setting up some ordinary components
        .spawn((Player, Life(10), Defense(2)))

        // Now, we're adding some callback-style behavior using observers,
        // watching the entity itself.
        // By attaching the observer to the entity it's watching, we ensure that it gets cleaned up.
        .observe(respond_to_damage_taken)
        .observe(respond_to_life_lost);

}

// We can send triggers using commands (or methods on `World`)
fn attack_player(monster_query: Query<&Damage, With<Monster>>, player_query: Query<Entity, With<Player>>, mut commands: Commands) {
    let player_entity = player_query.single();
    
    for damage in monster_query {
        // We could target multiple entities here just as easily!
        commands.trigger_targets(DealDamage {damage}, player_entity);
    }
}

// Observers use system syntax, with a special `Trigger` parameter as the first param,
// and can request any other `SystemParam` from the world
fn respond_to_damage_taken(trigger: Trigger<DealDamage>, query: Query<&Defense>, mut commands: Commands) {
    // We can access information about the entity responding to the event by reading data from the trigger,
    // and combining it with additional queries
    let defense = query.get(trigger.entity).unwrap_or_default();
    let damage = trigger.event().damage;
    let life_lost = damage.0.saturating_sub(defense.0);
    // Observers can be chained into each other, by sending more triggers using commands
    commands.trigger_targets(trigger.entity)
}

fn respond_to_losing_life(trigger: Trigger<LoseLife>, mut life_query: Query<&mut Life>, player_query: Query<Entity, With<Player>>, mut commands: Commands) {
    let mut life = life_query.single_mut(trigger.entity);
    let life_lost = trigger.event().life_lost;
    life.0 = life.0.saturating_sub(life_lost);

    if player_query.contains(trigger.entity){
        // Triggers can be sent globally, targeting no entity in particular
        commands.trigger(PlayerDeath);
    }
}

app
    .add_systems(Startup, spawn_player)
    .add_systems(Update, attack_player)
    // Similarly, observers can also be registered globally, listening to any matching event,
    // regardless of its entity target
    .observe(|_trigger: Trigger<PlayerDeath>, app_exit: EventWriter<AppExit>| {
        println!("You died. Game over!")
        app_exit.send_default();    
    });
```

In the future, we intend to use hooks and observers to [replace `RemovedComponents`], [make our hierarchy management more robust], create a first-party replacement for [`bevy_eventlistener`] as part of our UI work and [build out relations].
These are powerful, abstract tools: we can't wait to see the mad science the community cooks up!

When you're ready to get started, check out the [`component hooks`] and [`observers`] examples for more API details.

[`Event`]: https://dev-docs.bevyengine.org/bevy/ecs/event/trait.Event.html
[`Added`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.Added.html
[`Changed`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.Changed.html
[`RemovedComponents`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.RemovedComponents.html
["pull"-style mechanism]: https://dev.to/anubhavitis/push-vs-pull-api-architecture-1djo
[flecs]: https://www.flecs.dev/flecs/
[`Trigger`]: https://dev-docs.bevyengine.org/bevy/ecs/observer/struct.Trigger.html
[cleverness]: https://github.com/bevyengine/bevy/issues/1634
[replace `RemovedComponents`]: https://github.com/bevyengine/bevy/issues/13928
[make our hierarchy management more robust]: https://github.com/bevyengine/bevy/issues/12235
[`bevy_eventlistener`]: https://github.com/aevyrie/bevy_eventlistener
[build out relations]: https://github.com/bevyengine/rfcs/pull/79
[`component hooks`]: https://github.com/bevyengine/bevy/blob/main/examples/ecs/component_hooks.rs
[`observers`]: https://github.com/bevyengine/bevy/blob/main/examples/ecs/observers.rs
