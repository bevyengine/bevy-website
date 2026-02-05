+++
title = "Events"
insert_anchor_links = "right"
[extra]
weight = 9
status = 'hidden'
+++

<!-- TBW -->

While Systems are great for running logic at scheduled updates, many features need to be reactive, not scheduled. Picking up an item, performing an attack, or even hovering the mouse over an object are all actions that wouldn't work very well if they *had* to run on every schedule update. Instead of a System, we can use **Events** to activate some logic or functionality at a specific time or in response to something occurring.

There are three required parts when using an Event: a **Trigger**, an **Observer**, and the `Event` itself.

- A **Trigger** is the condition that will determine when an `Event` will happen in the `World`.
- The Entities that will react to our `Event` are known as **Observers**. These Entities "observe" the `World` and will run some functionality in response to our `Event`.
- Finally, the `Event` is a Rust type that implements the `Event` trait.

A basic use of an `Event` would something like this:

1. Implement the [`Event`] trait on a type:

```rust
#[derive(Event)]
struct Speak {
    message: String,
}
```

2. Add an [`Observer`] to the World that will watch for our event:

```rust
// To add the observer immediately:
world.add_observer(|speak: On<Speak>| {
    println!("{}", speak.message);
});

// To add the observer once the Command Queue is ran:
commands.add_observer(|speak: On<Speak>| {
    println!("{}", speak.message);
});
```

3. Trigger the `Speak` event:

```rust
// To trigger the event immediately:
world.trigger(Speak {
    message: "Hello!".to_string(),
});

// To trigger the event once the Command Queue is ran:
commands.trigger(Speak {
    message: "Hello!".to_string(),
});
```

Since an [`Event`] type is just a regular Rust type, we can add fields which will hold data that can be used when triggering the event. In the above example, our `Speak` event type has a `message` field which holds a `String` value. When we added an `Observer` that watches for our `Speak` event being triggered, we specified that the observer will print out the value held in the `message` field to the console using the `println!` macro. As such, we could trigger multiple `Speak` events and pass in different `String` values.

```rust
// Triggers `Speak` immediately.
world.trigger(Speak {
    message: "This event runs immediately!".to_string(),
});

// Triggers `Speak` after being placed in the command queue.
commands.trigger(Speak {
    message: "This event runs in the queue!".to_string(),
});

// We can define a variable with its type as 
// `Speak` to make the code look cleaner:
let custom_message: Speak = Speak {
    message: "And this is defined before-hand to make everything look nicer!".to_string(),
};
commands.trigger(custom_message);
```

[`Event`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Event.html
[`Observer`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Observer.html

## Event Triggers

Every `Event` requires a [`Trigger`], represented by default with the `On<Event>` syntax. It's best to think of a `Trigger` as the call which activates the `Event`, hence why we use `On`: our code will run `On` our `<Event>`.

A [`Trigger`] can be called by accessing [`World`] (via [`World::trigger`]) to run the `Event` immediately, or by using [`Commands`] (via [`Commands::trigger`]) to add the `Event` to the Command Queue.

```rust
// An event that deals damage to the player.
#[derive(Event)]
struct PlayerDamage {
    amount: i32,
    taken_by: String,
}

fn setup(mut commands: Commands) {
    // Add an observer that watches for the player damage event, and subtracts the player's health by the damage amount.
    commands.add_observer(|damage_event: On<PlayerDamage>, player: Single<(Entity, &mut Health), With<Player>>| {
        // Print out the values passed in when PlayerDamage is triggered.
        println!("{} damage dealt to {}", damage_event.amount, damage_event.taken_by);
        // Dereference out of the `player` Single query.
        let (player_entity, mut player_health): (Entity, &mut Health) = *player;
        // Subtract the damage value from player_health.
        player_health -= player_damage.amount;
    });
}

fn player_damage(mut commands: Commands, player: Single<(Entity, Name), With<Player>>) {
    // Damage value.
    let damage_value: i32 = 4;
    // Get the player's name.
    let (player_entity, player_name): (Entity, Name) = *player;    
    // Trigger a PlayerDamage event and pass the event the required values.
    commands.trigger(PlayerDamage {
        amount = damage_value,
        taken_by = player_name.to_string(),
    });
}

```

A `Trigger` determines which observers will be activated. When we call `commands.add_observer()` in the above example, `damage_event: On<PlayerDamage>` is the `Trigger` which will cause the `Observer` to run its code. If there were other observers added that also had `On<PlayerDamage>` triggers, those observers would also run their code when we trigger a `PlayerDamage` event.

As we mentioned above, a `Trigger` can also be used to pass values from the `Event` to an `Observer`. When we triggered the `PlayerDamage` event in `fn player_damage()`, we gave it `damage_value` and `player_name` in the `amount` and `taken_by` fields. These values are then used by the `Observer` we created in `fn setup()` to both print to the console and to adjust the Player's Health we got from a Single query.

[`Trigger`]: https://docs.rs/bevy/latest/bevy/ecs/event/trait.Trigger.html
[`World`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html
[`World::trigger`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html#method.trigger
[`Commands`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Commands.html
[`Commands::trigger`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Commands.html#method.trigger

## Entity Events

Up until now, Events have been utilized in a *global* context, meaning that observers will run their code without taking any specific entities into account.

What if we do have some unique functionality that we want to run on certain entities? We can achieve this by modifying our `Event` types to implement [`EntityEvent`] rather than [`Event`]:

```rust
// Global Event
#[derive(Event)]
struct Speak {
    message: String,
}

// Entity Event
#[derive(EntityEvent)]
struct Explode {
    entity: Entity,
}
```

With [`EntityEvent`], we are selecting a single `Entity` that will respond to our event. To determine the selected `Entity`, we need to provide the `EventEntity` with an `entity_target` field. By default, `event_target` will be set to the value inside an `entity` field in a struct (if it exists). Otherwise we can manually specify the `event_target` by using the `#[event_target]` field attribute.

```rust
// A simple EntityEvent:
#[derive(EntityEvent)]
struct Explode {
    entity: Entity, // `event_target` is automatically applied to this field.
}

// An EntityEvent using a tuple:
#[derive(EntityEvent)]
struct Explode(Entity); // A tuple struct with only an Entity field will also set `event_target` automatically.

// An EntityEvent with multiple fields:
#[derive(EntityEvent)]
struct Explode {
    #[event_target] // Using the #[event_target] field attribute,
    target: Entity, // `event_target` is applied to this field.
    damage: i32,
    radius: f32,
}

// An EntityEvent using a tuple struct with multiple fields:
#[derive(EntityEvent)]
struct Explode(#[event_target] Entity, i32, f32); // Using #[event_target] inside a tuple struct with multiple fields.

```

Just as with a global [`Event`], [`EntityEvent`] also needs a [`Trigger`]. Thankfully, we trigger an `EntityEvent` the same way we trigger a regular [`Event`]:

```rust
world.trigger(Explode(some_entity, 4, 2.5));
// Trigger an Explode `EntityEvent`, passing in a tuple consisting of 
// an Entity (`some_entity`), an i32 (`4`), and a f32 (`2.5`).
```

However, [`EntityEvent`] does differ from [`Event`] when it comes to using an observer. To make an Entity observe an [`EntityEvent`], we have to select the `Entity` with [`World::entity_mut`] and then add the observer with [`EntityCommands::observe`]:

```rust
/// This observer will only run for Explode events triggered for `some_entity`
world.entity_mut(some_entity).observe(|explode: On<Explode>| {});
```

It is worth noting that an [`EntityEvent`] is still an [`Event`]; we can still set global observers to watch for an [`EntityEvent`]:

```rust
world.add_observer(|explode: On<Explode>| {}); // Global observer that will run when the Explode event is triggered.

world.entity_mut(some_entity).observe(|explode: On<Explode| {}); // Entity observer that will only run when the Explode event is triggered for `some_entity`.
```

[`EntityEvent`]: https://docs.rs/bevy/latest/bevy/prelude/trait.EntityEvent.html
[`EntityCommands::observe`]: https://docs.rs/bevy/latest/bevy/prelude/struct.EntityCommands.html#method.observe
[`World::entity_mut`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html#method.entity_mut

### EntityEvent Propagation

Since [`EntityEvent`] is triggered for individual entities themselves and not for all independent observer entities, we can leverage any [Relations] present on an `Entity` when an [`EntityEvent`] is activated. This is known as Propagation, or "Event Bubbling", and allows an [`EntityEvent`] to traverse a hierarchy chain while triggering on each `Entity` in that hierarchy chain.

Propagation has to be explicitly enabled on an [`EntityEvent`], and any `Observer` has to opt-in. This is done by setting the `#[entity_event(propagate)]` attribute on an [`EntityEvent`] and also by allowing the desired `Observer` to allow propagation:

```rust
// Set the `entity_event(propagate)` attribute.
#[derive(EntityEvent)]
#[entity_event(propagate)]
struct Click {
    entity: Entity,
}

// Let the observer event propagate.
world.add_observer(|mut click: On<Click>| {
    click.propagate(true);
});
```

Likewise, if we have an [`EntityEvent`] that an `Observer` is observing, we can explicitly disable propagation:

```rust
// Disable event propagation on an observer.
world.add_observer(|mut click: On<Click>| {
    click.propagate(false);
});
```

Or, if we don't want to explicitly enable propagation for every observer, we can also specify a `auto_propagate` attribute alongside `propagate`:

```rust
// Enable `auto_propagate` on an EntityEvent.
#[derive(EntityEvent)]
#[entity_event(propagate, auto_propagate)]
struct Click {
    entity: Entity,
}
```

By default, [`EntityEvent`] propagation will follow the [`ChildOf`] relation, starting at the original entity ("child") that the [`EntityEvent`] was triggered on and then repeatedly triggering on the `Entity` that is contained within the [`ChildOf`] component ("parent"). However, we can also specify a custom traversal implementation that will propagate along a custom relation.

```rust
// "ChildOf" equivalent. 
#[derive(Component)]
#[relationship(relationship_target = ClickableBy)]
struct Clickable(Entity);

// "Children" equivalent.
#[derive(Component)]
#[relationship_target(relationship = Clickable)]
struct ClickableBy(Vec<Entity>);

// `EntityEvent` Click will target the Clickable/ClickableBy relation when propagating.
#[derive(EntityEvent)]
#[entity_event(propagate = &'static Clickable)]
struct Click {
    entity: Entity,
}
```

To see [`EntityEvent`] propagation in action, the [Bevy Examples Page] has an [Observer Propagation] example which showcases an `Attack` [`EntityEvent`] propagating up from a piece of armor to the entity that is wearing the armor.

[Relations]: /learn/book/storing-data/relations
[Bevy Examples Page]: https://bevy.org/examples
[Observer Propagation]: https://bevy.org/examples/ecs-entity-component-system/observer-propagation/

[`ChildOf`]: https://docs.rs/bevy/latest/bevy/prelude/struct.ChildOf.html
