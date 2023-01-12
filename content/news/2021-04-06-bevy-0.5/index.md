+++
title = "Bevy 0.5"
date = 2021-04-06
[extra]
author = "Carter Anderson"
twitter = "cart_cart"
github = "cart"
youtube = "cartdev"
image = "ante.png"
show_image = true
image_subtitle = "Screenshot of Ante: a voxel builder game being developed in Bevy by @TheNeikos"
image_subtitle_link = ""
+++

Thanks to **88** contributors, **283** pull requests, and our [**generous sponsors**](https://github.com/sponsors/cart), I'm happy to announce the **Bevy 0.5** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out [Quick Start Guide](/learn/book/getting-started/) to get started. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Awesome Bevy](https://github.com/bevyengine/awesome-bevy) for a list of community-developed plugins, games, and learning resources.

**Bevy 0.5** is quite a bit bigger than our past few releases (and took a bit longer) as we have made a number of foundational changes. If you plan on updating your App or Plugin to **Bevy 0.5**, check out our [0.4 to 0.5 Migration Guide](/learn/book/migration-guides/0.4-0.5/).

Here are some of the highlights from this release:

<!-- more -->

## Physically Based Rendering (PBR)

<div class="release-feature-authors">authors: @StarArawn, @mtsr, @mockersf, @IngmarBitter, @Josh015, @norgate, @cart</div>

Bevy now uses PBR shaders when rendering. PBR is a semi-standard approach to rendering that attempts to use approximations of real-world "physically based" lighting and material properties. We largely use techniques from the [Filament](https://github.com/google/filament/) PBR implementation, but we also incorporate some ideas from [Unreal](https://www.unrealengine.com/en-US/blog/physically-based-shading-on-mobile) and [Disney](https://google.github.io/filament/Filament.html#citation-burley12).

Bevy's `StandardMaterial` now has `base_color`, `roughness`, `metallic`, `reflection`, and `emissive` properties. It also now supports textures for `base_color`, `normal_map`, `metallic_roughness`, `emissive`, and `occlusion` properties.

The new PBR example helps visualize these new material properties:

![pbr](pbr.png)

## GLTF Improvements

### PBR Textures

<div class="release-feature-authors">authors: @mtsr, @mockersf</div>

The GLTF loader now supports normal maps, metallic/roughness, occlusion, and emissive textures. Our "flight helmet" gltf example utilizes the new PBR texture support and looks much nicer as a result:

<video controls loop><source src="flighthelmet.mp4" type="video/mp4"/></video>

### Top-Level GLTF Asset

<div class="release-feature-authors">authors: @mockersf</div>

Previously it was hard to interact with GLTF assets because scenes / meshes / textures / and materials were only loaded as "sub assets". Thanks to the new top level {{rust_type(type="struct" crate="bevy_gltf" version="0.5.0" name="Gltf" no_mod=true)}} asset type, it is now possible to navigate the contents of the GLTF asset:

```rust
// load GLTF asset on startup
fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let handle = assets.load("flight_helmet.gltf");
    commands.insert_resource(handle);
}

// access GLTF asset at some later point in time
fn system(handle: Res<Handle<Gltf>>, gltfs: Res<Assets<Gltf>>, materials: Res<Assets<StandardMaterial>>) {
    let gltf = gltfs.get(&handle).unwrap();
    let material_handle = gltf.named_materials.get("MetalPartsMat").unwrap();
    let material = materials.get(material_handle).unwrap();
}
```

## Bevy ECS V2

This release marks a huge step forward for Bevy's ECS. It has significant implications for how Bevy Apps are composed and how well they perform:

* **[A full rewrite of the ECS core:](#ecs-core-rewrite)**
  * Massively improved performance across the board
  * "Hybrid" component storage
  * An "Archetype Graph" for faster archetype changes
  * Stateful queries that cache results across runs
* **[A brand new parallel System executor:](#new-parallel-system-executor)**
  * Support for explicit system ordering
  * System Labels
  * System Sets
  * Improved system "run criteria"
  * Increased system parallelism
* **["Reliable" change detection:](#reliable-change-detection)**
  * Systems will now always detect component changes, even across frames
* **[A rewrite of the State system:](#states-v2)**
  * A much more natural "stack-based state machine" model
  * Direct integration with the new scheduler
  * Improved "state lifecycle" events

Read on for the details!

## ECS Core Rewrite

<div class="release-feature-authors">authors: @cart</div>

Up until this point, Bevy used a heavily forked version of [hecs](https://github.com/Ralith/hecs) for our ECS core. Since Bevy's first release, we've learned a lot about Bevy's ECS needs. We've also collaborated with other ECS project leaders, such as [Sander Mertens](https://github.com/SanderMertens) (lead [flecs](https://github.com/SanderMertens/flecs) developer) and [Gijs-Jan Roelofs](https://github.com/gjroelofs) (Xenonauts ECS framework developer). As an "ECS community", we've started to zero in on what the future of ECS could be.

Bevy ECS v2 is our first step into that future. It also means that Bevy ECS is no longer a "hecs fork". We are going out on our own!

### Component Storage (The Problem)

Two ECS storage paradigms have gained a lot of traction over the years:

* **Archetypal ECS**:
  * Stores components in "tables" with static schemas. Each "column" stores components of a given type. Each "row" is an entity.
  * Each "archetype" has its own table. Adding/removing an entity's component changes the archetype.
  * Enables super-fast Query iteration due to its cache-friendly data layout
  * Comes at the cost of more expensive add/remove operations for an Entity's components, because all components need to be copied to the new archetype's "table"
  * Parallelism-friendly: entities only exist in one archetype at a time so systems that access the same components but in different archetypes can run in parallel
  * Frameworks: Old Bevy ECS, hecs, legion, flecs, Unity DOTS
* **Sparse Set ECS**:
  * Stores components of the same type in densely packed arrays, which are sparsely indexed by densely packed unsigned integers (entity ids)
  * Query iteration is slower than Archetypal ECS (by default) because each entity's component could be at any position in the sparse set. This "random access" pattern isn't cache friendly. Additionally, there is an extra layer of indirection because you must first map the entity id to an index in the component array.
  * Adding/removing components is a cheap, constant time operation
  * "Component Packs" are used to optimize iteration performance on a case by case basis (but packs conflict with each other)
  * Less parallelism friendly: systems need to either lock a whole component storage (not granular) or individual entities (expensive)
  * Frameworks: Shipyard, EnTT

Developers selecting an ECS framework are stuck with a hard choice. Select an "archetypal" framework with "fast iteration everywhere" but without the ability to cheaply add/remove components, or select a "sparse set" framework to cheaply add/remove components but with slower iteration performance or manual (and conflicting) pack optimizations.

### Hybrid Component Storage (The Solution)

In Bevy ECS V2, we get to have our cake and eat it too. It now has _both_ of the component storage types above (and more can be added later if needed):

* **Tables** (aka "archetypal" storage in other frameworks)
  * The default storage. If you don't configure anything, this is what you get
  * Fast iteration by default
  * Slower add/remove operations
* **Sparse Sets**
  * Opt-in
  * Slower iteration
  * Faster add/remove operations

These storage types complement each other perfectly. By default Query iteration is fast. If developers know that they want to add/remove a component at high frequencies, they can set the storage to "sparse set":

```rust
app.register_component(
    ComponentDescriptor::new::<MyComponent>(StorageType::SparseSet)
);
```

#### Component Add/Remove Benchmark (in milliseconds, less is better)

This benchmark illustrates adding and removing a single 4x4 matrix component 10,000 times from an entity that has 5 other 4x4 matrix components. The "other" components are included to help illustrate the cost of "table storage" (used by Bevy 0.4, Bevy 0.5 (Table), and Legion), which requires moving the "other" components to a new table.

![component add/remove](add_remove_big.svg)

You may have noticed that **Bevy 0.5 (Table)** is also _way_ faster than **Bevy 0.4**, even though they both use "table storage". This is largely a result of the new [Archetype Graph](https://github.com/bevyengine/bevy/pull/1525), which significantly cuts the cost of archetype changes.

### Stateful Queries and System Parameters

{{rust_type(type="struct" crate="bevy_ecs" mod="world" version="0.5.0" name="World" no_mod=true)}} queries (and other system parameters) are now stateful. This allows us to:

1. Cache archetype (and table) matches
    * This resolves another issue with (naive) archetypal ECS: query performance getting worse as the number of archetypes goes up (and fragmentation occurs).
2. Cache Query Fetch and Filter state
    * The expensive parts of fetch/filter operations (such as hashing the TypeId to find the ComponentId) now only happen once when the Query is first constructed
3. Incrementally build up state
    * When new archetypes are added, we only process the new archetypes (no need to rebuild state for old archetypes)

As a result, the direct {{rust_type(type="struct" crate="bevy_ecs" mod="world" version="0.5.0" name="World" no_mod=true)}} query api now looks like this:

```rust
let mut query = world.query::<(&A, &mut B)>();
for (a, mut b) in query.iter_mut(&mut world) {
}
```

However for {{rust_type(type="trait" crate="bevy_ecs" mod="system" version="0.5.0" name="System" no_mod=true plural=true)}}  this is a non-breaking change. Query state management is done internally by the relevant SystemParam.

We have achieved some pretty significant performance wins as a result of the new Query system.

#### "Sparse" Fragmented Iterator Benchmark (in nanoseconds, less is better)

This benchmark runs a query that matches 5 entities within a single archetype and _doesn't_ match 100 other archetypes. This is a reasonable test of "real world" queries in games, which generally have many different entity "types", most of which _don't_ match a given query. This test uses "table storage" across the board.

![sparse_frag_iter](sparse_frag_iter.svg)

**Bevy 0.5** marks a huge improvement for cases like this, thanks to the new "stateful queries". **Bevy 0.4** needs to check every archetype each time the iterator is run, whereas **Bevy 0.5** amortizes that cost to zero.

#### Fragmented Iterator Benchmark (in milliseconds, less is better)

This is the [ecs_bench_suite](https://github.com/rust-gamedev/ecs_bench_suite) `frag_iter` benchmark. It runs a query on 27 archetypes with 20 entities each. However unlike the "Sparse Fragmented Iterator Benchmark", there are no "unmatched" archetypes. This test uses "table storage" across the board.

![frag_iter](frag_iter.svg)

The gains here compared to the last benchmark are smaller because there aren't any unmatched archetypes. However **Bevy 0.5** still gets a nice boost due to better iterator/query impls, amortizing the cost of matched archetypes to zero, and for_each iterators.

### Uber Fast "for_each" Query Iterators

Developers now have the choice to use a fast {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.5.0" name="Query" no_mod=true method="for_each")}} iterator, which yields ~1.5-3x iteration speed improvements for "fragmented iteration", and minor ~1.2x iteration speed improvements for unfragmented iteration.

```rust
fn system(query: Query<(&A, &mut B)>) {
    // you now have the option to do this for a speed boost
    query.for_each_mut(|(a, mut b)| {
    });

    // however normal iterators are still available
    for (a, mut b) in query.iter_mut() {
    }
}
```

We will continue to encourage "normal" iterators as they are more flexible and more "rust idiomatic". But when that extra "oomf" is needed, `for_each` will be there ... waiting for you :)

## New Parallel System Executor

<div class="release-feature-authors">authors: @Ratysz</div>

Bevy's old parallel executor had a number of fundamental limitations:

1. The only way to explicitly define system order was to create new stages. This was both boilerplate-ey and prevented parallelism (because stages run "one by one" in order). We've noticed that system ordering is a common requirement and stages just weren't cutting it.
2. Systems had "implicit" orderings when they accessed conflicting resources. These orderings were hard to reason about.
3. The "implicit orderings" produced execution strategies that often left a lot of parallelism potential on the table.

Fortunately @Ratysz has been [doing](https://ratysz.github.io/article/scheduling-1/) a lot of [research](https://github.com/Ratysz/yaks/) in this area and volunteered to contribute a new executor. The new executor solves all of the issues above and also adds a bunch of new usability improvements. The "ordering" rules are now dead-simple:

1. Systems run in parallel by default
2. Systems with explicit orderings defined will respect those orderings

### Explicit System Dependencies and System Labels

<div class="release-feature-authors">authors: @Ratysz, @TheRawMeatball</div>

Systems can now be assigned one or more {{rust_type(type="trait" crate="bevy_ecs" mod="schedule" version="0.5.0" name="SystemLabel" no_mod=true plural=true)}}. These labels can then be referenced by other systems (within a stage) to run before or after systems with that label:

```rust
app
    .add_system(update_velocity.system().label("velocity"))
    // The "movement" system will run after "update_velocity" 
    .add_system(movement.system().after("velocity"))
```

This produces an equivalent ordering, but it uses `before()` instead.

```rust
app
    // The "update_velocity" system will run before "movement" 
    .add_system(update_velocity.system().before("movement"))
    .add_system(movement.system().label("movement"));
```

Any type that implements the {{rust_type(type="trait" crate="bevy_ecs" mod="schedule" version="0.5.0" name="SystemLabel" no_mod=true)}} trait can be used. In most cases we recommend defining custom types and deriving {{rust_type(type="trait" crate="bevy_ecs" mod="schedule" version="0.5.0" name="SystemLabel" no_mod=true)}} for them. This prevents typos, allows for encapsulation (when needed), and allows IDEs to autocomplete labels:

```rust
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    UpdateVelocity,
    Movement,
}

app
    .add_system(update_velocity.system().label(PhysicsSystem::UpdateVelocity))
    .add_system(movement.system()
        .label(PhysicsSystem::Movement)
        .after(PhysicsSystem::UpdateVelocity)
    );
```

### Many-to-Many System Labels

Many-to-many labels is a powerful concept that makes it easy to take a dependency on many systems that produce a given behavior/outcome. For example, if you have a system that needs to run after all "physics" has finished updating (see the example above), you could label all "physics systems" with the same `Physics` label:

```rust
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub struct Physics;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemLabel)]
pub enum PhysicsSystem {
    UpdateVelocity,
    Movement,
}

app
    .add_system(update_velocity.system()
        .label(PhysicsSystem::UpdateVelocity)
        .label(Physics)
    )
    .add_system(movement.system()
        .label(PhysicsSystem::Movement)
        .label(Physics)
        .after(PhysicsSystem::UpdateVelocity)
    )
    .add_system(runs_after_physics.system().after(Physics));
```

Bevy plugin authors should export labels like this in their public APIs to enable their users to insert systems before/after logic provided by the plugin.

### System Sets

{{rust_type(type="struct" crate="bevy_ecs" mod="schedule" version="0.5.0" name="SystemSet" no_mod=true plural=true)}} are a new way to apply the same configuration to a group of systems, which significantly cuts down on boilerplate. The "physics" example above could be rephrased like this:

```rust
app
    .add_system_set(SystemSet::new()
        // this label is added to all systems in the set
        .label(Physics)
        .with_system(update_velocity.system().label(PhysicsSystem::UpdateVelocity))
        .with_system(movement.system()
            .label(PhysicsSystem::Movement)
            .after(PhysicsSystem::UpdateVelocity)
        )
    )
```

SystemSets can also use `before(Label)` and `after(Label)` to run all systems in the set before/after the given label.

This is also very useful for groups of systems that need to run with the same {{rust_type(type="struct" crate="bevy_ecs" mod="schedule" version="0.5.0" name="RunCriteria" no_mod=true)}}.

```rust
app
    // all systems in this set will run once every two seconds
    .add_system_set(SystemSet::new()
        .with_run_criteria(FixedTimestep::step(2.0))
        .with_system(foo.system())
        .with_system(bar.system())
    )
```

### Improved Run Criteria

Run Criteria are now decoupled from systems and will be re-used when possible. For example, the FixedTimestep criteria in the example above will only be run once per stage run. The executor will re-use the criteria's result for both the `foo` and `bar` system.

Run Criteria can now also be labeled and referenced by other systems:

```rust
fn every_other_time(mut has_ran: Local<bool>) -> ShouldRun {
    *has_ran = !*has_ran;
    if *has_ran {
        ShouldRun::Yes
    } else {
        ShouldRun::No
    }
}

app.add_stage(SystemStage::parallel()
   .with_system_run_criteria(every_other_time.system().label("every_other_time")))
   .add_system(foo.system().with_run_criteria("every_other_time"))
```

Results from Run Criteria can also be "piped" into other criteria, which enables interesting composed behaviors:

```rust
fn once_in_a_blue_moon(In(input): In<ShouldRun>, moon: Res<Moon>) -> ShouldRun {
    if moon.is_blue() {
        input
    } else {
        ShouldRun::No
    }
}

app
    .add_system(foo.with_run_criteria(
        "every_other_time".pipe(once_in_a_blue_moon.system())
    )
```

### Ambiguity Detection and Resolution

While the new executor is now much easier to reason about, it does introduce a new class of error: "system order ambiguities". When two systems interact with the same data, but have no explicit ordering defined, the output they produce is non-deterministic (and often not what the author intended).

Consider the following app:

```rust
fn increment_counter(mut counter: ResMut<usize>) {
    *counter += 1;
}

fn print_every_other_time(counter: Res<usize>) {
    if *counter % 2 == 0 {
        println!("ran");
    }
}

app
    .add_system(increment_counter.system())
    .add_system(print_every_other_time.system())
```

The author clearly intended `print_every_other_time` to run every other update. However, due to the fact that these systems have no order defined, they could run in a different order each update and create a situation where nothing is printed over the course of two updates:

```txt
UPDATE
- increment_counter (counter now equals 1)
- print_every_other_time (nothing printed)
UPDATE
- print_every_other_time (nothing printed)
- increment_counter (counter now equals 2)
```

The old executor would have implicitly forced `increment_counter` to run first because it conflicts with `print_every_other_time` and it was inserted first. But the new executor requires you to be explicit here (which we believe is a good thing).

To help detect this class of error, we built an opt-in tool that detects these ambiguities and logs them:

```rust
// add this resource to your App to enable ambiguity detection
app.insert_resource(ReportExecutionOrderAmbiguities)
```

Then when we run our App, we will see the following message printed to our terminal:

```txt
Execution order ambiguities detected, you might want to add an explicit dependency relation between some of these systems:
 * Parallel systems:
 -- "&app::increment_counter" and "&app::print_every_other_time"
    conflicts: ["usize"]
```

The ambiguity detector found a conflict and mentions that adding an explicit dependency would resolve the conflict:

```rust
app
    .add_system(increment_counter.system().label("increment"))
    .add_system(print_every_other_time.system().after("increment"))
```

There _are_ some cases where ambiguities are _not_ a bug, such as operations on unordered collection like `Assets`. This is why we don't enable the detector by default. You are free to just ignore these ambiguities, but if you want to suppress the messages in the detector (without defining a dependency), you can add your systems to an "ambiguity set":

```rust
app
    .add_system(a.system().in_ambiguity_set("foo"))
    .add_system(b.system().in_ambiguity_set("foo"))
```

I want to stress that this is totally optional. Bevy code should be ergonomic and "fun" to write. If sprinkling ambiguity sets everywhere isn't your cup of tea, just don't worry about it!

We are also actively seeking feedback on the new executor. We believe that the new implementation is easier to understand and encourages self-documenting code. The improved parallelism is also nice! But we want to hear from users (both new users starting fresh and old users porting their codebases to the new executor). This space is all about design tradeoffs and feedback will help us ensure we made the right calls.

## Reliable change detection

<div class="release-feature-authors">authors: @Davier, @bjorn3, @alice-i-cecile, @cart</div>

Global change detection, the ability to run queries on the Changed/Added status of any ECS component or resource, just got a major usability boost: changes are now detected across frames/updates:

```rust
// This is still the same change detection api we all know and love,
// the only difference is that it "just works" in every situation.
fn system(query: Query<Entity, Changed<A>>) {
    // iterates all entities whose A component has changed since
    // the last run of this system 
    for e in query.iter() {
    }
}
```

Global change detection was already a feature that set Bevy apart from other ECS frameworks, but now it is completely "fool proof". It works as expected regardless of system ordering, stage membership, or system run criteria.

The old behavior was "systems detect changes that ocurred in systems that ran before them this frame". This was because we used a `bool` to track when each component/resource is added/modified. This flag was cleared for each component at the end of the frame. As a result, users had to be very careful about order of operations, and using features like "system run criteria" could result in dropped changes if systems didn't run on a given update.

We now use a clever "world tick" design that allows systems to detect changes that happened at _any_ point in time since their last run.

## States V2

<div class="release-feature-authors">authors: @TheRawMeatball</div>

The [last Bevy release](https://bevyengine.org/news/bevy-0-4) added States, which enabled developers to run groups of ECS systems according to the value of a `State<T>` resource. Systems could be run according to "state lifecycle events", such as on_enter, on_update, and on_exit. States make things like separate "loading screen" and "in game" logic easier to encode in Bevy ECS.

The old implementation largely worked, but it had a number of quirks and limitations. First and foremost, it required adding a new `StateStage`, which cut down on parallelism, increased boilerplate, and forced ordering where it wasn't required. Additionally, some of the lifecycle events didn't always behave as expected.

The new {{rust_type(type="struct" crate="bevy_ecs" mod="schedule" version="0.5.0" name="State" no_mod=true)}} implementation is built on top of the new parallel executor's SystemSet and RunCriteria features, for a much more natural, flexible, and parallel api that builds on existing concepts instead of creating new ones:

```rust
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum AppState {
    Menu,
    InGame,
}

fn main() {
    App::build()
        .add_state(AppState::Menu)
        .add_system_set(SystemSet::on_enter(AppState::Menu).with_system(setup_menu.system()))
        .add_system_set(SystemSet::on_update(AppState::Menu).with_system(menu_logic.system()))
        .add_system_set(SystemSet::on_exit(AppState::Menu).with_system(cleanup_menu.system()))
        .add_system_set(SystemSet::on_enter(AppState::InGame).with_system(setup_game.system()))
        .add_system_set(
            SystemSet::on_update(AppState::InGame)
                .with_system(game_logic.system())
                .with_system(more_game_logic.system())
        )
        .run();
}
```

States now use a "stack-based state machine" model. This opens up a number of options for state transitions:

```rust
fn system(mut state: ResMut<State<AppState>>) {
    // Queues up a state change that pushes a new state on to the
    // stack (preserving previous states)
    state.push(AppState::InGame).unwrap();

    // Queues up a state change that removes the current state on
    // the stack and reverts to the previous state
    state.pop().unwrap();

    // Queues up a state change that overwrites the current state at
    // the "top" of the stack
    state.set(AppState::InGame).unwrap();

    // Queues up a state change that replaces the entire stack of states
    state.replace(AppState::InGame).unwrap();
}
```

Just like the old implementation, state changes are applied in the same frame. This means it is possible to transition from states `A->B->C` and run the relevant state lifecycle events without skipping frames. This builds on top of "looping run criteria", which we also use for our "fixed timestep" implementation (and which you can use for your own run criteria logic).

## Event Ergonomics

<div class="release-feature-authors">authors: @TheRawMeatball</div>

Events now have a first-class shorthand syntax for easier consumption:

```rust
// Old Bevy 0.4 syntax
fn system(mut reader: Local<EventReader<SomeEvent>>, events: Res<Events<SomeEvent>>) {
    for event in reader.iter(&events) {
    }
}

// New Bevy 0.5 syntax
fn system(mut reader: EventReader<SomeEvent>) {
    for event in reader.iter() {
    }
}
```

There is also now a symmetrical `EventWriter` api:

```rust
fn system(mut writer: EventWriter<SomeEvent>) {
    writer.send(SomeEvent { ... })
}
```

The old "manual" approach is still possible via `ManualEventReader`:

```rust
fn system(mut reader: Local<ManualEventReader<SomeEvent>>, events: Res<Events<SomeEvent>>) {
    for event in reader.iter(&events) {
    }
}
```

## Rich Text

<div class="release-feature-authors">authors: @tigregalis</div>

Text can now have "sections", each with their own style / formatting. This makes text much more flexible, while still respecting the text layout rules:

![rich_text](rich_text.png)

This is accomplished using the new "text section" api:

```rust
commands
    .spawn_bundle(TextBundle {
        text: Text {
            sections: vec![
                TextSection {
                    value: "FPS: ".to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraSans-Bold.ttf"),
                        font_size: 90.0,
                        color: Color::WHITE,
                    },
                },
                TextSection {
                    value: "60.03".to_string(),
                    style: TextStyle {
                        font: asset_server.load("FiraMono-Medium.ttf"),
                        font_size: 90.0,
                        color: Color::GOLD,
                    },
                },
            ],
            ..Default::default()
        },
        ..Default::default()
    })
```

## HIDPI Text

<div class="release-feature-authors">authors: @blunted2night</div>

Text is now rendered according to the current monitor's scale factor. This gives nice, crisp text at any resolution.

![hidpi_text](hidpi_text.png)

## Render Text in 2D World Space

<div class="release-feature-authors">authors: @CleanCut, @blunted2night</div>

Text can now be spawned into 2D scenes using the new `Text2dBundle`. This makes it easier to do things like "draw names above players".

<video controls loop><source src="2d_text.mp4" type="video/mp4"/></video>

## World To Screen Coordinate Conversions

<div class="release-feature-authors">authors: @aevyrie</div>

It is now possible to convert world coordinates to a given camera's screen coordinates using the new `Camera::world_to_screen()` function. Here is an example of this feature being used to position a UI element on top of a moving 3d object.

<video controls loop><source src="world_to_screen.mp4" type="video/mp4"/></video>

## 3D Orthographic Camera

<div class="release-feature-authors">authors: @jamadazi</div>

Orthographic cameras can now be used in 3D! This is useful for things like CAD applications and isometric games.

![ortho_3d](ortho_3d.png)

## Orthographic Camera Scaling Modes

<div class="release-feature-authors">authors: @jamadazi</div>

Prior to **Bevy 0.5**, Bevy's orthographic camera had only one mode: "window scaling". It would adapt the projection according to the vertical and horizontal size of the window. This works for some styles of games, but other games need arbitrary window-independent scale factors or scale factors defined by either horizontal or vertical window sizes.

**Bevy 0.5** adds a new `ScalingMode` option to `OrthographicCamera`, which enables developers to customize how the projection is calculated.

It also adds the ability to "zoom" the camera using `OrthographicProjection::scale`.

## Flexible Camera Bindings

<div class="release-feature-authors">authors: @cart</div>

Bevy used to "hack in" camera bindings for each RenderGraph PassNode. This worked when there was only one binding type (the combined `ViewProj` matrix), but many shaders require other camera properties, such as the world space position.

In Bevy 0.5 we removed the "hack" in favor of the `RenderResourceBindings` system used elsewhere. This enables shaders to bind arbitrary camera data (with any set or binding index) and only pull in the data they need.

The new PBR shaders take advantage of this feature, but custom shaders can also use it.

```glsl
layout(set = 0, binding = 0) uniform CameraViewProj {
    mat4 ViewProj;
};
layout(set = 0, binding = 1) uniform CameraPosition {
    vec3 CameraPos;
};
```

## Render Layers

<div class="release-feature-authors">authors: @schell</div>

Sometimes you don't want a camera to draw everything in a scene, or you want to temporarily hide a set of things in the scene. **Bevy 0.5** adds a `RenderLayer` system, which gives developers the ability to add entities to layers by adding the `RenderLayers` component.

Cameras can also have a {{rust_type(type="struct" crate="bevy_render" mod="camera" version="0.5.0" name="RenderLayers" no_mod=true)}} component, which determines what layers they can see.

```rust
// spawn a sprite on layer 0
commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -50.0, 1.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
    })
    .insert(RenderLayers::layer(0));
// spawn a sprite on layer 1
commands
    .spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(1.0, 0.5, 0.5).into()),
        transform: Transform::from_xyz(0.0, -50.0, 1.0),
        sprite: Sprite::new(Vec2::new(30.0, 30.0)),
    })
    .insert(RenderLayers::layer(1));
// spawn a camera that only draws the sprite on layer 1
commands
    .spawn_bundle(OrthographicCameraBundle::new_2d());
    .insert(RenderLayers::layer(1));
```

## Sprite Flipping

<div class="release-feature-authors">authors: @zicklag</div>

Sprites can now be easily (and efficiently) flipped along the x or y axis:

![sprite_flipping](sprite_flipping.png)

```rust
commands.spawn_bundle(SpriteBundle {
    material: material.clone(),
    transform: Transform::from_xyz(150.0, 0.0, 0.0),
    ..Default::default()
});
commands.spawn_bundle(SpriteBundle {
    material,
    transform: Transform::from_xyz(-150.0, 0.0, 0.0),
    sprite: Sprite {
        // Flip the logo to the left
        flip_x: true,
        // And don't flip it upside-down ( the default )
        flip_y: false,
        ..Default::default()
    },
    ..Default::default()
});
```

## Color Spaces

<div class="release-feature-authors">authors: @mockersf</div>

{{rust_type(type="enum" crate="bevy_render" mod="color" version="0.5.0" name="Color" no_mod=true)}} is now internally represented as an enum, which enables lossless (and correct) color representation. This is a significant improvement over the previous implementation, which internally converted all colors to linear sRGB (which could cause precision issues). Colors are now only converted to linear sRGB when they are sent to the GPU. We also took this opportunity to fix some incorrect color constants defined in the wrong color space.

```rust
pub enum Color {
    /// sRGBA color
    Rgba {
        /// Red component. [0.0, 1.0]
        red: f32,
        /// Green component. [0.0, 1.0]
        green: f32,
        /// Blue component. [0.0, 1.0]
        blue: f32,
        /// Alpha component. [0.0, 1.0]
        alpha: f32,
    },
    /// RGBA color in the Linear sRGB colorspace (often colloquially referred to as "linear", "RGB", or "linear RGB").
    RgbaLinear {
        /// Red component. [0.0, 1.0]
        red: f32,
        /// Green component. [0.0, 1.0]
        green: f32,
        /// Blue component. [0.0, 1.0]
        blue: f32,
        /// Alpha component. [0.0, 1.0]
        alpha: f32,
    },
    /// HSL (hue, saturation, lightness) color with an alpha channel
    Hsla {
        /// Hue component. [0.0, 360.0]
        hue: f32,
        /// Saturation component. [0.0, 1.0]
        saturation: f32,
        /// Lightness component. [0.0, 1.0]
        lightness: f32,
        /// Alpha component. [0.0, 1.0]
        alpha: f32,
    },
}
```

## Wireframes

<div class="release-feature-authors">authors: @Neo-Zhixing</div>

Bevy can now draw wireframes using the opt-in `WireframePlugin`

![wireframe](wireframe.png)

These can either be enabled globally or per-entity by adding the new `Wireframe` component.

## Simple 3D Game Example: Alien Cake Addict

<div class="release-feature-authors">authors: @mockersf</div>

This example serves as a quick introduction to building 3D games in Bevy. It shows how to spawn scenes, respond to input, implement game logic, and handle state transitions. Pick up as many cakes as you can!

![alien_cake_addict](alien_cake_addict.png)

## Timer Improvements

<div class="release-feature-authors">authors: @kokounet</div>

The {{rust_type(type="struct" crate="bevy_core" version="0.5.0" name="Timer" no_mod=true)}} struct now internally uses {{rust_type(type="struct" crate="std" mod="time" name="Duration" no_mod=true plural=true)}} instead of using `f32` representations of seconds. This both increases precision and makes the api a bit nicer to look at.

```rust
fn system(mut timer: ResMut<Timer>, time: Res<Time>) {
    if timer.tick(time.delta()).just_finished() {
        println!("timer just finished");
    }
}
```

## Assets Improvements

<div class="release-feature-authors">authors: @willcrichton, @zicklag, @mockersf, @Archina</div>

Bevy's asset system had a few minor improvements this release:

* Bevy no longer panics on errors when loading assets
* Asset paths with multiple dots are now properly handled
* Improved type safety for "labeled assets" produced by asset loaders
* Made asset path loading case-insensitive

## WGPU Configuration Options

<div class="release-feature-authors">authors: @Neo-Zhixing</div>

It is now possible to enable/disable wgpu features (such as `WgpuFeature::PushConstants` and `WgpuFeature::NonFillPolygonMode`) by setting them in the `WgpuOptions` resource:

```rust
app
    .insert_resource(WgpuOptions {
        features: WgpuFeatures {
            features: vec![WgpuFeature::NonFillPolygonMode],
        },
        ..Default::default()
    })

```

Wgpu limits (such as `WgpuLimits::max_bind_groups`) can also now be configured in the `WgpuOptions` resource.

## Scene Instance Entity Iteration

<div class="release-feature-authors">authors: @mockersf</div>

It is now possible to iterate all entities in a spawned scene instance. This makes it possible to perform post-processing on scenes after they have been loaded.

```rust
struct MySceneInstance(InstanceId);

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut scene_spawner: ResMut<SceneSpawner>,
) {
    // Spawn a scene and keep its `instance_id`
    let instance_id = scene_spawner.spawn(asset_server.load("model.gltf#Scene0"));
    commands.insert_resource(MySceneInstance(instance_id));
}

fn print_scene_entities(
    scene_spawner: Res<SceneSpawner>,
    scene_instance: Res<MySceneInstance>,
) {
    if let Some(entity_iter) = scene_spawner.iter_instance_entities(scene_instance.0) {
        for entity in entity_iter {
            println!("Found scene entity {:?}", entity);
        }
    }
}
```

## Window Resize Constraints

<div class="release-feature-authors">authors: @digital7-code</div>

Windows can now have "resize constraints". Windows cannot be resized past these constraints

```rust
app
    .insert_resource(WindowDescriptor {
        resize_constraints: WindowResizeConstraints {
            min_height: 200.0,
            max_height: 800.0,
            ..Default::default()
        },
        ..Default::default()
    })
```

## !Send Tasks

<div class="release-feature-authors">authors: @alec-deason</div>

Bevy's async task system now supports `!Send` tasks. Some tasks cannot be sent / run on other threads (such as tasks created by the upcoming Distill asset plugin). "Thread local" tasks can now be spawned in Bevy `TaskPools` like this:

```rust
let pool = TaskPool::default();
pool.scope(|scope| {
    scope.spawn_local(async move {
        println!("I am a local task");
    });
});
```

## More ECS V2 Changes

### EntityRef / EntityMut

<div class="release-feature-authors">authors: @cart</div>

World entity operations in **Bevy 0.4** require that the user passes in an `entity` id to each operation:

```rust
let entity = world.spawn((A, )); // create a new entity with A
world.get::<A>(entity);
world.insert(entity, (B, C));
world.insert_one(entity, D);
```

This means that each operation needs to look up the entity location / verify its validity. The initial spawn operation also requires a Bundle as input. This can be awkward when no components are required (or one component is required).

These operations have been replaced by `EntityRef` and `EntityMut`, which are "builder-style" wrappers around world that provide read and read/write operations on a single, pre-validated entity:

```rust
// spawn now takes no inputs and returns an EntityMut
let entity = world.spawn()
    .insert(A) // insert a single component into the entity
    .insert_bundle((B, C)) // insert a bundle of components into the entity
    .id() // id returns the Entity id

// Returns EntityMut (or panics if the entity does not exist)
world.entity_mut(entity)
    .insert(D)
    .insert_bundle(SomeBundle::default());

// The `get_X` variants return Options, in case you want to check existence instead of panicking 
world.get_entity_mut(entity)
    .unwrap()
    .insert(E);

if let Some(entity_ref) = world.get_entity(entity) {
    let d = entity_ref.get::<D>().unwrap();
}
```

`Commands` have also been updated to use this new pattern

```rust
let entity = commands.spawn()
    .insert(A)
    .insert_bundle((B, C))
    .insert_bundle(SomeBundle::default())
    .id();
```

`Commands` also still support spawning with a Bundle, which should make migration from **Bevy 0.4** easier. It also cuts down on boilerplate in some situations:

```rust
commands.spawn_bundle(SomeBundle::default());
```

Note that these Command methods use the "type state" pattern, which means this style of chaining is no longer possible:

```rust
// Spawns two entities, each with the components in SomeBundle and the A component
// Valid in Bevy 0.4, but invalid in Bevy 0.5
commands
    .spawn(SomeBundle::default())
    .insert(A)
    .spawn(SomeBundle::default())
    .insert(A);
```

Instead, you should do this:

```rust
commands
    .spawn_bundle(SomeBundle::default())
    .insert(A);
commands
    .spawn_bundle(SomeBundle::default())
    .insert(A);
```

This allows us to make things like "entity id retrieval" infallible and opens the doors to future api improvements.

### Query::single

<div class="release-feature-authors">authors: @TheRawMeatball</div>

Queries now have {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.5.0" name="Query" no_mod=true method="single")}} and {{rust_type(type="struct" crate="bevy_ecs" mod="system" version="0.5.0" name="Query" no_mod=true method="single_mut")}} methods, which return a single query result if there is _exactly_ one matching entity:

```rust
fn system(query: Query<&Player>) {
    // only returns Ok if there is exactly one Player
    if let Ok(player) = query.single() {
    }
}
```

### Removed ChangedRes

<div class="release-feature-authors">authors: @TheRawMeatball</div>

We have removed `ChangedRes<A>` in favor of the following:

```rust
fn system(a: Res<A>) {
    if a.is_changed() {
        // do something
    }
}
```

### Optional Resource Queries

<div class="release-feature-authors">authors: @jamadazi</div>

It is now possible for a system to check for Resource existence via `Option` queries:

```rust
fn system(a: Option<Res<A>>) {
    if let Some(a) = a {
        // do something
    }
}
```

### New Bundle Naming Convention

Component Bundles previously used the `XComponents` naming convention (ex: `SpriteComponents`, `TextComponents`, etc). We decided to move to a `XBundle` naming convention (ex: `SpriteBundle`, `TextBundle`, etc) to be more explicit about what these types are and to help prevent new users from conflating Bundles and Components.

### World Metadata Improvements

<div class="release-feature-authors">authors: @cart</div>

`World` now has queryable `Components`, `Archetypes`, `Bundles`, and `Entities` collections:

```rust
// you can access these new collections from normal systems, just like any other SystemParam
fn system(archetypes: &Archetypes, components: &Components, bundles: &Bundles, entities: &Entities) {
}
```

This enables developers to access internal ECS metadata from their Systems.

### Configurable SystemParams

<div class="release-feature-authors">authors: @cart, @DJMcNab</div>

Users can now provide some initial configuration / values for system parameters (when possible). Most SystemParams have no config (the config type is `()`), but the `Local<T>` param now supports user-provided parameters:

```rust
fn foo(value: Local<usize>) {    
}

app.add_system(foo.system().config(|c| c.0 = Some(10)));
```

### Preparation for Scripting Support

<div class="release-feature-authors">authors: @cart</div>

Bevy ECS Components are now decoupled from Rust types. The new `Components` collection stores metadata such as memory layout and destructors. Components also no longer require Rust TypeIds.

New component metadata can be added at any time using `world.register_component()`.

All component storage types (currently Table and Sparse Set) are "blob storage". They can store any value with a given memory layout. This enables data from other sources (ex: a Python data type) to be stored and accessed in the same way as Rust data types.

We haven't completely enabled scripting yet ([and will likely never officially support non-Rust scripting](https://discord.com/channels/691052431525675048/692648082499829760/817178225791729716)), but this is a major step toward enabling community-supported scripting languages.

### Merged Resources into World

<div class="release-feature-authors">authors: @cart</div>

Resources are now just a special kind of Component. This allows us to keep the code size small by reusing existing Bevy ECS internals. It also enabled us to optimize the parallel executor access controls and it should make scripting language integration easier down the line.

```rust
world.insert_resource(1);
world.insert_resource(2.0);
let a = world.get_resource::<i32>().unwrap();
let mut b = world.get_resource_mut::<f64>().unwrap();
*b = 3.0;

// Resources are still accessed the same way in Systems
fn system(foo: Res<f64>, bar: ResMut<i32>) {
}
```

_But_ this merge did create problems for people directly interacting with `World`. What if you need mutable access to multiple resources at the same time? `world.get_resource_mut()` borrows World mutably, which prevents multiple mutable accesses! We solved this with `WorldCell`.

### WorldCell

<div class="release-feature-authors">authors: @cart</div>

WorldCell applies the "access control" concept used by Systems to direct world access:

```rust
let world_cell = world.cell();
let a = world_cell.get_resource_mut::<i32>().unwrap();
let b = world_cell.get_resource_mut::<f64>().unwrap();
```

This adds cheap runtime checks to ensure that world accesses do not conflict with each other.

We made this a separate api to enable users to decide what tradeoffs they want. Direct World access has stricter lifetimes, but it is more efficient and does compile time access control. `WorldCell` has looser lifetimes, but incurs a _small_ runtime penalty as a result.

The api is currently limited to resource access, but it will be extended to queries / entity component access in the future.

### Resource Scopes

<div class="release-feature-authors">authors: @cart</div>

WorldCell does not yet support component queries, and even when it does there will sometimes be legitimate reasons to want a mutable world ref _and_ a mutable resource ref (ex: bevy_render and bevy_scene both need this). In these cases we could always drop down to the unsafe `world.get_resource_unchecked_mut()`, but that is not ideal!

Instead developers can use a "resource scope"

```rust
world.resource_scope(|world: &mut World, mut a: Mut<A>| {
})
```

This temporarily removes the `A` resource from `World`, provides mutable pointers to both, and re-adds A to World when finished. Thanks to the move to ComponentIds/sparse sets, this is a cheap operation.

If multiple resources are required, scopes can be nested. We could also consider adding a "resource tuple" to the api if this pattern becomes common and the boilerplate gets nasty.

### Query Conflicts Use ComponentId Instead of ArchetypeComponentId

<div class="release-feature-authors">authors: @cart</div>

For safety reasons, systems cannot contain queries that conflict with each other without wrapping them in a `QuerySet`. In **Bevy 0.4**, we used `ArchetypeComponentIds` to determine conflicts. This was nice because it could take into account filters:

```rust
// these queries will never conflict due to their filters
fn filter_system(a: Query<&mut A, With<B>>, b: Query<&mut B, Without<B>>) {
}
```

But it also had a significant downside:

```rust
// these queries will not conflict _until_ an entity with A, B, and C is spawned
fn maybe_conflicts_system(a: Query<(&mut A, &C)>, b: Query<(&mut A, &B)>) {
}
```

The system above will panic at runtime if an entity with A, B, and C is spawned. This makes it hard to trust that your game logic will run without crashing.

In **Bevy 0.5**, we switched to using `ComponentId` instead of `ArchetypeComponentId`. This _is_ more constraining. `maybe_conflicts_system` will now always fail, but it will do it consistently at startup.

Naively, it would also _disallow_ `filter_system`, which would be a significant downgrade in usability. Bevy has a number of internal systems that rely on disjoint queries and we expect it to be a common pattern in userspace. To resolve this, we added a new internal `FilteredAccess<T>` type, which wraps `Access<T>` and adds with/without filters. If two `FilteredAccess` have with/without values that prove they are disjoint, they will no longer conflict.

This means `filter_system` is still perfectly valid in **Bevy 0.5**. We get most of the benefits of the old implementation, but with consistent and predictable rules enforced at app startup.

## What's Next For Bevy?

We still have a long road ahead of us, but the Bevy developer community is growing at a rapid pace and we already have big plans for the future. Expect to see progress in the following areas soon:

* "Pipelined" rendering and other renderer optimizations
* Bevy UI redesign
* Animation: component animation and 3d skeletal animation
* ECS: relationships/indexing, async systems, archetype invariants, "stageless" system schedules
* 3D Lighting Features: shadows, more light types
* More Bevy Scene features and usability improvements

We also plan on breaking ground on the Bevy Editor as soon as we converge on a final Bevy UI design.

## Support Bevy

[Sponsorships](https://github.com/sponsors/cart) help make full time work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring @cart](https://github.com/sponsors/cart) ... every bit helps!

<a class="button button--pink header__cta" href="https://github.com/sponsors/cart">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"/></a>

## Contributors

A huge thanks to the **88 contributors** that made this release (and associated docs) possible!

* mockersf
* CAD97
* willcrichton
* Toniman20
* ElArtista
* lassade
* Divoolej
* msklywenn
* cart
* maxwellodri
* schell
* payload
* guimcaballero
* themilkybit
* Davier
* TheRawMeatball
* alexschrod
* Ixentus
* undinococo
* zicklag
* lambdagolem
* reidbhuntley
* enfipy
* CleanCut
* LukeDowell
* IngmarBitter
* MinerSebas
* ColonisationCaptain
* tigregalis
* siler
* Lythenas
* Restioson
* kokounet
* ryanleecode
* adam-bates
* Neo-Zhixing
* bgourlie
* Telzhaak
* rkr35
* jamadazi
* bjorn3
* VasanthakumarV
* turboMaCk
* YohDeadfall
* rmsc
* szunami
* mnmaita
* WilliamTCarroll
* Ratysz
* OptimisticPeach
* mtsr
* AngelicosPhosphoros
* Adamaq01
* Moxinilian
* tomekr
* jakobhellermann
* sdfgeoff
* Byteron
* aevyrie
* verzuz
* ndarilek
* huhlig
* zaszi
* Puciek
* DJMcNab
* sburris0
* rparrett
* smokku
* TehPers
* alec-deason
* Fishrock123
* woubuc
* Newbytee
* Archina
* StarArawn
* JCapucho
* M2WZ
* TotalKrill
* refnil
* bitshifter
* NiklasEi
* alice-i-cecile
* joshuajbouw
* DivineGod
* ShadowMitia
* memoryruins
* blunted2night
* RedlineTriad

## Change Log

### Added

* [PBR Rendering][1554]
* [PBR Textures][1632]
* [HIDPI Text][1132]
* [Rich text][1245]
* [Wireframe Rendering Pipeline][562]
* [Render Layers][1209]
* [Add Sprite Flipping][1407]
* [OrthographicProjection scaling mode + camera bundle refactoring][400]
* [3D OrthographicProjection improvements + new example][1361]
* [Flexible camera bindings][1689]
* [Render text in 2D scenes][1122]
* [Text2d render quality][1171]
* [System sets and run criteria v2][1675]
* [System sets and parallel executor v2][1144]
* [Many-to-many system labels][1576]
* [Non-string labels (#1423 continued)][1473]
* [Make EventReader a SystemParam][1244]
* [Add EventWriter][1575]
* [Reliable change detection][1471]
* [Redo State architecture][1424]
* [Query::get_unique][1263]
* [gltf: load normal and occlusion as linear textures][1762]
* [Add separate brightness field to AmbientLight][1605]
* [world coords to screen space][1258]
* [Experimental Frustum Culling (for Sprites)][1492]
* [Enable wgpu device limits][1544]
* [bevy_render: add torus and capsule shape][1223]
* [New mesh attribute: color][1194]
* [Minimal change to support instanced rendering][1262]
* [Add support for reading from mapped buffers][1274]
* [Texture atlas format and conversion][1365]
* [enable wgpu device features][547]
* [Subpixel text positioning][1196]
* [make more information available from loaded GLTF model][1020]
* [use Name on node when loading a gltf file][1183]
* [GLTF loader: support mipmap filters][1639]
* [Add support for gltf::Material::unlit][1341]
* [Implement Reflect for tuples up to length 12][1218]
* [Process Asset File Extensions With Multiple Dots][1277]
* [Update Scene Example to Use scn.ron File][1339]
* [3d game example][1252]
* [Add keyboard modifier example (#1656)][1657]
* [Count number of times a repeating Timer wraps around in a tick][1112]
* [recycle `Timer` refactor to duration.sparkles Add `Stopwatch` struct.][1151]
* [add scene instance entity iteration][1058]
* [Make Commands and World apis consistent][1703]
* [Add `insert_children` and `push_children` to EntityMut][1728]
* [Extend AppBuilder api with `add_system_set` and similar methods][1453]
* [add labels and ordering for transform and parent systems in POST_UPDATE stage][1456]
* [Explicit execution order ambiguities API][1469]
* [Resolve (most) internal system ambiguities][1606]
* [Change 'components' to 'bundles' where it makes sense semantically][1257]
* [add `Flags<T>` as a query to get flags of component][1172]
* [Rename add_resource to insert_resource][1356]
* [Update init_resource to not overwrite][1349]
* [Enable dynamic mutable access to component data][1284]
* [Get rid of ChangedRes][1313]
* [impl SystemParam for Option<Res<T>> / Option<ResMut<T>>][1494]
* [Add Window Resize Constraints][1409]
* [Add basic file drag and drop support][1096]
* [Modify Derive to allow unit structs for RenderResources.][1089]
* [bevy_render: load .spv assets][1104]
* [Expose wgpu backend in WgpuOptions and allow it to be configured from the environment][1042]
* [updates on diagnostics (log + new diagnostics)][1085]
* [enable change detection for labels][1155]
* [Name component with fast comparisons][1109]
* [Support for !Send tasks][1216]
* [Add missing spawn_local method to Scope in the single threaded executor case][1266]
* [Add bmp as a supported texture format][1081]
* [Add an alternative winit runner that can be started when not on the main thread][1063]
* [Added use_dpi setting to WindowDescriptor][1131]
* [Implement Copy for ElementState][1154]
* [Mutable mesh accessors: indices_mut and attribute_mut][1164]
* [Add support for OTF fonts][1200]
* [Add `from_xyz` to `Transform`][1212]
* [Adding copy_texture_to_buffer and copy_texture_to_texture][1236]
* [Added `set_minimized` and `set_position` to `Window`][1292]
* [Example for 2D Frustum Culling][1503]
* [Add remove resource to commands][1478]

### Changed

* [Bevy ECS V2][1525]
* [Fix Reflect serialization of tuple structs][1366]
* [color spaces and representation][1572]
* [Make vertex buffers optional][1485]
* [add to lower case to make asset loading case insensitive][1427]
* [Replace right/up/forward and counter parts with local_x/local_y and local_z][1476]
* [Use valid keys to initialize AHasher in FixedState][1268]
* [Change Name to take Into<String> instead of String][1283]
* [Update to wgpu-rs 0.7][542]
* [Update glam to 0.13.0.][1550]
* [use std clamp instead of Bevy's][1644]
* [Make Reflect impls unsafe (Reflect::any must return `self`)][1679]

### Fixed

* [convert grayscale images to rgb][1524]
* [Glb textures should use bevy_render to load images][1454]
* [Don't panic on error when loading assets][1286]
* [Prevent ImageBundles from causing constant layout recalculations][1299]
* [do not check for focus until cursor position has been set][1070]
* [Fix lock order to remove the chance of deadlock][1121]
* [Prevent double panic in the Drop of TaksPoolInner][1064]
* [Ignore events when receiving unknown WindowId][1072]
* [Fix potential bug when using multiple lights.][1055]
* [remove panics when mixing UI and non UI entities in hierarchy][1180]
* [fix label to load gltf scene][1204]
* [fix repeated gamepad events][1221]
* [Fix iOS touch location][1224]
* [Don't panic if there's no index buffer and call draw][1229]
* [Fix Bug in Asset Server Error Message Formatter][1340]
* [add_stage now checks Stage existence][1346]
* [Fix Un-Renamed add_resource Compile Error][1357]
* [Fix Interaction not resetting to None sometimes][1315]
* [Fix regression causing "flipped" sprites to be invisible][1399]
* [revert default vsync mode to Fifo][1416]
* [Fix missing paths in ECS SystemParam derive macro][1434]
* [Fix staging buffer required size calculation (fixes #1056)][1509]

[400]: https://github.com/bevyengine/bevy/pull/400
[542]: https://github.com/bevyengine/bevy/pull/542
[547]: https://github.com/bevyengine/bevy/pull/547
[562]: https://github.com/bevyengine/bevy/pull/562
[1020]: https://github.com/bevyengine/bevy/pull/1020
[1042]: https://github.com/bevyengine/bevy/pull/1042
[1055]: https://github.com/bevyengine/bevy/pull/1055
[1058]: https://github.com/bevyengine/bevy/pull/1058
[1063]: https://github.com/bevyengine/bevy/pull/1063
[1064]: https://github.com/bevyengine/bevy/pull/1064
[1070]: https://github.com/bevyengine/bevy/pull/1070
[1072]: https://github.com/bevyengine/bevy/pull/1072
[1081]: https://github.com/bevyengine/bevy/pull/1081
[1085]: https://github.com/bevyengine/bevy/pull/1085
[1089]: https://github.com/bevyengine/bevy/pull/1089
[1096]: https://github.com/bevyengine/bevy/pull/1096
[1104]: https://github.com/bevyengine/bevy/pull/1104
[1109]: https://github.com/bevyengine/bevy/pull/1109
[1112]: https://github.com/bevyengine/bevy/pull/1112
[1121]: https://github.com/bevyengine/bevy/pull/1121
[1122]: https://github.com/bevyengine/bevy/pull/1122
[1131]: https://github.com/bevyengine/bevy/pull/1131
[1132]: https://github.com/bevyengine/bevy/pull/1132
[1144]: https://github.com/bevyengine/bevy/pull/1144
[1151]: https://github.com/bevyengine/bevy/pull/1151
[1154]: https://github.com/bevyengine/bevy/pull/1154
[1155]: https://github.com/bevyengine/bevy/pull/1155
[1164]: https://github.com/bevyengine/bevy/pull/1164
[1171]: https://github.com/bevyengine/bevy/pull/1171
[1172]: https://github.com/bevyengine/bevy/pull/1172
[1180]: https://github.com/bevyengine/bevy/pull/1180
[1183]: https://github.com/bevyengine/bevy/pull/1183
[1194]: https://github.com/bevyengine/bevy/pull/1194
[1196]: https://github.com/bevyengine/bevy/pull/1196
[1200]: https://github.com/bevyengine/bevy/pull/1200
[1204]: https://github.com/bevyengine/bevy/pull/1204
[1209]: https://github.com/bevyengine/bevy/pull/1209
[1212]: https://github.com/bevyengine/bevy/pull/1212
[1216]: https://github.com/bevyengine/bevy/pull/1216
[1218]: https://github.com/bevyengine/bevy/pull/1218
[1221]: https://github.com/bevyengine/bevy/pull/1221
[1223]: https://github.com/bevyengine/bevy/pull/1223
[1224]: https://github.com/bevyengine/bevy/pull/1224
[1229]: https://github.com/bevyengine/bevy/pull/1229
[1236]: https://github.com/bevyengine/bevy/pull/1236
[1244]: https://github.com/bevyengine/bevy/pull/1244
[1245]: https://github.com/bevyengine/bevy/pull/1245
[1252]: https://github.com/bevyengine/bevy/pull/1252
[1257]: https://github.com/bevyengine/bevy/pull/1257
[1258]: https://github.com/bevyengine/bevy/pull/1258
[1262]: https://github.com/bevyengine/bevy/pull/1262
[1263]: https://github.com/bevyengine/bevy/pull/1263
[1266]: https://github.com/bevyengine/bevy/pull/1266
[1268]: https://github.com/bevyengine/bevy/pull/1268
[1274]: https://github.com/bevyengine/bevy/pull/1274
[1277]: https://github.com/bevyengine/bevy/pull/1277
[1283]: https://github.com/bevyengine/bevy/pull/1283
[1284]: https://github.com/bevyengine/bevy/pull/1284
[1286]: https://github.com/bevyengine/bevy/pull/1286
[1292]: https://github.com/bevyengine/bevy/pull/1292
[1299]: https://github.com/bevyengine/bevy/pull/1299
[1313]: https://github.com/bevyengine/bevy/pull/1313
[1315]: https://github.com/bevyengine/bevy/pull/1315
[1339]: https://github.com/bevyengine/bevy/pull/1339
[1340]: https://github.com/bevyengine/bevy/pull/1340
[1341]: https://github.com/bevyengine/bevy/pull/1341
[1346]: https://github.com/bevyengine/bevy/pull/1346
[1349]: https://github.com/bevyengine/bevy/pull/1349
[1356]: https://github.com/bevyengine/bevy/pull/1356
[1357]: https://github.com/bevyengine/bevy/pull/1357
[1361]: https://github.com/bevyengine/bevy/pull/1361
[1365]: https://github.com/bevyengine/bevy/pull/1365
[1366]: https://github.com/bevyengine/bevy/pull/1366
[1399]: https://github.com/bevyengine/bevy/pull/1399
[1407]: https://github.com/bevyengine/bevy/pull/1407
[1409]: https://github.com/bevyengine/bevy/pull/1409
[1416]: https://github.com/bevyengine/bevy/pull/1416
[1424]: https://github.com/bevyengine/bevy/pull/1424
[1427]: https://github.com/bevyengine/bevy/pull/1427
[1434]: https://github.com/bevyengine/bevy/pull/1434
[1453]: https://github.com/bevyengine/bevy/pull/1453
[1454]: https://github.com/bevyengine/bevy/pull/1454
[1456]: https://github.com/bevyengine/bevy/pull/1456
[1469]: https://github.com/bevyengine/bevy/pull/1469
[1471]: https://github.com/bevyengine/bevy/pull/1471
[1473]: https://github.com/bevyengine/bevy/pull/1473
[1476]: https://github.com/bevyengine/bevy/pull/1476
[1478]: https://github.com/bevyengine/bevy/pull/1478
[1485]: https://github.com/bevyengine/bevy/pull/1485
[1492]: https://github.com/bevyengine/bevy/pull/1492
[1494]: https://github.com/bevyengine/bevy/pull/1494
[1503]: https://github.com/bevyengine/bevy/pull/1503
[1509]: https://github.com/bevyengine/bevy/pull/1509
[1524]: https://github.com/bevyengine/bevy/pull/1524
[1525]: https://github.com/bevyengine/bevy/pull/1525
[1544]: https://github.com/bevyengine/bevy/pull/1544
[1550]: https://github.com/bevyengine/bevy/pull/1550
[1554]: https://github.com/bevyengine/bevy/pull/1554
[1572]: https://github.com/bevyengine/bevy/pull/1572
[1575]: https://github.com/bevyengine/bevy/pull/1575
[1576]: https://github.com/bevyengine/bevy/pull/1576
[1605]: https://github.com/bevyengine/bevy/pull/1605
[1606]: https://github.com/bevyengine/bevy/pull/1606
[1632]: https://github.com/bevyengine/bevy/pull/1632
[1639]: https://github.com/bevyengine/bevy/pull/1639
[1644]: https://github.com/bevyengine/bevy/pull/1644
[1657]: https://github.com/bevyengine/bevy/pull/1657
[1675]: https://github.com/bevyengine/bevy/pull/1675
[1679]: https://github.com/bevyengine/bevy/pull/1679
[1689]: https://github.com/bevyengine/bevy/pull/1689
[1703]: https://github.com/bevyengine/bevy/pull/1703
[1728]: https://github.com/bevyengine/bevy/pull/1728
[1762]: https://github.com/bevyengine/bevy/pull/1762
