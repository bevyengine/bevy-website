+++
title = "Systems"
insert_anchor_links = "right"
[extra]
weight = 0
status = 'hidden'
+++

**Systems** are the primary way that Bevy applications interact with the [`World`].
When you need to perform regularly scheduled logic, a [`System`] will usually be the best tool for the job.
As the third pillar of the ECS model, systems give us the power to operate over a large number of similar entities in an efficient way.
When we create entities with similar components, those components are laid out beside each other in memory.
This gives us good [cache locality] when operating on them in a batched fashion.

A Bevy `System` is any Rust function where every argument implements the [`SystemParam`] trait.
Some common parameters you'll use and see in Bevy are [`Query`], [`Res`], [`ResMut`], [`MessageReader`], [`MessageWriter`], [`Local`] and [`Commands`].
Each of these is covered in their own chapters throughout this book, but we'll be using them here to help explain the concepts that directly tie in to systems.

```rust
// A simple system accessing a Commands parameter, a Query parameter,
// and a mutable Resource (ResMut) parameter.
fn register_players_system(
    // Our System's System Parameters.
    mut commands: Commands,
    player_query: Query<Entity, With<ActivePlayerMarker>>,
    mut player_resource: ResMut<PlayerListResource>
) {
    // Using our parameters in a System.
    for player_entity in player_query.iter() {
        // Insert a InGameMarker component into the Entity returned by the Query.
        commands.entity(player_entity).insert(InGameMarker);
        // Add the Entity to our Resource.
        player_resource.player_list.push(player_entity);
    }
}

// A simple Component to mark a newly added active Player.
#[derive(Component)]
struct ActivePlayerMarker;

// A simple Component to mark a Player as being in game.
#[derive(Component)]
struct InGameMarker;

// A simple Resource to track a list of players in the current game.
#[derive(Resource)]
struct PlayerListResource {
    pub player_list: Vec<Entity>,
}
```

If we have several parameters we want to access together repeatedly, we can be more ergonomic: We can create a custom system parameter by deriving the [`SystemParam`] trait on a custom struct.
As long as every field on our custom struct is a system parameter, we can access the custom struct as a single system parameter.

```rust
// Create a custom system parameter accessing a ResMut resource and a Query.
#[derive(SystemParam)]
pub struct PlayerSystemParameter{
    commands: Commands<'w, 's>,
    player_query: Query<'w, 's, Entity, With<ActivePlayerMarker>>,
    list_of_players: ResMut<'w, PlayerListResource>,
}

// Use our custom system parameter to replicate the same functionality
// as the system in the previous example.
fn register_players_system(mut player_parameter: PlayerSystemParameter) {
    for player_entity in player_parameter.player_query.iter() {
        player_parameter.commands.entity(player_entity).insert(InGameMarker);
        
        player_parameter.list_of_players.player_list.push(player_entity);
    }
}
```

There are some additional caveats and restrictions that come with the `SystemParam` trait, but those are beyond the scope of this section.
If you're interested, you can check the [`SystemParam`] page for more details.

[cache locality]: https://en.wikipedia.org/wiki/Locality_of_reference
[`World`]: https://docs.rs/bevy/latest/bevy/prelude/struct.World.html

[`System`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/trait.System.html
[`SystemParam`]: https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html
[`Query`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Query.html
[`Res`]: https://docs.rs/bevy/latest/bevy/ecs/change_detection/struct.Res.html
[`ResMut`]: https://docs.rs/bevy/latest/bevy/ecs/change_detection/struct.ResMut.html
[`MessageReader`]: https://docs.rs/bevy/latest/bevy/ecs/message/struct.MessageReader.html
[`MessageWriter`]: https://docs.rs/bevy/latest/bevy/ecs/message/struct.MessageWriter.html
[`Local`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Local.html
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.Commands.html

## Accessing Data In Systems

One of the major benefits of Bevy's system abstraction is that it easily and efficiently ["splits the borrow"] of a `World`.
When a system is run, the requested data is automatically fetched from the `World`.
Other systems are prevented from accessing the requested data if their access would violate the rules of the borrow checker.
The prime directive of Rust still applies: accessing `World` data can be mutable *or* shared, but never both at once.

We can see this in action when using multiple systems.
If two systems access non-overlapping parts of the world they can run at the same time.
In the example below, both systems would be able to run in parallel since they do not access the same data.

```rust
// This System accesses two Queries, one for the Player Entity, 
// and one for the Enemy Entity.
fn player_and_enemy_access(
    player_query: Single<&Transform, With<Player>>,
    enemy_query: Single<&Transform, With<Enemy>>
) {
    println!("Player Location: {:?}", player_query.translation);
    println!("Enemy Location: {:?}", enemy_query.translation);
}

// While this System accesses a Resource.
fn access_resource(custom_resource: Res<CustomResource>) {
    println!("Custom Resource Value: {:?}", custom_resource.value);
}
```

Systems can run in parallel, as long as one system isn't *mutably* accessing the same data as the other system.
Both systems in the example below access the same data, but neither changes the data.
These systems can run in parallel.

```rust
// This System immutably accesses the Transform component. 
fn print_player_and_enemy_locations(
    player_query: Single<&Transform, With<Player>>,
    enemy_query: Single<&Transform, With<Enemy>>
) {
    println!("Player Location: {:?}", player_query.translation);
    println!("Enemy Location: {:?}", enemy_query.translation);
}

// While this System also immutably accesses the same Transform components,
// meaning it can run alongside `print_player_and_enemy_locations`
// even though they both access the same data.
fn damage_player(
    mut player_query: Single<(&Transform, &mut Stats), With<Player>>,
    enemy_query: Single<&Transform, With<Enemy>>,
) {
    let (player_transform, player_stats) = player_query.deref_mut();

    let distance_to_player = enemy_query
        .translation
        .distance(player_transform.translation);

    if distance_to_player < DAMAGE_RADIUS {
        player_stats.health -= 1
    }
}
```

Even though we've been talking about multiple systems, single systems also have to abide by the borrow checker when accessing data.
As an example, `Component` data cannot be accessed both mutably and immutably without using some workarounds (which you can read about in the [Queries book section]). 
The same concept applies to resource access as well.

```rust
// This system would not compile because it accesses the 
// Transform component data both mutably and immutably.
fn player_and_enemy_access(
    player_query: Single<&Transform, With<Player>>,
    enemy_query: Single<&mut Transform, With<Enemy>>
) {
    ...
}

// This system would not compile because it accesses the same resource 
// data both mutably and immutably.
fn access_resource(
    custom_resource: Res<CustomResource>,
    mut mut_custom_resource: ResMut<CustomResource>
) {
    ...
}
```

In addition to the data retrieved from the `World`, each system also has its own cached state.
This system state is used for performance optimizations (e.g. for queries), but can also be used to keep track of private internal state or as scratch space.
See the section on [local system state] for more details.

["splits the borrow"]: https://doc.rust-lang.org/nomicon/borrow-splitting.html

[local system state]: /learn/book/storing-data/local-system-param

## Running Systems In Schedules

Systems are usually repeatedly run throughout the life of your application.
We are able to control when systems run and how systems are ordered by placing them into a [`Schedule`].
The [`App::add_system`] method is the simplest way to do this, allowing us to specify a `System` that we want to insert into a specific `Schedule` running in our application.

```rust
fn main() {
    App::new()
        .add_systems(Update, my_system);
}
```

In the above example, we're running the `my_system` system in the [`Update`] schedule.
`Update` is one of the *standard* schedules that Bevy provides, but there are many more to choose from.
Each standard schedule provides access to a different point of time for a single frame, allowing you to place your systems in relation to the state of the frame.
You can read more about schedules in the dedicated [Schedules section], but for now you just need to know a couple things:

- By default, systems within a single schedule run in parallel.
- Systems within a single schedule can also be explicitly ordered to run relative to each other.
- Each of the standard schedules are evaluated *once per frame*.
- There are also [`Fixed`] schedules which run at a consistent interval.

[Schedules section]: /learn/book/the-game-loop/schedules

[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`App::add_system`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.add_systems
[`Update`]: https://docs.rs/bevy/latest/bevy/app/struct.Update.html
[`Fixed`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Fixed.html

## One-shot Systems

Systems can also be run on demand, via a "one-shot" pattern.
This is an extremely flexible tool, allowing you to execute arbitrary logic on the world in an ergonomic way whenever you please.
One-shot systems are particularly useful for testing, handling callbacks in UI, or creating scripted events.

We have two means of running one-shot systems:

- Register and invoke a system's [`SystemId`].
- Access a System by name, caching it in a [`CachedSystemId`].

Both means involve accessing the *state* of a particular system.
When working with one-shot systems, entities are spawned to store this information.
The [`Entity`] identifier for each system are stored in a `SystemId` (or `CachedSystemId`).

If we want to handle the `SystemId` ourselves, we have to manually register it in our application by using [`World::register_system`].
After the system is registered, we can use [`World::run_system`] and pass in the `SystemId` value to run the system at a given point.

```rust
// This System will register and then run two One-shot Systems.
fn register_one_shot_systems(mut world: &mut World) {
    // Register two One-shot Systems.
    let some_system_id = world.register_system(one_shot_system);
    let another_system_id = world.register_system(one_shot_system_with_input);
    
    // Run the first One-shot System. 
    world.run_system(some_system_id);
    // Run the second One-shot System.
    world.run_system_with(another_system_id, 5);
}

// System that runs on command.
fn one_shot_system() {
    println!("This is a one-shot system that is run on demand.");
}

// System that runs on command and takes a usize input.
fn one_shot_system_with_input(In(input): In<usize>) {
    println!("This is a one-shot system run on demand with input: {}", input);
}
```

Alternatively, if we don't want to handle the `SystemId` ourselves, we can use [`World::run_system_cached`].
This will automatically cache the system and will retrieve its `SystemId` based on its [`TypeId`].
However this approach can be harder to abstract, and limits you to one copy of each system.
Any internal state (such as [locals] or [change detection] information) will be shared.

```rust
fn run_one_shot_systems(mut world: &mut World) {
    // This will print a `local_value: 1` since this is the first time
    // the system is run.
    world.run_system_cached(one_shot_system);
    // This will print a `local_value: 2` since this is the second time 
    // the system is run, and `local_value` is cached from the first run 
    // of the system.
    world.run_system_cached(one_shot_system);
}

fn one_shot_system(mut local_value: Local<usize>) {
    local_value += 1;
    println!("local_value: {}", local_value);
}
```

Be aware that it's possible for these systems to *not* run successfully.
`World::run_system`, `World::run_system_with`, and `World::run_system_cached` (along with other methods for running systems) return a `Result`, although if you aren't receiving any values you might not notice any failures.
For example, if you attempt to run a system by id that hasn't been registered, you'll receive a `Result` carrying a [`RegisteredSystemError`] enum with a `SystemIdNotRegistered` variant.

```rust
// Use a SystemId for a system that hasn't been registered.
let result = world.run_system(999);
// Attempting to use the Result of the above will result 
// in a RegisteredSystemError.
assert_eq!(result.unwrap_err(), RegisteredSystemError::SystemIdNotRegistered(999))
```

For convenience, [`Commands`] has a set of equivalent methods, allowing you to queue up systems from within other systems.
This can recurse; use this power wisely!

```rust
// This is the same as World::register_system, but is queued to run 
// instead of executing immediately.
commands.register_system(some_system);

// This is the same as World::run_system, but is queued to run 
// instead of executing immediately.
commands.run_system(SystemId);

// This is the same as World::run_system_with, but is queued to
// run instead of executing immediately.
commands.run_system_with(SystemId, SystemInput);
```

Note that an entire [`Schedule`] can be run on demand in much the same way,
which can be valuable when trying to evaluate complex blocks of logic in response to specific triggers,
or at a rate other than "once per frame".

```rust
// A custom Schedule that we would run Systems in. 
#[derive(ScheduleLabel, Hash, Debug, PartialEq, Eq, Clone, Copy)]
struct FooSchedule;

// Queue all of the Systems within FooSchedule to run.
commands.run_schedule(FooSchedule);
```

[locals]: /learn/book/storing-data/local-system-param
[change detection]: /learn/book/control-flow/change-detection

[`Entity`]: https://docs.rs/bevy/latest/bevy/ecs/entity/struct.Entity.html
[`SystemId`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.SystemId.html
[`CachedSystemId`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.CachedSystemId.html
[`World::register_system`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.register_system
[`World::run_system`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.run_system
[`World::run_system_cached`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.run_system_cached
[`TypeId`]: https://doc.rust-lang.org/nightly/core/any/struct.TypeId.html
[`RegisteredSystemError`]: https://docs.rs/bevy/latest/bevy/ecs/system/enum.RegisteredSystemError.html

## Exclusive Systems

So far we've used different system parameters to safely access non-overlapping data within our `World`, but how can we safely access the `World` itself?
In order to safely mutate our `World`, we have to use **exclusive systems**.

Exclusive Systems:

- *Can* access any data in the world.
- *Cannot* be run in parallel with any other system.
- *Do not* use [`Commands`], including to spawn or despawn entities or to insert or remove components.
- Are created by using `&mut World` as the first system parameter.

```rust
// This system exclusively accesses `World` in a mutable way.
fn exclusive_system(mut world: &mut World) {
    // This registers a System.
    world.register_system(SystemdId);
    // This removes all entities from the World.
    world.clear_entities();
    // This adds a custom schedule to be run in the World.
    world.add_schedule(MySchedule);
}
```

Exclusive systems are useful for operations that require making large or unique changes to your `World`.
Spawning large numbers of entities at once is one example of this, as there is no additional overhead incurred from *queueing* these changes like there is when using `Commands`.
They are also extremely useful for unusually complex game logic or control flow and can be used to immediately run schedules and one-shot systems (including other exclusive systems).

That said, exclusive systems are harder to schedule, both because they prevent any other system from running at the same time and because their scheduling order is ambiguous with any other systems in the same schedule. Remember that all systems in a specific `Schedule` are run in parallel by default unless explicitly ordered.

## System Input and Output

Systems can optionally take input, and produce output, via the [`In`] and [`Out`] generics on the various system traits
such as [`System`].
The application of this is simplest to understand in the context of one-shot systems.

```rust
fn call_system(mut world: &mut World) {
    // Call the one-shot system with an input value of 42.
    let system_value: Result<usize, _> = world.run_system_once_with(one_shot_system_with_input, 42);
    // After going through the one-shot system, system_value will equal `Ok<44>`.
    println!("System value: {}", system_value);
}

fn one_shot_system_with_input(In(input): In<usize>) -> usize {
    // `input` is passed into the system as an `In` usize value.
    // Because this function returns a usize value, our `Out` value 
    // is inferred to be a usize.
    input + 2
}
```

We can call [`World::run_system_once_with`] like we would normally, except we pass in an input value and receive an output value wrapped in a `Result`.
Remember, one-shot systems can potentially fail if the conditions of the system parameters weren't met.
The type of the output is always inferred, but when using system input the first parameter must be [`In<T>`], where `T` is any type of input that you want to pass in.

### System Piping

System inputs and outputs can also be used in scheduled systems, via **system piping**.
System piping uses the [`IntoSystem::pipe`] method to take the output of one system and feed it into a second system.

```rust
fn main() {
    App::new()
        // Pipe the value that first_system returns into second_system.
        .add_systems(Update, first_system.pipe(second_system))
        .run();
}

// This system reads from a Resource and returns a usize value.
fn first_system(res: Res<Resource>) -> usize {
    res.value.clone()
}

// This system takes in a usize value.
fn second_system(In(input): In<usize>) {
    input * 2;
    println!("{}", input);
}
```

Using the `IntoSystem::pipe` method requires that the second system have an `In<T>` type as it's first parameter.
Both of the systems are treated as a single system from the perspective of the scheduler.
This can be repeated indefinitely, but branching is not supported.

System piping is mostly useful for composing fragments of logic in a modular, reusable way.
System output is also used when returning errors from systems, as explained in the [Handling Errors] section of this chapter.

[Handling Errors]: /learn/book/control-flow/handling-errors

[`In`]: https://docs.rs/bevy/latest/bevy/prelude/trait.System.html#associatedtype.In
[`Out`]: https://docs.rs/bevy/latest/bevy/prelude/trait.System.html#associatedtype.Out
[`World::run_system_once_with`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/struct.World.html#method.run_system_once_with
[`In<T>`]: https://docs.rs/bevy/latest/bevy/prelude/struct.In.html
[`IntoSystem::pipe`]: https://docs.rs/bevy/latest/bevy/prelude/trait.IntoSystem.html#method.pipe
