<!-- Hooks: https://github.com/bevyengine/bevy/pull/10756 -->
<!-- Observers: https://github.com/bevyengine/bevy/pull/10839 -->

As much as we love churning through homogeneous blocks of data in a tight loop here at Bevy, not every task is a perfect fit for the straightforward ECS model.
Responding to changes and/or processing events are vital tasks in any application, and games are no exception.

Bevy already has a number of distinct tools to handle this:

- **Buffered [`Event`]s**: Multiple-producer, multiple-consumer queues. Flexible and efficient, but requires regular polling as part of a schedule. Events are dropped after two frames.
- **Change detection via [`Added`] and [`Changed`]**: Enable writing queries that can respond to added or changed components. These queries linearly scan the change state of components that match the query to see if they have been added or changed.
- **[`RemovedComponents`]**: A special form of event that is triggered when a component is removed from an entity, or an entity with that component is despawned.

All of these (and systems themselves!) use a ["pull"-style mechanism]: events are sent regardless of whether or not anyone is listening, and listeners must periodically poll to ask if anything has changed.
This is a useful pattern, and one we intend to keep around!
By polling, we can process events in batch, getting more context and improving data locality (which makes the CPU go *brr*).

But it comes with some limitations:

- There is an unavoidable delay between an event being triggered and the response being processed
- Polling introduces a small (but non-zero) overhead every frame

This delay is the critical problem:

- Data (like indexes or hierarchies) can exist, even momentarily, in an invalid state
- We can't process arbitrary chain of events of recursive logic within a single cycle

To overcome these limitations, **Bevy 0.14** introduces **Component Lifecycle Hooks** and **Observers**: two complementary "push"-style mechanisms inspired by the ever-wonderful [flecs] ECS.

#### Component Lifecycle Hooks

[Component Hooks](https://docs.rs/bevy/0.14/bevy/ecs/component/struct.ComponentHooks.html) are functions (capable of interacting with the ECS World) registered for a specific component type (as part of the [`Component`] trait impl), which are run automatically in response to "component lifecycle events", such as when that component is added, overwritten, or removed.

For a given component type, only one hook can be registered for a given lifecycle event, and it cannot be overwritten.

Hooks exist to enforce invariants tied to that component (ex: maintaining indices or hierarchy correctness).
Hooks cannot be removed and always take priority over observers: they run before any on-add / on-insert observers, but after any on-remove observers.
As a result, they can be thought of as something closer to constructors & destructors, and are more suitable for maintaining critical safety or correctness invariants.
Hooks are also somewhat faster than observers, as their reduced flexibility means that fewer lookups are involved.

Let's examine a simple example where we care about maintaining invariants: one entity (with a `Target` component) targeting another entity (with a `Targetable` component).

```rust
#[derive(Component)]
struct Target(Option<Entity>);

#[derive(Component)]
struct Targetable {
    targeted_by: Vec<Entity>
};
```

We want to automatically clear the `Target` when the target entity is despawned: how do we do this?

If we were to use the pull-based approach (`RemovedComponents` in this case), there could be a delay between the entity being despawned and the `Target` component being updated. We can remove that delay with hooks!

Let's see what this looks like with a hook on `Targetable`:

```rust
// Rather than a derive, let's configure the hooks with a custom
// implementation of Component
impl Component for Targetable {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        // Whenever this component is removed, or an entity with
        // this component is despawned...
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

#### Observers

Observers are on-demand systems that listen to "triggered" events. These events can be triggered for specific entities *or* they can be triggered "globally" (no entity target).

In contrast to hooks, observers are a flexible tool intended for higher level application logic. They can watch for when user-defined events are triggered.

```rust
#[derive(Event)]
struct Message {
    text: String
}

world.observe(|trigger: Trigger<Message>| {
    println!("{}", message.text);
});
```

Observers are run *immediately* when an event they are watching for is triggered:

```rust
// All registered `Message` observers are immediately run here
world.trigger(Message { text: "Hello".to_string() });
```

If an event is triggered via a [`Command`], the observers will run when the [`Command`] is flushed:

```rust
fn send_message(mut commands: Commands) {
    // This will trigger all `Message` observers when this system's commands are flushed
    commands.trigger(Message { text: "Hello".to_string() } );
}
```

Events can also be triggered with an entity target:

```rust
#[derive(Event)]
struct Resize { size: usize }

commands.trigger_targets(Resize { size: 10 }, some_entity);
```

You can trigger an event for more than one entity at the same time:

```rust
commands.trigger_targets(Resize { size: 10 }, [e1, e2]);
```

A "global" observer will be executed when *any* target is triggered:

```rust
fn main() {
    App::new()
        .observe(on_resize)
        .run()
}

fn on_resize(trigger: Trigger<Resize>, query: Query<&mut Size>) {
    let size = query.get_mut(trigger.entity()).unwrap();
    size.value = trigger.event().size;
} 
```

Notice that observers can use system parameters like [`Query`], just like a normal system.

You can also add observers that only run for *specific* entities:

```rust
commands
    .spawn(Widget)
    .observe(|trigger: Trigger<Resize>| {
        println!("This specific widget entity was resized!");
    });
```

Observers are actually just an entity with the [`Observer`](https://docs.rs/bevy/0.14/ecs/observer/struct.Observer.html) component. All of the `observe()` methods used above are just shorthand for spawning a new observer entity. This is what a "global" observer entity looks like:

```rust
commands.spawn(Observer::new(|trigger: Trigger<Message>| {}));
```

Likewise, an observer watching a specific entity looks like this

```rust
commands.spawn(
    Observer::new(|trigger: Trigger<Resize>| {})
        .with_entity(some_entity)
);
```

This API makes it easy to manage and clean up observers. It also enables advanced use cases, such as sharing observers across multiple targets!

Now that we know a bit about observers, lets examine the API through a simple gameplay-flavored example:

<details>
<summary>Click to expand...</summary>

```rust
use bevy::prelude::*;

#[derive(Event)]
struct DealDamage {
    damage: u8,
}

#[derive(Event)]
struct LoseLife {
    life_lost: u8,
}

#[derive(Event)]
struct PlayerDeath;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Life(u8);

#[derive(Component)]
struct Defense(u8);

#[derive(Component, Deref, DerefMut)]
struct Damage(u8);

#[derive(Component)]
struct Monster;

fn main() {
    App::new()
        .add_systems(Startup, spawn_player)
        .add_systems(Update, attack_player)
        .observe(on_player_death);
}

fn spawn_player(mut commands: Commands) {
    commands
        .spawn((Player, Life(10), Defense(2)))
        .observe(on_damage_taken)
        .observe(on_losing_life);
}

fn attack_player(
    mut commands: Commands,
    monster_query: Query<&Damage, With<Monster>>,
    player_query: Query<Entity, With<Player>>,
) {
    let player_entity = player_query.single();

    for damage in &monster_query {
        commands.trigger_targets(DealDamage { damage: damage.0 }, player_entity);
    }
}

fn on_damage_taken(
    trigger: Trigger<DealDamage>,
    mut commands: Commands,
    query: Query<&Defense>,
) {
    let defense = query.get(trigger.entity()).unwrap();
    let damage = trigger.event().damage;
    let life_lost = damage.saturating_sub(defense.0);
    // Observers can be chained into each other by sending more triggers using commands.
    // This is what makes observers so powerful ... this chain of events is evaluated
    // as a single transaction when the first event is triggered.
    commands.trigger_targets(LoseLife { life_lost }, trigger.entity());
}

fn on_losing_life(
    trigger: Trigger<LoseLife>,
    mut commands: Commands,
    mut life_query: Query<&mut Life>,
    player_query: Query<Entity, With<Player>>,
) {
    let mut life = life_query.get_mut(trigger.entity()).unwrap();
    let life_lost = trigger.event().life_lost;
    life.0 = life.0.saturating_sub(life_lost);

    if life.0 == 0 && player_query.contains(trigger.entity()) {
        commands.trigger(PlayerDeath);
    }
}

fn on_player_death(_trigger: Trigger<PlayerDeath>, mut app_exit: EventWriter<AppExit>) {
    println!("You died. Game over!");
    app_exit.send_default();
}
```

</details>

In the future, we intend to use hooks and observers to [replace `RemovedComponents`], [make our hierarchy management more robust], create a first-party replacement for [`bevy_eventlistener`] as part of our UI work, and [build out relations].
These are powerful, general-purpose tools: we can't wait to see the mad science the community cooks up with them!

When you're ready to get started, check out the [`component hooks`] and [`observers`] examples for more API details.

[`Event`]: https://docs.rs/bevy/0.14/bevy/ecs/event/trait.Event.html
[`Added`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Added.html
[`Changed`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Changed.html
[`RemovedComponents`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.RemovedComponents.html
["pull"-style mechanism]: https://dev.to/anubhavitis/push-vs-pull-api-architecture-1djo
[flecs]: https://www.flecs.dev/flecs/
[replace `RemovedComponents`]: https://github.com/bevyengine/bevy/issues/13928
[make our hierarchy management more robust]: https://github.com/bevyengine/bevy/issues/12235
[`bevy_eventlistener`]: https://github.com/aevyrie/bevy_eventlistener
[build out relations]: https://github.com/bevyengine/rfcs/pull/79
[`component hooks`]: https://github.com/bevyengine/bevy/tree/v0.14.0/examples/ecs/component_hooks.rs
[`observers`]: https://github.com/bevyengine/bevy/tree/v0.14.0/examples/ecs/observers.rs
[`Component`]: https://docs.rs/bevy/0.14/bevy/ecs/component/trait.Component.html
[`Command`]: https://docs.rs/bevy/0.14/bevy/ecs/world/trait.Command.html
[`Query`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Query.html
