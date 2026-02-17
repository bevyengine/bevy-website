+++
title = "Component Lifecycles"
insert_anchor_links = "right"
[extra]
weight = 11
status = 'hidden'
+++

<!-- TBW -->

In the previous chapter we learned about `Events` and how they allow us to run code in the `World` or on a specific `Entity` in response to a trigger condition. We can extend this concept by using **Lifecycle Events** to run code in response to altering a `Component` within an `Entity`. Lifecycle events are still `Events`, but specifically they are `EntityEvents` meaning that they will have an `entity` field which holds the `Entity` being targeted.

Within Bevy we currently have access to five distinct lifecycle events: `Add`, `Insert`, `Replace`, `Remove`, and `Despawn`. We can split these into two categories: lifecycle events that trigger when a `Component` is *added* to an `Entity`, and lifecycle events that trigger when a `Component` is *removed* from an `Entity`.

On adding a `Component`:

- [`Add`] triggers when a component is added to an `Entity` *that did not already have it*.
- [`Insert`] triggers when a component is added to an `Entity`, *regardless of whether it already had it or not*.

On removing/altering a `Component`:

- [`Replace`] triggers when a component is removed from an `Entity`, *regardless of if it is replaced with a new value*.
- [`Remove`] triggers when a component is removed from an `Entity` *and not replaced*. (This also happens before the component is actually removed.)
- [`Despawn`] triggered on *each* component on an `Entity` when the `Entity` is *despawned*.

It's also important to know that lifecycle events have an order in which they are evaluated. When both `Add` and `Insert` occur, `Add` hooks are evaluated before `Insert` hooks. `Replace` hooks are evaluated before `Remove` hooks, and `Despawn` hooks are evaluated last.

[`Add`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Add.html
[`Insert`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Insert.html
[`Replace`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Replace.html
[`Remove`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Remove.html
[`Despawn`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Despawn.html

## Component Hooks

The most common way of interacting with lifecycle events is by using [`ComponentHooks`]. We have a couple ways to use `ComponentHooks`, the first of which is through the [`World::register_component_hook`] method.

```rust
// Create a ComponentHook with the `World` method:
#[derive(Component)]
pub struct MyComponent;

// This method adds the ComponentHook to `MyComponent`, and will
// run it's code whenever `MyComponent` is added to an Entity.
world.register_component_hooks::<MyComponent>().on_add(|add| {
    println!("MyComponent added to {}", add.entity);
});
```

This method allows us to enable the `ComponentHook` within a system at any point. It's especially helpful if we only need to add a `ComponentHook` to a `Component` after a certain point in time or under certain conditions. However, if we know that any time a `Component` is modified we want to run a `ComponentHook`, then we can do so with the `component` attribute:

```rust
// Create a ComponentHook with an attribute:
#[derive(Component)]
#[component(on_add = hook_function)] // Attribute that enables the ComponentHook.
pub struct MyComponent;

// Function that will run when `MyComponent` is added to an Entity.
fn hook_function(component_hook: HookContext) {
    println!("MyComponent added to {}", component_hook.entity);
}
```

By deriving the `component` attribute on `MyComponent` and pointing to the function that should be run when `MyComponent` is modified, we can ensure that our lifecycle event is ran every time `MyComponent` is added. Using an attribute can also be further extended through closures if we want to avoid repeating similar code:

```rust
// Multiple ComponentHooks with a closure:
#[derive(Component)]
#[component(on_add = hook_closure("add"))]
#[component(on_remove = hook_closure("remove"))]
pub struct MyComponent;

// One function is provided for both `on_add` and `on_remove` to use with the value they both provide.
fn hook_closure(action: &'static str) -> impl Fn(HookContext) {
    move |ctx| {
        // Prints out the string slice passed in to the closure along 
        // with the Entity that the ComponentHook is triggered on.
        println!("{action} on {}", ctx.entity);
    }
}
```

Finally, we can also elide the `ComponentHook` function path if it matches the lifecycle event:

```rust
// Elide the `on_remove` lifecycle event for `MyComponent`.
#[derive(Component)]
#[component(on_remove)] // Note there is no function path given.
pub struct MyComponent;

impl MyComponent {
    // Instead of a separate function, we implement an `on_remove` method to run our 
    // code in response to `MyComponent` being removed.
    fn on_remove(context: HookContext) {
        println!("MyComponent was removed from {}", context.entity);
    }
}
```

[`ComponentHooks`]: https://docs.rs/bevy/latest/bevy/ecs/lifecycle/struct.ComponentHooks.html
[`World::register_component_hook`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html#method.register_component_hooks

## Lifecycle Observers

We can also use regular `Observers` to react to lifecycle events. Like we mentioned above, lifecycle events are `EntityEvents` which means they will carry an `entity` field. However, we don't have to specify an `event_target` field like we do with regular `EntityEvents`. Instead, by including the target `Component` inside of the `Trigger` we are telling the `Observer` to run whenever our target `Component` is altered on any `Entity`. When this happens, the `event_target` of the `EntityEvent` is filled by the `Entity` being altered with the target `Component`.

```rust
#[derive(Component)]
pub struct MyComponent;

// Using Observers:

// This observer will trigger whenever `MyComponent` is added to any Entity. 
world.add_observer(|add: On<Add, MyComponent>| {
    println!("MyComponent added to {}", add.entity);
});
```

### Observers Vs ComponentHooks

While it is possible to interact with lifecycle events through `Observers`, it might not always be the smoothest way. Since lifecycle events exclusively deal with `Components`, it is usually more intuitive to use `ComponentHooks` instead.

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

Note that we have to explicitly add the `Observer` before we could print out the value inside `PlayerName`. While this seems obvious in this context, it becomes easy to overlook if our `Observer` isn't added in the same `System` that our `PlayerName` component is spawned in. If our `Observer` wasn't added to the `World`, or was despawned for some reason, we wouldn't get the `PlayerName` value printed out like we want.

`ComponentHooks` can fix this issue by tying `PlayerName` to an `Add` lifecycle event (or any other lifecycle event). Instead of relying on an `Observer` to be present and to react to our lifecycle event, the `World` will react for us whenever it sees the `Component` we specify being modified in some way.

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

## Other Lifecycle Interactions

In earlier chapters, we went over several lifecycle event interactions without specifically naming them as lifecycle events. This is because they aren't used in the same manner as we've been using them so far. However they are still lifecycle event interactions, so we will briefly return to them to show the differences between them and the tools introduced in this chapter.

### Removed Components Parameter

The [`RemovedComponents`] system parameter can be used to access a list of `Entities` that had a specific `Component` removed. Notably this includes `Entities` that were also *despawned* with the target `Component`, meaning that any `Entity` included in `RemovedComponents` might not exist in the `World` when accessed.

The biggest difference when using `RemovedComponents` is that you cannot access or see any of the data that was removed. Instead, this system parameter acts like a [`MessageReader`], displaying which `Entities` were despawned with or had the target `Component` removed.

```rust
// This system can print out each Entity in `removed`, but we can't access the 
// data within `MyComponent` because it's already been removed.
fn react_on_removal(mut removed: RemovedComponents<MyComponent>) {
    removed.read().for_each(|removed_entity| println!("{}", removed_entity));
}

// Meanwhile, this ComponentHook can see both the Entity and the data within 
// `MyComponent` (if it exists). This is because the `on_remove` ComponentHook 
// runs *before* `MyComponent` is actually removed from the `Entity`. 
world.register_component_hooks::<MyComponent>().on_remove(|mut world, remove| {
    // Access the value within `MyComponent` before it's removed.
    let value = world.get::<MyComponent>(remove.entity).unwrap()
    println!("MyComponent with value {} removed from {}", value, remove.entity);
});
```

By default, `RemovedComponents` is automatically cleared on every `World` update. We can also manually clear `RemovedComponents` by using the [`World::clear_trackers`] method.

[`RemovedComponents`]: https://docs.rs/bevy/latest/bevy/prelude/struct.RemovedComponents.html
[`MessageReader`]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageReader.html
[`World::clear_trackers`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html#method.clear_trackers

### Added Query Filter

The [`Added`] query filter can be used to see `Entities` that have had a new instance of a specific `Component` added to them for the first time. Much like `RemovedComponents` though, `Added` can only be accessed after the lifecycle event occurs. Additionally `Added` is less precise, returning `Entities` that had a target `Component` added for the first time and `Entities` that had a target `Component` reinserted, even if the component already existed. In effect, this combines both the `Add` and `Insert` lifecycle events, which can be an important distinction depending on the functionality you want to run.

To show the differences, lets imagine we are building a networked multiplayer game where we want to track the following:

- Create a new `Entity` for each player when a new match starts.
- Have a way for players to reconnect to a match with an existing `Entity` if they happen to leave for some reason.
- Keep a general log of everytime a Player connects or reconnects to the match.

We can use both the `Added` query filter and `ComponentHooks` to achieve each use case.

```rust
// Use the `on_add` and `on_insert` ComponentHooks to provide specific event handling.
#[derive(Component)]
#[component(on_add = player_new_connection)]
#[component(on_insert = player_reconnect)]
pub struct PlayerConnection{
    // Fields containing relevant information about the PlayerConnection
}

// This function will run when a PlayerConnection component is added to an Entity for the first time.
fn player_new_connection(new_player: HookContext) {
    // Some functionality that will run when a new Player connects to the match for the 
    // first time.
}

// This function will run when a PlayerConnection component is inserted into an Entity, 
// even if it already existed.
fn player_reconnect(reconnection: HookContext) {
    // Some functionality that will run when an existing Player reconnects to the match.
}

// This system will log whenever a PlayerConnection component is added to an Entity.
// In effect, this will log all connections to the match.
fn log_player_connections(query: Query<&Name, Added<PlayerConnection>>) {
    for player in &query {
        println!{"Player Connection Log: {} Joined The Match"};
    }
}
```

Obviously this is an extremely simplified example (and probably shouldn't be used in an actual multiplayer game), but you can see how `Added` and `ComponentHooks` can each be useful when applied in the right scenarios.

[`Added`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Added.html
