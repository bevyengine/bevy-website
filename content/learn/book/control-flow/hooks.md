+++
title = "Hooks"
insert_anchor_links = "right"
[extra]
weight = 11
status = 'hidden'
+++

<!-- TBW -->

In the previous chapter we learned about `Events` and how they allow us to run code in the `World` or on a specific `Entity` in response to a trigger condition. We can extend this concept by using *lifecycle events* to run code whenever we alter the `Components` that make up an `Entity`. These can be accessed through [`ComponentHooks`], or we can also utilize them with regular `Events` and `Observers`.

[`ComponentHooks`]: https://docs.rs/bevy/latest/bevy/ecs/lifecycle/struct.ComponentHooks.html

## Lifecycle Events

**Lifecycle Events** are `Events` that occur when a `Component` is *added to*, *inserted in*, *replaced on*, or *removed from* an `Entity`, or when a `Component` is *despawned*. Specifically these are all `EntityEvents`, meaning that they will carry an `entity` field which holds the `EntityID` of the `Entity` being targeted.

We can split these five lifecycle events into two categories: lifecycle events that trigger when a `Component` is *added* to an `Entity`, and lifecycle events that trigger when a `Component` is *removed* from an `Entity`.

Adding a `Component`:

- [`Add`] triggers when a component is added to an `Entity` *that did not already have it*.
- [`Insert`] triggers when a component is added to an `Entity`, *regardless of whether it already had it or not*.

When both `Add` and `Insert` occur, `Add` hooks are evaluated before `Insert` hooks.

```rust
#[derive(Component)]
pub struct MyComponent;

// Using Observers:
world.add_observer(|add: On<Add, MyComponent>| {
    println!("MyComponent added to {}", add.entity);
});
world.add_observer(|insert: On<Insert, MyComponent>| {
    println!("MyComponent inserted into {}", insert.entity);
});

// Using a ComponentHooks method:
world.register_component_hooks::<MyComponent>().on_add(|add| {
    println!("MyComponent added to {}", add.entity);
});
world.register_component_hooks::<MyComponent>().on_insert(|insert| {
    println!("MyComponent inserted into {}", insert.entity);
});
```

Removing a `Component`:

- [`Replace`] triggers when a component is removed from an `Entity`, *regardless of if it is replaced with a new value*.
- [`Remove`] triggers when a component is removed from an `Entity` *and not replaced*. (This also happens before the component is actually removed.)
- [`Despawn`] triggered on *each* component on an `Entity` when the `Entity` is *despawned*.

`Replace` hooks are evaluated before `Remove` hooks, and `Despawn` hooks are evaluated last.

```rust
#[derive(Component)]
pub struct MyComponent;

// Using Observers:
world.add_observer(|replace: On<Replace, MyComponent>| {
    println!("MyComponent was replaced on {}", replace.entity);
});
world.add_observer(|remove: On<Remove, MyComponent>| {
    println!("MyComponent was removed from {}", remove.entity);
});
world.add_observer(|despawn: On<Despawn, MyComponent>| {
    println!("MyComponent was despawned with {}", despawn.entity);
})

// Using a ComponentHooks method:
world.register_component_hooks::<MyComponent>().on_replace(|replace| {
    println!("MyComponent was replaced on {}", replace.entity);
});
world.register_component_hooks::<MyComponent>().on_remove(|remove| {
    println!("MyComponent was removed from {}", remove.entity);
});
world.register_component_hooks::<MyComponent>().on_despawn(|despawn| {
    println!("MyComponent was despawned with {}", despawn.entity);
});
```

[`Add`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Add.html
[`Insert`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Insert.html
[`Replace`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Replace.html
[`Remove`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Remove.html
[`Despawn`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Despawn.html

## ComponentHooks Vs Observers

While it is possible to interact with lifecycle events through `Observers`, it might not always be the smoothest way. Since lifecycle events exclusively deal with `Components`, it can be more intuitive to use [`ComponentHooks`] instead.

Consider the following use case. We want to spawn an `Entity` with a `PlayerName` component and print the value inside `PlayerName` to the console upon creation. Using `Observers`, we'd have to structure our code like such:

```rust
// First we create our Component.
#[derive(Component)]
struct PlayerName(pub String);

// Then we add an Observer that will watch for `PlayerName` being added.
commands.add_observer(|print_name: On<Add, PlayerName>| {
    println!("Spawned: {}", print_name.components[0].0);
});

commands.spawn((
    PlayerName("Player1".to_string()),
    Transform::from_xyz(x: 0.0, y: 0.0, z: 0.0),
    Visibility::Visible,
));
```

Note that we have to implicitly add the `Observer` before we could print out the value inside `PlayerName`. While this seems obvious in this context, it becomes easy to overlook if our `Observer` isn't added in the same `System` that our `PlayerName` component is spawned in. If our `Observer` wasn't added to the `World`, or was despawned for some reason, we wouldn't get the `PlayerName` value printed out like we want.

`ComponentHooks` can fix this issue by explicitly tying `PlayerName` to an `Add` lifecycle event (or any other lifecycle event). Instead of relying on an `Observer` to be present and to react to our lifecycle event, the `World` will react for us whenever it sees the `Component` we specify being modified in some way.

```rust
// First we create our Component.
#[derive(Component)]
// Note the lifecycle event we attach to a specific function in this attribute:
#[component(on_add = print_player_name)]
struct PlayerName(pub String);

// Whenever a `PlayerName` component is added to an Entity, this function will trigger.
fn print_player_name(player_name: HookContext) {
    println!("Spawned: {}", player_name.0);
}

// Now we can spawn in our Entity with the `PlayerName` without needing an Observer.
commands.spawn((
    PlayerName("Player1".to_string()),
    Transform::from_xyz(x: 0.0, y: 0.0, z: 0.0),
    Visibility::Visible,
));
```

Alternatively, we can use the [`World::register_component_hook`] method to accomplish the same as above without needing to define a whole separate function.

```rust
// Create our Component like normal.
#[derive(Component)]
struct PlayerName(pub String);

// Tell the `World` to run this code whenever a `PlayerName` component is added to an Entity.
world.register_component_hooks::<PlayerName>().on_add(|context| {
    println!("Spawned: {}", context.0);
});
```

[`World::register_component_hook`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html#method.register_component_hooks
