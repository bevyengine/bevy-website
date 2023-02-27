+++
title = "Bevy 0.10"
date = 2023-03-04
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), I'm happy to announce the **Bevy 0.10** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.10**, check out our [0.9 to 0.10 Migration Guide](/learn/book/migration-guides/0.9-0.10/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Simpler, more flexible scheduling**: systems are now stored in a unified schedule, commands can be applied explicitly via `apply_system_buffers` and a whole lot of quality of life and bug fixes.

## Simpler, more flexible scheduling

<div class="release-feature-authors">authors: @alice-i-cecile, @maniwani, @WrongShoe, @jakobhellermann, @JoJoJet, @geieredgar and a whole lot more </div>

Thanks to the fantastic work of our ECS team, the hotly awaited ["stageless" scheduling RFC](https://github.com/bevyengine/rfcs/blob/main/rfcs/45-stageless.md) has been implemented! But as we all know, plans and implementations (start at [#6587](https://github.com/bevyengine/bevy/pull/6587) by `@maniwani` and [#7267](https://github.com/bevyengine/bevy/pull/7267) by `@alice-i-cecile`) are two different things. Let's take a look at what actually shipped for 0.10.

There's been a lot of changes, but we really do think that ripping off the band-aid now (before any form of stability guarantees) is essential to the health of Bevy's scheduling model going forward.

The [migration path](../../learn/book/migration-guides/0.9-0.10/_index.md) for existing applications won't be trivial, but we've done our best to keep it surprisingly straightforward. Don't sweat it!

## A Single Unified Schedule

Ever wanted to specify that `system_a` runs before `system_b`, only to be met with confusing warnings that `system_b` isn't found because it's in a different stage?

No more! All systems within a single **schedule** are now stored in a single data structure with a global awareness of what's going on.

This simplifies our internal logic, makes your code more robust to refactoring, and allows plugin authors to specify high-level invariants (e.g. "movement must occur before collision checking") without locking themselves in to an exact schedule location.

[!main_schedule_diagram](main_schedule_diagram.svg)

This diagram, made with [@jakobhellermann's `bevy_mod_debugdump` crate](https://github.com/jakobhellermann/bevy_mod_debugdump) shows a simplified version of Bevy's default schedule.

## Configurable System Sets

To support more natural and flexible control over "how are my systems run and scheduled", the idea of a "system set" has been redefined, rolling up the existing "system label" concept into one straightforward but powerful abstraction.

**System sets** are named collections of systems that share a set of **system configuration**: if there are run conditions attached, how they are ordered relative to other systems or sets and so on. This is distributive: Ordering systems relative to a system set applies that ordering to _all_ systems in that set.

Let's jump right in to what this would look like.

```rust
// System set types are used to provide stable, typed identifiers
// for groups of systems, allowing external systems to order themselves
// without being aware of internal details.
// Each variant of this enum is a distinct system set.
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum PhysicsSet {
    Forces,
    Kinematics,
    CollisionDetection
}

app
    // .with_run_criteria -> .run_if <3
   .add_system(gravity.in_set(PhysicsSet::Forces).run_if(gravity_enabled))
    // Add multiple systems at once with add_systems!    
    .add_systems((
        apply_acceleration,
        apply_velocity
    // Quickly order a list of systems with .chain()
    ).chain().in_set(PhysicsSet::Kinematics))
    .add_system(detect_collisions.in_set(PhysicsSet::CollisionDetection))
    // You can add configuration for an entire set in a single place
    .configure_set(
        PhysicSet::Forces
        .in_base_set(CoreSet::Update)
        .before(PhysicsSet::Kinematics)
    )
    .configure_set(
        PhysicSet::Kinematics
        // Look ma, I can order systems across command flushes
        .in_base_set(CoreSet::PostUpdate)
        .before(PhysicsSet::CollisionDetection)
        // Ooh run condition combinators :eyes:
        .run_if(not(game_paused))
    )
    .configure_set(
        PhysicSet::CollisionDetection
        .in_base_set(CoreSet::PostUpdate)
    )
```

A system can belong to any number of sets, adding the configuration from each set it belongs to to its own configuration. Similarly, sets can be nested, allowing you to granularly define a clear set of rules for app-level scheduling patterns.

These rules must be compatible with each other: any paradoxes (like a system set inside of itself, or a system that must run both before and after a set) will result in a runtime panic with a helpful error message.

As long as you can construct the type of a system set, you can both order your systems relative to it, and configure its behavior even after it has been initialized elswhere! Crucially system configuration is strictly additive: you cannot _remove_ rules added elsewhere. This is both a "anti-spaghetti" and "plugin privacy" consideration. When this rule is combined with Rust's robust type privacy rules, plugin authors can make careful decisions about which exact invariants need to be upheld, and reorganize code and systems internally without breaking consumers.

Plugin authors: consider offering both a "default configuration" and a "minimal configuration" version of your plugins to support more unusual scheduling patterns while keeping that "it just works" behavior you've come to love.

## Directly Schedule Exclusive Systems

Ever wished that you could just flush commands or run an exclusive system right before this system but after that system without shuffling your entire schedule to make it work?

Now you can! Thanks to ongoing cleanup work in the ECS scheduling internals, and the unified schedule mentioned above, exclusive systems can now be scheduled and ordered like any other system.

```rust
app
    .add_system(ordinary_system)
    // This works?!
    .add_system(exclusive_system.after(ordinary_system))
```

This is particularly powerful, as **command flushes** (which apply any queued up `Commands` added in systems to e.g. spawn and despawn entities) are now simply performed in the `apply_system_buffers` exclusive system.

```rust
app
    .add_systems((
        system_that_produces_commands,
        // Built-in exclusive system that applies generated commands
        apply_system_buffers,
        system_that_needs_commands
    // chain() creates an ordering between each of these systems,
    // so we know that our commands will be ready in time
    ).chain().in_set(CoreSet::PostUpdate))
```

Do be careful with this pattern though: it's easy to quickly end up with many poorly ordered exclusive systems, creating bottlenecks and chaos.

Similarly, state transitions can be scheduled manually, one type at a time, in the `apply_state_transitions::<S>` exclusive system.

What will you do with this much power? We're keen to find out!

## It's All Schedules? Managing complex control flow

But what if you want to do something _weird_ with your schedule. Something non-linear, or branching, or looping. What should you reach for?

It turns out, Bevy already _had_ a great tool for this: schedules run inside of an exclusive system. The idea is pretty simple:

1. Construct a schedule, that stores whatever complex logic you want to run.
2. Store that schedule inside of a resource.
3. In an exclusive system, perform any arbitrary Rust logic you want to decide if and how your schedule runs.
4. Temporarily take the schedule out of the world, run it on the rest of the world to mutate both the schedule and the world, and then put it back in.

With the addition of the new `Schedules` resource and the `world.run_schedule(schedule_label: impl ScheduleLabel)`API it's more :sparkles: ergonomic :sparkles: than ever.

```rust!

// A schedule!
let mut my_schedule = Schedule::new();
schedule.add_system(my_system);

// A schedule label for it
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct MySchedule;

// An exclusive system to run this schedule
fn run_my_schedule(world: &mut World) {
    while very_complex_logic(){
        world.run_schedule(MySchedule);
    }
}

// Behold the ergonomics
app
    .add_schedule(MySchedule, my_schedule)
    .add_system(run_my_schedule);
```

Bevy uses this pattern for five rather different things at 0.10 release:

1. **Startup systems:** these now live in their own schedule, which is run once at the start of the app.
2. **Fixed timestep systems:** another schedule?! The exclusive system that runs this schedule accumulates time, running a while loop that repeatedly runs `CoreSchedule::FixedTimestep` until all of the accumulated time has been spent.
3. **Entering and exiting states:** a bonanza of schedules. Each collection of systems that runs logic to enter and exit a state variant is stored in its own schedule, which are called based on the change in state in the `apply_state_transitions::<S>` exclusive system.
4. **Rendering:** all rendering logic is stored in its own schedule to allow it to run asynchronously relative to gameplay logic.
5. **Controlling the outermost loop:** in order to handle the "startup schedule first, then main schedule" logic, we wrap it all up in a minimal overhead `CoreSchedule::Outer` and then run our schedules as the sole exclusive system there.

Follow the bread crumbs starting at [`CoreSchedule`](https://dev-docs.bevyengine.org/bevy/app/enum.CoreSchedule.html) for more info.

## Simpler Run Conditions

Systems may have any number of run conditions (and inherit them from the sets they belong to), but will only run if all of their run conditions return `true`.
Run criteria have been renamed to the clearer **run conditions**, which can be constructed out of any read-only system that returns `bool`.

With a new blessed pattern for complex control flow, we can finally get rid of looping run criteria. [`ShouldRun::YesAndCheckAgain`](https://docs.rs/bevy/0.9.1/bevy/ecs/schedule/enum.ShouldRun.html) was not exactly straightforward to reason about, either for engine devs or users. It's always a bad sign when your bool-like enums have four possible values.

If you crave that powerful, complex control flow: use the "schedules in exclusive systems" pattern listed above.
For the other 99% of use cases, enjoy the simpler `bool`-based run conditions.

```rust!
// Let's make our own run condition
fn contrived_run_condition(query: Query<&Life, With<Player>>, score: Res<Score>) -> bool{
    let player_life = query.single();
    
    if score.0 * player_life > 9000 {
        true
    }
}

app.add_system(win_game.run_if(contrived_run_condition));
```

Run conditions can serve as a lightweight optimization tool: each one is evaluated only each schedule update, and shared across the system set. Reducing the number of tasks spawned can really add up. Like always though: benchmark!

Bevy 0.10 is shipping with a lovely collection of built-in [common run conditions](https://dev-docs.bevyengine.org/bevy/ecs/schedule/common_conditions/index.html). Courtesy of [#6587 by `@maniwani`](https://github.com/bevyengine/bevy/pull/6587), [#7579 by `@inodentry`](https://github.com/bevyengine/bevy/pull/7579)and [#7806 by `@jakobhellermann`](https://github.com/bevyengine/bevy/pull/7806), you can quickly check if there are events to process, changes to resources, input states and more.

When you need something more sophisticated, combining run conditions is a breeze. Courtesy of [#7547](https://github.com/bevyengine/bevy/pull/7547), [#7559](https://github.com/bevyengine/bevy/pull/7559), and [#7605](https://github.com/bevyengine/bevy/pull/7605), you can create new run conditions with the use of system piping and the `not`, `and_then` or `or_else` run criteria combinators.

## Simpler States

Of course, looping run criteria were used to power states.
How do they work in Bevy 0.10?

1. The current value of the state of type `S` is stored in the `State<S: States>` resource. The pending value is stored in `NextState<S: States>`.
    1. To set the next state, simply mutate the value of the `NextState<S>` resource.
2. Run conditions can read the value of the `State<S>` resource.
    1. Systems with the `in_state(AppState::InGame)` run condition will only run if the value of the `State<AppState>` resource equals `AppState::InGame`.
3. Check for and apply state transitions as part of the `apply_state_transitions<S>` exclusive system. When transitioning between states:
    1. First run the `OnExit(S::VariantLeft)` schedule for the state you're leaving.
    2. Then run the `OnEnter(S::VariantEntered)` schedule.
    3. These schedules are stored in the `Schedules` resource, and can be looked up via their `ScheduleLabel`.
4. When the user calls `app.add_state:<s>()`:
    1. Initialize an `OnEnter` and an `OnExit` schedule for each variant of our state type `S`.
    2. Configure the `OnUpdate(S::Variant)` system set to belong to `CoreSet::Update` and only run when `State<S>` is `S::Variant`.
    3. Add a copy of `apply_state_transitions<S>` to `CoreSet::ApplyStateTransitions`.
    4. Set the starting state of `S` using its `Default` trait.

As a user though, you don't have to worry about those details:

```rust!
// Setting up our state type.
// Note that each variant of this enum is a distinct state.
#[derive(States, PartialEq, Eq, Debug, Default)]
enum AppState {
    InGame,
    #[default]
    MainMenu
}

app
    // Don't forget to initialize the state!
    .add_state::<AppState>()
    .add_system(load_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
    .add_system(start_game.in_set(OnUpdate(AppState::MainMenu)))
    .add_system(cleanup_main_menu.in_schedule(OnExit(AppState::MainMenu)))
    .add_system(make_game_fun.in_set(OnUpdate(AppState::InGame)));

fn start_game(
    button_query: Query<&Interaction, With<StartGameButton>>,
    next_state: ResMut<NextState<AppState>>,
){
    let start_game_interaction_state = button_query.single();
    if start_game_interaction_state == Interaction::Pressed {
        *next_state = NextState(AppState::InGame);
    }
}
```

But wait you say: what about my state stack? My elaborate queued transitions?! My meticulous error handling on every operation that I definitely didn't just unwrap?!!

In practice, we found that the state stack was a) very complex to learn b) very prone to exasperating bugs c) mostly ignored.
As a result, states are now "stackless": only one queued state of each type at a time.

Thanks to the help of some brave alpha testers, we're reasonably confident that this shouldn't be too bad to migrate away from.
If you were relying on the state stack, you might choose to:

* rearchitect some of that logic out of states
* use additional state types, which capture orthogonal elements of your app's status
* build your own state stack abstraction using the same patterns as Bevy's first-party version: please let the rest of the community know so you can collaborate!

## Base Sets: Getting Default Behavior Right

Of course the skeptical reader may point out that:

1. Bevy automatically runs its systems in parallel.
2. [The order of systems is nondeterministic unless there is an explicit ordering relationship between them](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/nondeterministic_system_order.rs)?
3. All of the systems are now stored in a single `Schedule` object with no barriers between them?
4. Systems can belong to any number of system sets, each of which can add their own behavior?

Won't this lead to utter chaos and tedious spaghetti-flavored work to resolve every last ordering ambiguity?
Many users _liked_ stages, they were helpful for understanding the structure of my app!

Well, I'm glad you asked, rhetorical skeptic. To reduce this chaos (and ease migration), Bevy 0.10 comes with a brand new collection of system sets with the default plugins: [`CoreSet`](https://dev-docs.bevyengine.org/bevy/app/enum.CoreSet.html), [`StartupSet`](https://dev-docs.bevyengine.org/bevy/app/enum.StartupSet.html) and [`RenderSet`](https://dev-docs.bevyengine.org/bevy/render/enum.RenderSet.html). The similarity of their names to [`CoreStage`](https://docs.rs/bevy/0.9.1/bevy/app/enum.CoreStage.html), [`StartupStage`](https://docs.rs/bevy/0.9.1/bevy/app/enum.StartupStage.html) and [`RenderStage`](https://docs.rs/bevy/0.9.1/bevy/render/enum.RenderStage.html) is not a coincidence: there are command flush points between each set, and existing systems have been migrated directly.

Some parts of the stage-centric architecture were appealing: a clear high level structure, coordination on flush points (to reduce excessive bottlenecks) and good default behavior.
To keep those bits (while excising the frustrating ones), we've introduced the concept of **base sets**, added in [#7466](https://github.com/bevyengine/bevy/pull/7466) by `@cart`. Base sets are system sets, except:

1. Every system (but not every system set) must belong to exactly one base set.
2. Systems that do not specify a base set will be added to the default base set for the schedule.

```rust
// You can add new base sets to any built-in ones
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum MyBaseSet {
    Early,
    Late,
}

app
    // This ends up in CoreSet::Update by default
    .add_system(no_base_set)
    // You must use .in_base_set rather than .in_set for explicitness
    // This is a high-impact decision!
    .add_system(post_update.in_base_set(CoreSet::PostUpdate))
    // Look, it works!
    .add_system(custom_base_set.in_base_set(MyBaseSet::Early))
    // Ordering your base sets relative to CoreSet is probably wise
    .configure_set(MyBaseSet::Early.before(CoreSet::Update))
    .configure_set(MyBaseSet::Late.after(CoreSet::Update));
```

Pretty simple, but what does this buy us?
First, it gives you a clear hook to impose, reason about and visualize high level structure to your schedule. Yearning for a linear, stage-like design? Just order your base sets!
Secondly, it allows Bevy to set good default behavior for systems added by users, without removing their control.

Let me tell you a story, set in a world where all of Mr. Straw Man's points above are true, and no default set is added.

1. A new user adds the `make_player_run` system to their app.
2. Sometimes this system runs before input handling, leading to randomly dropped inputs. Sometimes it runs after rendering, leading to stranges flickers.
3. After much frustration, the user discovers that these are due to "system execution order ambiguities".
4. The user runs a specialized tool, digs into the source code of the engine, figures out what order their system should run in relative to the engine's system sets, and then continues on their merry way, doing this for each new system.
5. Bevy (or one of their third-party plugins) updates, breaking all of our poor users system ordering once again.

In practice, there are three broad classes of systems: gameplay logic (the majority of all end user systems), stuff that needs to happen before gameplay logic (like event cleanup and input handling) and stuff that needs to happen after gameplay logic (like rendering and audio).

By broadly ordering the schedule via base sets, we hope that Bevy apps can have good default behavior and clear high level structure without compromising on the scheduling flexibility and explicitness that advanced users crave.
Let us know how it works out for you!

## Polish Matters

As part of this work, we've taken the time to listen to our users and fix some small but high-impact things about how scheduling works.

Compare the following options for adding and ordering four systems, one after the other.

:coffee: **Enterprise-grade** :coffee::

```rust
#[derive(SystemSet, PartialEq, Eq, Clone, Copy, Hash, Debug)]
#[allow(missing_docs)]
pub enum Step {
    A,
    B,
    C,
    D
}

app
    .configure_set(Step::A.before(Step::B))
    .configure_set(Step::B.before(Step::C))
    .configure_set(Step::C.before(Step::D))
    .add_system(a.in_set(Step::A))
    .add_system(b.in_set(Step::B))
    .add_system(c.in_set(Step::C))
    .add_system(d.in_set(Step::D));
```

:weary: **Tedious** :weary::

```rust
app
    .add_system(a.before(b))
    .add_system(b.before(c))
    .add_system(c.before(d))
    .add_system(d);
```

:sparkles: **Ergonomic** :sparkles::

```rust
    app.add_systems((a, b, c, d).chain());
```

There's another lovely change lurking in that last example: the `add_systems` API.

Bevy 0.9:

```rust
app
    .add_system_set(SystemSet::on_update(AppState::InGame)
        .with_system(a.before(b))
        .with_system(b.label(MyLabel::Variant))
        .with_system(c)
        .with_run_criteria(blue_moon)
    )    

```

Bevy 0.10:

```rust
app.add_systems(
    (
        a.before(b),
        b.in_set(MySet::Variant),
        c
    )
    .run_if(blue_moon)
    .in_set(OnUpdate(AppState::InGame))
)
```

We've also:

* added trivial single threaded evaluation via the [`SingleThreadedExecutor`](https://dev-docs.bevyengine.org/bevy/ecs/schedule/struct.SingleThreadedExecutor.html) for users who prefer alternate parallelization strategies (or simply don't need it) by `@maniwani` as part of the `bevy_ecs::schedule` rewrite
  * we already default to this on WASM, so don't worry about setting it up for your jam games!
  * wish commands just applied instantly? We've got you: use [`SimpleExecutor`](https://dev-docs.bevyengine.org/bevy/ecs/schedule/struct.SimpleExecutor.html) and trade performance for clarity and convenience to your heart's content.
* added ultra-convenient prebuilt error-handling system piping adaptors in [#6751 by `@edwox`](https://github.com/bevyengine/bevy/pull/6751) so you can quickly and easily use the `?` operator in your Bevy systems and log any failure cases
  * Put an end to the rightward drift: just use `.add_system(fallible_system.pipe(system_adaptor::warn)))` :heart_eyes:
* removed string-based labels: these were prone to nasty conflicts, easy to typo, didn't play nice with IDEs and are no longer needed due to the much improved ergonomics of ordering systems in other forms
* made sure you can pipe data into and out of exclusive systems in [#6698 by `@inodentry`](https://github.com/bevyengine/bevy/pull/6698)
* significantly improved ambiguity detection and cycle reporting: check out the [`ScheduleBuildSettings`](https://dev-docs.bevyengine.org/bevy/ecs/schedule/struct.ScheduleBuildSettings.html) docs for more info. If you haven't tried this out on your app yet: you should take a look!

The Bevy ECS team has worked closely with `@jakobhellerman`, the author of [`bevy_mod_debugdump`](https://crates.io/crates/bevy_mod_debugdump), the leading third-party schedule visualization plugin, to ensure it keeps working better than ever.

It's a great tool that we are looking to build on to create a first party solution: you should strongly consider adding it to your toolbox.

## What's Next?

* **[One-shot systems](https://github.com/bevyengine/bevy/issues/2192):** Run arbitrary systems in a push-based fashion via commands, and store them as callback components for ultra-flexible behavior customization.
* **Better plugins:** Clearer and more standardized tools for [adapting third-party plugins to your app's unique architecture](https://github.com/bevyengine/bevy/issues/2160), eliminating [order-dependence in their initialization](https://github.com/bevyengine/bevy/issues/1255) and defining [dependencies](https://github.com/bevyengine/bevy/issues/69) between them.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](Bevy ) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

A huge thanks to the **X contributors** that made this release (and associated docs) possible! In random order:

* @Foo

## Full Change Log
