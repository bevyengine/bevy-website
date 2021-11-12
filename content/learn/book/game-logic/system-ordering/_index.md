+++
title = "Scheduling systems"
weight = 1
template = "book-section.html"
page_template = "book-section.html"
+++

As we peeked at in the introduction to this chapter, {{rust_type(type="trait" crate="bevy_ecs" mod="system" name="System" plural="true")}} are stored within {{rust_type(type="trait" crate="bevy_ecs" mod="schedule" name="Stage" plural = "true")}} which themselves live within a {{rust_type(type="struct" crate="bevy_ecs" mod="schedule" name="Schedule")}}.

At this point, you should be comfortable with the idea that systems are the basic unit of logic in Bevy, reading and modifying the game state.
All of the systems within your game (probably) belong to a single schedule, which is run each game loop.
Stages are used to coarsely organize systems with a schedule: all systems within the same {{rust_type(type="struct" crate="bevy_ecs" mod="schedule" name="SystemStage" method="parallel")}} can be run in parallel with each other (if their data accesses are compatible).

In particular, splitting systems between stages is important for command processing: as discussed in their chapter, commands can only be applied when we have exclusive access to the {{rust_type(type="struct" crate="bevy_ecs" name="World")}} due to their far-reaching effects.

If we have a {{rust_type(type="struct" crate="bevy_ecs" name="World")}}, we can run a {{rust_type(type="struct" crate="bevy_ecs" mod="schedule" name="Schedule")}} on it!

```rust
use bevy::ecs::schedule::{Schedule, StageLabel, SystemStage};
use bevy::ecs::system::{Res, ResMut};
use bevy::ecs::world::World;

fn main() {
    // Creating our world and initializing our demo resource
    let mut world = World::new();
    world.insert_resource(Count(0));

    // Creating our first stage
    let mut stage_1 = SystemStage::parallel();
    stage_1.add_system(report_count_system);

    let mut stage_2 = SystemStage::parallel();
    stage_2.add_system(count_up_system);

    // Notice that the same function can be inserted multiple times,
    // creating new and entirely distinct systems
    let mut stage_3 = SystemStage::parallel();
    stage_3.add_system(report_count_system);

    // Now, let's assemble these stages into a `Schedule`
    let mut schedule = Schedule::default();
    schedule
        // You can use analagous methods on `App` to modify your schedule
        .add_stage(DemoStages::One, stage_1)
        // Each stage needs its own unique label
        .add_stage_after(DemoStages::One, DemoStages::Two, stage_2)
        // Stages can be inserted before or after existing stages
        .add_stage_after(DemoStages::Two, DemoStages::Three, stage_3);

    // Schedules are run on worlds, applying their systems
    // This schedule will produce the following debug output:
    /*
        [src\main.rs:62] count = Res(
            Count(
                0,
            ),
        )
        [src\main.rs:62] count = Res(
            Count(
                1,
            ),
        )
    */
    schedule.run_once(&mut world);
}

// Each of our stages needs a label to refer to it
// By using an enum the compiler can catch careless errors
#[derive(StageLabel, Clone, PartialEq, Eq, Hash, Debug)]
enum DemoStages {
    One,
    Two,
    Three,
}

#[derive(Debug)]
struct Count(usize);

fn report_count_system(count: Res<Count>) {
    dbg!(count);
}

fn count_up_system(mut count: ResMut<Count>) {
    count.0 += 1;
}
```

Most of the time, you'll be letting your {{rust_type(type="struct" crate = "bevy_app" name="App" field="runner")}} manage stages and run schedules for you, but the more direct approach is educational and can be helpful for writing tests or handling particularly unusual app structure.

## Stages

Stages execute logic in five steps: four groups of systems, plus an additional step where commands are applied:

1. Exclusive systems are run.
   1. Use `.at_start()` to set the exclusive system to run during this step.
2. Parallel systems are run.
   1. You can change the execution strategy for this group using the `Self`-creating methods on {{rust_type(type="struct" crate="bevy_ecs" mod="schedule" name="SystemStage")}}.
   2. These use a different **executor** to change the strategy by which systems are run.
3. Exclusive systems are run.
   1. Use `.before_commands()` to set the exclusive system to run during this step.
4. Commands are applied to the world.
   1. Commands are applied one at a time, in a first-in-first-out order.
5. Exclusive systems are run.
   1. This is the default timing of exclusive systems.
   2. Use `.at_end()` to set the exclusive system to run during this step.

All systems within each group must complete before we can advance to the next step,
and thus all systems within a stage must complete before the next stage can proceed.
As a result, **adding more stage boundaries can significantly reduce the parallelism of your app**.

In most cases, explicit system ordering (see below) is sufficient to ensure that your game logic works as intended.

### Startup stages

TODO: explain startup stages

### Stage labels

As the example at the start of this section showed, each stage needs a {{rust_type(type="trait" crate="bevy_ecs" mod="schedule" name="StageLabel")}}.
These labels allow you to add stages relative to existing stages,
as well as insert systems into stages with `app.add_system_to_stage(CoreStage::PostUpdate, my_system)`.

The default stage labels provided by Bevy can be found in the {{rust_type(type="enum" crate="bevy_app" name="CoreStage")}} enum.
Any systems added to the app using `app.add_system(my_system)` will be placed in the `CoreStage::Update` stage.

While you can use strings as labels for quick prototyping, generally, you'll want to use enums for your labels.
This has several benefits:

- you can't accidentally clash with other usages
- your IDE can quickly jump to each place where it is used, and correctly rename things
- you can control [visibility](https://doc.rust-lang.org/reference/visibility-and-privacy.html) of the type
- the compiler can catch careless typos and type errors

## Parallel system scheduling

During step 2 above, systems are allowed to operate in parallel, carefully dividing up access to the `World` according to the **data access** requested by their system parameters to avoid undefined behavior.

Without any user-provided ordering constraints, systems within the same parallel phase can be executed in any order so long as Rust's ownership semantics are obeyed.
That means that a **waiting** (scheduled to run during this stage) system cannot be started if any **active** (currently running) systems are **incompatible** (cannot be scheduled at the same time) if they have conflicting data access.

Systems are incompatible if and only if:

- they need to write to the same data at the same time
- one needs to write to the data that the other is attempting to read

This is the standard "no aliased mutability" logic that Rust enforces in safe code.
Note that multiple systems can *read* from the same data at once: only writes require exclusivity.

### Execution order ambiguities

This scheduling strategy is helpful for performance reasons, but, as the precise order in which systems start and complete is nondeterministic, can result in logic bugs or inconsistencies due to **execution order ambiguities**.

The order of two systems is said to be **ambiguous** if they are (hypothetically) incompatible (share access to the same data), and do not have a well-defined order between them (either directly or indirectly).

Let's construct an example where these system order ambiguities matter, in order to demonstrate why they're painful:

```rust

// TODO: add example

```

If your game is experiencing strange inconsistent frame delays or buggy logic, you should probably consider looking at and resolving any ambiguities that touch on the area of the game responsible.

As shown in the example above, we can enable Bevy's built-in reporting tools by adding (and configuring) the {{rust_type(type="struct" crate = "bevy_ecs" name="ReportExecutionOrderAmbiguities")}} resource.
Note that if you need true determinism (e.g. for scientific simulation, tool-assisted speed-runs or lockstep networking), you will almost certainly need to ensure that your entire system graph is fully free of execution order ambiguities.

*Note for the curious:* system order ambiguities suck! Why doesn't Bevy just break ties on the basis of insertion order?
As it turns out: this is (approximately) how it used to work, back in the fabled days of Bevy 0.4 and below.
So why did we change this?

Turns out, there are a two serious problems with that design:

- system parallelism is pointlessly restricted, hurting our ultimate performance: in many cases, system order ambiguities are spurious, due to commutativity, robustness of the game systems or simple irrelevance.
- undocumented and entirely implicit dependencies are added between systems: what should be cosmetic or organizational choices (e.g. which order plugins are added in) silently causes or fixes bugs. This problem scales quadratically with the number of systems, making it a serious issue for large projects.

And so: we have maximal parallelism by default, but give users the tools to fix the execution order in ways that make sense for their application in explicit, documentable ways.

### Explicit system ordering

We can resolve system ambiguities by explicitly ordering the ambiguous systems.
In Bevy, there is currently one form of ordering: **strict ordering.**

If a system `A` is strictly before another system `B`, `B` cannot be started until `A` has completed.
These constraints are collected into a **system dependency graph**, which the **executor** uses to greedily run systems in parallel.
If this system dependency graph has any cycles (e.g. `A` must run before `B` but `B` must also run before `A`), it is said to be **unsatisfiable**, and the schedule will panic.

We can control the ordering of systems using the `.before()` and `.after()` methods, provided we have a {{rust_type(type="trait" crate="bevy_ecs" mod="system" name="SystemLabel")}} to anchor ourselves to:

```rust
use bevy::prelude::*;

fn main() {
    App::new()
        .add_system(debug_system::<1>.before("2"))
        .add_system(debug_system::<2>.label("2"))
        .add_system(debug_system::<3>.after("2"))
        .run()
}

// Showing off generic systems with const generics
fn debug_system<const N: usize>() {
    dbg!(N);
}
```

All of the [advice from above](#stage-labels) about how `enum` labels are better than string labels applies to system labels too.
Unless you're writing prototype (or example) code, spend the minute to define a real type.

Note that indirect ordering dependencies can be created, for better or worse.
If `A` must run before `B`, which must run before `C`, `C` will always run after `A`.

Due to [internal implementation details](https://github.com/bevyengine/bevy/discussions/2801), explicit dependencies cannot be declared between systems in different stages.

### Many-to-many system labels and system sets

As you experiment more with system ordering, there's a good chance you'll find yourself wishing you had more powerful, less verbose abstractions.
Fortunately, we have a (partial) remedy for you!

System labels, as introduced above, operate in a "many-to-many" fashion.
Each system can have multiple labels, and each label can be applied to multiple systems.
All systems in a given label must complete before the label is said to have been completed, and no system in a label .

This can be particularly useful for encapsulation: stick a label on all systems which perform a similar role, and then export that label type in your module or crate's public API.

Of course, if we want to apply labels to lots of systems, it would be convenient to have a tool to manipulate systems en-masse.
That's where {{rust_type(type="struct" crate = "bevy_ecs" name="SystemSet")}} comes in, allowing us to add labels, specify ordering dependencies and more to an entire collection of systems at once.
{{rust_type(type="struct" crate = "bevy_ecs" name="SystemSet")}} is just syntactic sugar: all of its methods can also be used on individual systems.

### Advanced: data access and archetypes

So far, we've hand-waved a bit when discussing which systems can run in parallel, deferring to the concept of "incompatible data access".
But what does that actually mean?

First, we have to compute the data access required by each system parameter.
In the common cases, this will either be a resource, or some fraction of the entity-component data.
Then, all of these accesses are combined across the entire system in a union-like fashion: write access trumps read access trumps no access.

Resources are simple: each resource is a unique, indivisible (at least from the scheduler's perspective) piece of data.
If a system is writing to a resource, nothing else can use that resource for any reason.
But when it comes to entities and their components things get a bit trickier.

If the scheduler were omniscient, it could allow for perfect interleaving: each atomic piece of entity-component data (a single component for a single entity) could be accessed simultaneously by separate systems.
However, the scheduler doesn't have access to this information.
All it can see are the system parameters given, and the list of current entities and components that exist.

If we have a particular query, say `Query<&mut Life, With<Player>>`, we know for sure that it won't access any entity-component data other than the `Life` component.
Naively, we could lock that entire column, and then move on to scheduling the next system.

However, we can do better than that!
We know that only entities with both the `Player` and `Life` components will be touched: any `Life` component value that belongs to an entity that does not have a `Player` component cannot be read or modified by this query.
Similarly, if we have a more complex query, say `Query<(&Life, &Attack), (With<InCombat>, Without<Elf>)>`, we can further restrict the possible data this system could touch.

This idea can be formalized by working with the concept of an **archetype.**
In Bevy, an archetype is a group of entities that share the same set of components.

Conceptually, each query defines a set of possible archetypes that *could* be accessed.
Suppose we have three components in our world: `A`, `B` and `C`.
This gives us 2^3 = 8 possible archetypes, given by the [power set](https://en.wikipedia.org/wiki/Power_set) of our components:

1. `{}`, the naked, component-less entity
2. `{A}`, the entity with only the component `A`
3. `{B}`
4. `{C}`
5. `{A, B}`
6. `{A, C}`
7. `{B, C}`
8. `{A, B, C}`, the entity which has all of our components.

Every entity that *could* exist belongs to exactly one of these archetypes, so we can reason about archetypes rather than individual entities.

Let's take a look at a couple of example queries, and see which of these archetypes they can access:

1. `Query<&A>`: archetypes 1, 5 and 8.
2. `Query<&mut B>`: archetypes 2, 5 and 8.
3. `Query<(&B, &mut C)>`: archetypes 7 and 8.
4. `Query<&mut A, Without<B>>`: archetype 1 and 6.
5. `Query<&mut A, With<B>>`: archetypes 5 and 8.
6. `Query<&mut B, With<A>>`: archetypes 5 and 8.
7. `Query<Entity>:` no access! We can't modify the `Entity` identifiers during runtime, so we can always read these freely.

Based on these archetype accesses alone, we can guarantee that query 2 and query 4 are safe to run concurrently: it is impossible for them to touch the same archetypes.
This makes sense: an entity cannot both *have* and *not have* a `B` component.

But what about queries 5 and 6?
They both touch the same archetypes, but *intuitively*, they're perfectly safe to run together, as they touch different data.
We see the same thing with queries 1 and 2, which both touch archetype 8!

There's another layer that we're missing: archetype accesses need to be divided by component type as well.
These **archetype-components** are the atoms of data access (for entity-component data): they capture all of the information we need, and cannot be broken down further.

Let's redo our analysis, but looking at archetype-components instead (with the component in question denoted by a letter):

1. `Query<&A>`: archetype-components 1A, 5A and 8A.
2. `Query<&mut B>`: archetype-components 2B, 5B and 8B.
3. `Query<(&B, &mut C)>`: archetype-components 7B, 7C and 8B and 8C.
4. `Query<&mut A, Without<B>>`: archetype-components 1A and 6A.
5. `Query<&mut A, With<B>>`: archetype-components 5A and 8A.
6. `Query<&mut B, With<A>>`: archetypes 5B and 8B.

Aha!
Queries 2 and 4 do not share any archetype-component accesses, and neither do queries 5 and 6 or 1 and 2!
But queries 1 and 5, and queries 2 and 6 *do* conflict, as we intuitively expected.

This was what was meant by "system parameters may not conflict": the scheduler must be able to *prove* that all queries in a given system cannot access the same data in a way that would violate aliased mutability.
By analyzing the archetype-components accessed, we can prove that *any* possible entity would be safe, avoiding bizarre panics in the middle of our game.

However, when we're scheduling systems, we don't care about the **hypothetical incompatibility** within the set of all possible archetypes.
Instead, we only care about the much less restrictive **factual incompatibility** within the set of archetypes that *actually* have entities.
If the data accesses are disjoint in practice, it's impossible (from the perspective of the `World`) to tell which system ran first.

This lets us arrange the real archetype-components into a two-dimensional array, and then use dense bitset operations to very quickly check if a system is in conflict with any currently running systems.
As a result, systems that would *theoretically* block each other may not in practice, depending on which components you've added to your archetypes.
