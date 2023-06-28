+++
title = "Bevy 0.11"
date = 2023-07-07
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.11** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.11**, check out our [0.10 to 0.11 Migration Guide](/learn/migration-guides/0.10-0.11/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Feature**: description

## Schedule-First ECS APIs

<div class="release-feature-authors">authors: @cart</div>

In **Bevy 0.10** we introduced [ECS Schedule V3](/news/bevy-0-10/#ecs-schedule-v3), which _vastly_ improved the capabilities of Bevy ECS system scheduling: scheduler API ergonomics, system chaining, the ability to run exclusive systems and apply deferred system operations at any point in a schedule, a single unified schedule, configurable System Sets, run conditions, and a better State system.

However it pretty quickly became clear that the new system still had some areas to improve:

* **Base Sets were hard to understand and error prone**: What _is_ a Base Set? When do I use them? Why do they exist? Why is my ordering implicitly invalid due to incompatible Base Set ordering? Why do some schedules have a default Base Set while others don't? [Base Sets were confusing!](https://github.com/bevyengine/bevy/pull/8079#base-set-confusion)
* **There were too many ways to schedule a System**: We've accumulated too many scheduling APIs. As of Bevy **0.10**, we had [_SIX_ different ways to add a system to the "startup" schedule](https://github.com/bevyengine/bevy/pull/8079#unify-system-apis). Thats too many ways!
* **Too much implicit configuration**: There were both default Schedules and default Base Sets. In some cases systems had default schedules or default base sets, but in other cases they didn't! [A system's schedule and configuration should be explicit and clear](https://github.com/bevyengine/bevy/pull/8079#schedule-should-be-clear).
* **Adding Systems to Schedules wasn't ergonomic**: Things like `add_system(foo.in_schedule(CoreSchedule::Startup))` were not fun to type or read. We created special-case helpers, such as `add_startup_system(foo)`, but [this required more internal code, user-defined schedules didn't benefit from the special casing, and it completely hid the `CoreSchedule::Startup` symbol!](https://github.com/bevyengine/bevy/pull/8079#ergonomic-system-adding).

### Unraveling the Complexity

If your eyes started to glaze over as you tried to wrap your head around this, or phrases like "implicitly added to the `Update` Base Set" filled you with dread ... don't worry. After [a lot of careful thought](https://github.com/bevyengine/bevy/pull/8079) we've unraveled the complexity and built something clear and simple.

In **Bevy 0.11** the "scheduling mental model" is _much_ simpler thanks to **Schedule-First ECS APIs**:

```rust
app
    .add_systems(Startup, (a, b))
    .add_systems(Update, (c, d, e))
    .add_systems(FixedUpdate, (f, g))
    .add_systems(PostUpdate, h)
    .add_systems(OnEnter(AppState::Menu), enter_menu)
    .add_systems(OnExit(AppState::Menu), exit_menu)
```

* **There is _exactly_ one way to schedule systems**
    * Call `add_systems`, state the schedule name, and specify one or more systems
* **Base Sets have been entirely removed in favor of Schedules, which have friendly / short names**
    * Ex: The `CoreSet::Update` Base Set has become `Update`
* **There is no implicit or implied configuration**
    * Default Schedules and default Base Sets don't exist
* **The syntax is easy on the eyes and ergonomic**
    * Schedules are first so they "line up" when formatted

<details>
    <summary>To compare, expand this to see what it used to be!</summary>

```rust
app
    // Startup system variant 1.
    // Has an implied default StartupSet::Startup base set
    // Has an implied CoreSchedule::Startup schedule
    .add_startup_systems((a, b))
    // Startup system variant 2.
    // Has an implied default StartupSet::Startup base set
    // Has an implied CoreSchedule::Startup schedule
    .add_systems((a, b).on_startup())
    // Startup system variant 3.
    // Has an implied default StartupSet::Startup base set
    .add_systems((a, b).in_schedule(CoreSchedule::Startup))
    // Update system variant 1.
    // `CoreSet::Update` base set and `CoreSchedule::Main` are implied
    .add_system(c)
    // Update system variant 2 (note the add_system vs add_systems difference)
    // `CoreSet::Update` base set and `CoreSchedule::Main` are implied
    .add_systems((d, e))
    // No implied default base set because CoreSchedule::FixedUpdate doesn't have one
    .add_systems((f, g).in_schedule(CoreSchedule::FixedUpdate))
    // `CoreSchedule::Main` is implied, in_base_set overrides the default CoreSet::Update set
    .add_system(h.in_base_set(CoreSet::PostUpdate))
    // This has no implied default base set
    .add_systems(enter_menu.in_schedule(OnEnter(AppState::Menu)))
    // This has no implied default base set
    .add_systems(exit_menu.in_schedule(OnExit(AppState::Menu)))
```

</details>

Note that normal "system sets" still exist! You can still use sets to organize and order your systems:

```rust
app.add_systems(Update, (
    (walk, jump).in_set(Movement),
    collide.after(Movement),
))
```

The `configure_set` API has also been adjusted for parity:

```rust
// before
app.configure_set(Foo.after(Bar).in_schedule(PostUpdate))
// after
app.configure_set(PostUpdate, Foo.after(Bar))
```

## Nested System Tuples and Chaining

<div class="release-feature-authors">authors: @cart</div>

It is now possible to infinitely nest tuples of systems in a `.add_systems` call!

```rust
app.add_systems(Update, (
    (a, (b, c, d, e), f),
    (g, h),
    i
))
```

At first glance, this might not seem very useful. But in combination with per-tuple configuration, it allows you to easily and cleanly express schedules:

```rust
app.add_systems(Update, (
    (attack, defend).in_set(Combat).before(check_health)
    check_health,
    (handle_death, respawn).after(check_health)
))
```

`.chain()` has also been adapted to support arbitrary nesting! The ordering in the example above could be rephrased like this:

```rust
app.add_systems(Update,
    (
        (attack, defend).in_set(Combat)
        check_health,
        (handle_death, respawn)
    ).chain()
)
```

This will run `attack` and `defend` first (in parallel), then `check_health`, then `handle_death` and `respawn` (in parallel).

This allows for powerful and expressive "graph-like" ordering expressions:

```rust
app.add_systems(Update,
    (
        (a, b, c).chain(),
        (d, e),
    ).chain()
)
```

This will run a, then b, then c, then run d _and_ e in parallel.

## <a name="what-s-next"></a>What's Next?

* **X**: Y

Check out the [**Bevy 0.12 Milestone**](https://github.com/bevyengine/bevy/milestone/14) for an up-to-date list of current work being considered for **Bevy 0.12**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:

* @author
