+++
title = "Bevy 0.12"
date = 2023-10-21
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.12** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.12**, check out our [0.11 to 0.12 Migration Guide](/learn/migration-guides/0.11-0.12/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

## Feature Name

<div class="release-feature-authors">authors: @author</div>

## One-Shot Systems

<div class="release-feature-authors">authors: @alice-i-cecile @pascualex, @Trashtalk217, @Zeenobit</div>

Ordinarily, systems run once per frame, as part of a schedule.
But what if you don't want to poll for them regularly?
Maybe you're responding to a very rare event like in a complex turn-based game, or simply don't want to clutter your schedule with a new system for every single button.
One-shot systems flip that logic on its head, and provide you the ability to run arbitrary logic on demand, using the powerful and familiar system syntax.

```rust
#[derive(Resource, Default, Debug)]
struct Counter(u8);

fn increment(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("{}", counter.0);
}

let mut world = World::new();
world.init_resource::<Counter>();
let id = world.register_system(increment);
let _ = world.run_system(id); // prints 1
let _ = world.run_system(id); // prints 2
```

There are three simple steps to using one-shot systems: register a system, store its `SystemId` somewhere, and then use either exclusive world access or commands to run the system corresponding to that id. 

One-shot systems are very flexible. For example, you can have multiple instances of one system registered at the same time. One-shot systems can be nested (although not recursive), and you can also wrap `SystemId`s into components, making it possible to treat registered systems like entities.

```rust
use bevy::ecs::system::SystemId;

#[derive(Component)]
struct Callback(SystemId);

// calling all callbacks!
fn call_all(query: Query<&Callback>, mut commands: Commands) {
    for callback in query.iter() {
        commands.run_system(callback.0);
    }
}
```

One-shot systems are not without their limitations.
Currently, exclusive systems and systems designed for system piping (with either an `In` parameter or a return type) can't be used at all.
Additionally, one-shot systems are always evaluated sequentially, rather than in parallel.
While this reduces both complexity and overhead, this can be meaningfully slower than using a schedule with a parallel executor if you want to run a large number of heavy systems at once.

Registering and running systems offers improved performance and correctness: you can have multiple copies of a single system, each with their own `Local` values and cached system state. 
It also plays nice with asset-driven workflows: recording a mapping from a string to an identifier in a serialized callback is much nicer than trying to do so with Rust functions!

However, when you're just prototyping or writing a unit test, it can be a real hassle: who wants all that boilerplate and indirection!
When those caveats don't bother you, just use the `World::run_system_once` method.

```rust
use bevy::ecs::system::RunSystemOnce;

#[derive(Resource, Default, Debug)]
struct Counter(u8);

fn increment(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("{}", counter.0);
}

let mut world = World::new();
world.init_resource::<Counter>();
world.run_system_once(increment); // prints 1
world.run_system_once(increment); // prints 2
```

This is great for unit testing systems and queries, and it's both lower overhead and simpler to use. However, there is one caveat. Some systems have state, either in the form of `Local` arguments or change detection. This state isn't saved between two `run_system_once` calls, creating odd behavior. The `Local`'s reset every run, while change detection will *always* detect data as added/changed. Be careful and you'll be alright.

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:
