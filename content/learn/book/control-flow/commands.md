+++
title = "Commands"
insert_anchor_links = "right"
[extra]
weight = 5
status = 'hidden'
+++

When we want to make structural changes to our application, we require mutable access to the `World`. Since only one system can mutably access the `World` at a time, we need a way to organize that access. **Commands** allow us to do just that. Each [`Command`] represents an instruction for manipulations to be performed on the world, and when we call a `Command` it gets placed in the **Command Queue**. All [`Commands`] in this queue are then ran at a specific time when we're safely able to mutably access the `World`.

Many operations in the ECS can *only* be done via exclusive world access, such as:

- Spawning and Despawning Entities

```rust
// Spawn a new Entity with a `bundle` of Components.
commands.spawn(bundle);

// Despawn the `entity` Entity.
commands.entity(entity).despawn(); 
```

- Inserting and Removing Resources

```rust
// Add `resource` to the World.
commands.init_resource::<resource>();

// Remove `resource` from the World.
commands.remove_resource::<resource>(); 
```

- Running One-Shot Systems and Schedules

```rust
// Run a System matching the ID in `system_id`.
commands.run_system(system_id); 

// Run all Systems that are in the `schedule` Schedule.
commands.run_schedule(schedule);
```

- Triggering Observers

```rust
// Trigger the `event` Event for all Observers watching for events of a matching type.
commands.trigger(event); 
```

To use `Commands` in your systems, it is easiest to pass it in as a system parameter. This ensures that every system that needs access to `Commands` gets it and that all `Commands` that are called are placed into the `Command` queue.

```rust
fn my_system(mut commands: Commands) {
    // Add a new Resource to track scores with implicit values.
    commands.insert_resource(Scoreboard{
        current_score: 0,
        high_score: 0,
    });
}
```

While Bevy offers a number of different pre-defined `Commands` to use, it also offers the [`queue`] method to make changes that aren't provided by the default `Commands`. `queue` gives us mutable access to the `World`, which we can use however we want. The most straightforward approach is to construct our modifications within the method itself (like we do below), however we can go one step further and create custom commands by implementing the `Command` trait on a custom struct, or even extend the `Commands` struct with custom traits and methods. We'll go more into this aspect further down the page in [Custom Commands].

```rust
// A custom Resource
#[derive(Resource, Default)]
struct Counter(u64);

fn add_twenty_five_to_counter_system(mut commands: Commands) {
    // A custom Command that accesses the Counter resource and adds a number to it.
    commands.queue(|world: &mut World| {
        let mut counter = world.get_resource_or_insert_with(Counter::default);
        counter.0 += 25;
    });
}
```

[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Commands.html
[`Command`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Command.html
[`queue`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Commands.html#method.queue
[Custom Commands]: /learn/book/control-flow/commands#custom-commands

## When Do Commands Take Effect?

`Commands` allow users to queue up changes as part of systems, deferring the work until later to avoid disrupting both system parallelism and data-oriented access of `Components`. Specifically, by default `Commands` are applied whenever a [`Schedule`] is completed. Ordinarily, this will occur multiple times during and after each frame. As a result, systems will always see the effects of `Commands` queued by systems in other schedules.

```rust
// An Event we want to trigger.
#[derive(Event)]
struct UpdateEvent;

// Add our Systems to their Schedules.
app.add_systems(Startup, insert_observer_system);
app.add_system(Update, update_system);

// This System will run in the `Startup` schedule.
fn insert_observer_system(mut commands: Commands) {
    commands.add_observer(|update: On<UpdateEvent>| {
        println!("This runs every Update!");
    });
}

// Meanwhile this System runs in the `Update` schedule.
fn update_system(mut commands: Commands) {
    commands.trigger(UpdateEvent);
}
```

In the above example, we have two systems: `insert_observer_system` running in the `Startup` schedule and `update_system` running in the `Update` schedule. Since the `Startup` schedule only runs once, the `add_observer()` command is only ran when our applications is launched. However the `trigger()` command is run everytime the `Update` schedule finishes, meaning that a `trigger()` command is sent to the queue and ran every time `Update` completes.

We mentioned above that by default `Commands` are applied at the *end* of a `Schedule`. While correct, what is really happening is that all of the `Commands` we queue are placed into a special `System` known as [`ApplyDeferred`]. This system will run at the end of every `Schedule` that has a system which uses `Commands` as a `SystemParameter`. Since `ApplyDeferred` is ran after all of systems in a given schedule, all of the changes made by the `ApplyDeferred` system are able to be seen by the systems in other schedules.

In addition, if a system accessing `Commands` is ordered before another system also accessing `Commands` in the same `Schedule`, that system will always see the effects of the `Commands` in the first system. Bevy ensures this occurs by dynamically inserting synchronization points, during which all `Commands` are applied. Each system can hold their own copy of `Commands` in their local system state. When `Commands` are applied, these queues are evaluated as in the same order that the systems were run. Within each system, the `Commands` are applied in a first-in-first-out order.

```rust
// Add our Systems in the same Schedule, but placed in a specific order.
app.add_systems(
    Update, (
        add_the_component, 
        remove_the_component.after(add_the_component)
        // `after` specifies that the `remove_the_component` System 
        // will only run after `add_the_component` has completed.
    )
);

// This System will add the TargetComponent to the `player` Entity.
fn add_the_component(mut commands: Commands, mut player: Single<Entity, Without<TargetComponent>) {
    commands.entity(player).insert_if_new((TargetComponent));
    println!("TargetComponent added!");
}

// This System will remove the TargetComponent from the `player` Entity.
fn remove_the_component(mut commands: Commands, mut player: Single<Entity, With<TargetComponent>) {
    commands.entity(player).remove((TargetComponent));
    println!("TargetComponent removed!");
}

```

[`Schedule`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Schedule.html
[`ApplyDeferred`]: https://docs.rs/bevy/0.18.0/bevy/ecs/prelude/struct.ApplyDeferred.html

## Entity Commands

Apart from making changes to the `World`, `Commands` are also used when making structural changes to `Entities`. When applying changes to a single entity, the [`Commands`] type is transformed into [`EntityCommands`] via [`Commands::entity`]. While the `Command` trait can have arbitrary effect on the `World`, the [`EntityCommand`] trait is designed to modify a single entity.

```rust
// Marker Component for a Player.
#[derive(Component)]
struct Player;

// Health Component for a Player.
#[derive(Component)]
struct Health(pub usize);

// Create a new Entity with the Player Component and save the Entity to the variable `player`.
let player = commands.spawn((Player));

// This accesses the EntityCommands for the Entity in `player`.
let player_commands = commands.entity(player);

// This adds a Health Component to the `player` Entity if it doesn't already exist.
player_commands.insert_if_new((Health(10)));
```

Like `Commands`, `EntityCommands` aren't run immediately. This means that it is possible for the `Entity` you want to modify to have despawned before the `EntityCommand` can be ran. All `EntityCommands` will check whether the `Entity` exists when the `EntityCommand` is ran and will return an error if it doesn't.

[`EntityCommands`]: https://docs.rs/bevy/latest/bevy/prelude/struct.EntityCommands.html
[`Commands::entity`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Commands.html#method.entity
[`EntityCommand`]: https://docs.rs/bevy/latest/bevy/prelude/trait.EntityCommand.html

## Parallel Commands

We stated above that `World` can only be mutably accessed by one system at a time, and while that is still true, that doesn't mean that we can't access and make *multiple* changes at the same time. As long as the changes we want to make are reliant on each other, we can use [`ParallelCommands`] to achieve this. A great example of this can be seen when dealing with `Queries`. Specifically, we can use [`Query::par_iter_mut`] along with `ParallelCommands` and it's method [`command_scope`] to perform structural changes to each `Entity` in a given `Query`.

```rust
// A marker Component that an Entity is going very fast.
#[derive(Component)]
struct SuperSpeed;

// This System runs across multiple threads, and ensures that each Query entry is only iterated over once.
fn parallel_command_system(
    mut query: Query<(Entity, &Velocity)>,
    par_commands: ParallelCommands
) {
    query.par_iter_mut().for_each(|(entity, velocity)| {
        if velocity.magnitude() > 10.0 {
            par_commands.command_scope(|mut commands| {
                // command_scope gives us access to a unique Commands instance
                // for each parallel operation. 
                commands.entity(entity).insert_if_new((SuperSpeed));
            });
        }
    });
}
```

[`ParallelCommands`]: https://docs.rs/bevy/latest/bevy/prelude/struct.ParallelCommands.html
[`Query::par_iter_mut`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Query.html#method.par_iter_mut
[`command_scope`]: https://docs.rs/bevy/latest/bevy/prelude/struct.ParallelCommands.html#method.command_scope

## Custom Commands

By now you should be able to see that `Commands` are just a series of queued changes that affect some part of the `World`. Bevy provides many `Commands` methods that you can easily take advantage of, however it is also possible (and can even be simple!) to build your own **Custom Commands**. Because of their flexible nature, custom commands are a powerful tool for implementing game-specific operations. While they may not be as fast or transparent as working with `Events` or `Observers`, the arbitrary flexibility can be great for quickly evolving game logic and performing operations atomically.

Writing custom commands is quite simple: create a struct and implement `Command` for it. If you want to pass in data, add fields to your struct. To send a custom command, simply call `commands.queue(CustomCommandStruct { my_data })`.

```rust
// A custom Resource
#[derive(Resource, Default)]
struct Counter(u64);

// Our custom Command
struct AddToCounter(u64);

// Implement the Command trait onto AddToCounter.
impl Command for AddToCounter {
    // `apply()` is a required method when implementing the Command trait. 
    fn apply(self, world: &mut World) {
        let mut counter = world.get_resource_or_insert_with(Counter::default);
        counter.0 += self.0;
    }
}

fn some_system(mut commands: Commands) {
    // Use our custom Command to add a number to the Counter Resource.
    commands.queue(AddToCounter(42));
}
```

At the top of the page we used the `queue` method to perform some changes on the `World`. In the above example we took that same premise and extended it into a full custom command. Creating custom commands can help reduce the amount of boilerplate code you write, especially if you know that you need to repeat that code at multiple points.

You can make this pattern even more ergonomic by writing an extension trait for the `Commands` type, allowing you to call new methods as long as the extension trait is imported. Calling `commands.custom_command(my_data)` is shorter and plays nicer with auto-complete, however this approach has no functional benefit or cost; it's simply a matter of style. These same strategies can also be applied for the `EntityCommand` trait and the `EntityCommands` struct.

```rust
// A custom Resource
#[derive(Resource, Default)]
struct Counter(u64);

// A custom Trait to add a value to the Counter Resource.
pub trait CounterAdd {
    fn add_to_counter(&mut self, value: u64)
}

// Implement our custom Trait on Commands.
impl CounterAdd for Commands {
    fn add_to_counter(&mut self, value: u64) {
        self.queue(|world: &mut World| {
            let resource = world.get_resource_mut::<Counter>().unwrap();
            resource.0 += value;
        })
    }
}

fn some_system(mut commands: Commands) {
    // Use the implementation of ResourceAdd to add a number to the Counter Resource.
    commands.add_to_counter(42);
}
```
