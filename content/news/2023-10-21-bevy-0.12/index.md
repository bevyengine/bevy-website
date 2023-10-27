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

## Wireframe Improvements

<div class="release-feature-authors">authors: @IceSentry</div>

The wireframes now use bevy's Material abstraction. This means it will automatically use the new batching and instancing features while being easier to maintain. This change also made it easier to add support for colored wireframe. You can configure the color globally or per mesh using the `WireframeColor` component. It's also now possible to disable wireframe rendering by using the `NeverRenderWireframe` component.

![wireframe](wireframe.png)

## Reflect Commands

<div class="release-feature-authors">authors: @NoahShomette</div>

It is now possible to insert and remove reflect components from an entity in a normal system via new functions on [`Commands`]!

```rust
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Component(u32);

fn reflect_commands(mut commands: Commands) {
    let boxed_reflect_component: Box<dyn Reflect> = Box::new(Component(916));

    let entity = commands
        .spawn_empty()
        .insert_reflect(boxed_reflect_component.clone_value()).id();

    commands.entity(entity).remove_reflect(boxed_reflect_component.type_name().to_owned());

}
```

The above commands use the [`AppTypeRegistry`] by default. If you use a different TypeRegistry then you can use the ...`with_registry` commands instead.

```rust
 #[derive(Resource)]
 struct TypeRegistryResource {
     type_registry: TypeRegistry,
 }

 impl AsRef<TypeRegistry> for TypeRegistryResource {
     fn as_ref(&self) -> &TypeRegistry {
         &self.type_registry
     }
 }

 fn reflect_commands_with_registry(mut commands: Commands) {
    let boxed_reflect_component: Box<dyn Reflect> = Box::new(Component(916));

    let entity = commands
        .spawn_empty()
        .insert_reflect_with_registry::<TypeRegistryResource>(boxed_reflect_component.clone_value()).id();

    commands.entity(entity).remove_reflect_with_registry::<TypeRegistryResource>(boxed_reflect_component.type_name().to_owned());

}
```

See [`ReflectCommandExt`] for more examples and documentation

[`Commands`]: https://docs.rs/bevy/0.12.0/bevy/ecs/system/struct.Commands.html

[`AppTypeRegistry`]: https://docs.rs/bevy/0.12.0/bevy/ecs/reflect/struct.AppTypeRegistry.html

[`ReflectCommandExt`]: https://docs.rs/bevy/0.12.0/bevy/ecs/reflect/trait.ReflectCommandExt.html

## Limit Background FPS

<div class="release-feature-authors">authors: @maniwani</div>

If an app has no window in focus, Bevy will now limit its update rate (to 60Hz by default).

Before, many Bevy apps running on desktop operating systems (particularly macOS) would see spikes in CPU usage whenever their windows were minimized or completely covered, even with VSync enabled. The reason for this is that many desktop window managers ignore VSync for windows that aren't visible. As VSync normally limits how often an app updates, that speed limit vanishes while it's effectively disabled.

Now, apps running in the background will sleep in between updates to limit their FPS.

The one caveat is that most operating systems will not report if a window is visible, only if it has focus. So the throttle is based on focus, not visibility. 60Hz was then chosen as the default to maintain high FPS in cases where the window is not focused but still visible.

## `AnimationPlayer` API Improvements

<div class="release-feature-authors">authors: @devinleamy</div>

The `AnimationPlayer` now has new methods for controlling playback, and utilities for checking
if an animation is playing or completed, and getting its `AnimationClip` handle.

`set_elapsed` and has been removed in favor of `seek_to`. `elapsed` now
returns the actual elapsed time and is not affected by the animation speed. `stop_repeating` have been removed
in favor of `set_repeat(RepeatAnimation::Never)`.

```rust
let mut player = q_animation_player.single_mut();
// Check if an animation is complete.
if player.is_finished() {
    // Set the playback mode.
    player.set_repeat(RepeatAnimation::Forever);
    player.set_repeat(RepeatAnimation::Never);
    player.set_repeat(RepeatAnimation::Count(4));
}
// Get a handle to the playing AnimationClip.
let clip_handle = player.animation_clip();
// Seek to 1s in the current clip.
player.seek_to(1.0);
```

## Ignore Ambiguous Components and Resources

<div class="release-feature-authors">authors: @hymm</div>

Ambiguity Reporting is an optional feature of Bevy's scheduler. When enabled it reports conflicts between systems that modify the same data, but are not ordered in relation to each other. While some reported conflicts can cause subtle bugs, many do not. Bevy has a couple existing methods and two new ones for ignoring these.

The existing APIs: `ambiguous_with`, which ignores conflicts between specific sets, and `ambiguous_with_all`, which ignores all conflicts with the set it's applied to. In addition, there are now 2 new APIs that let you ignore conflicts on a type of data, `allow_ambiguous_component` and `allow_ambiguous_resource`. These ignore all conflicts between systems on that specific type, component or resource, in a world.

```rust
#[derive(Resource)]
struct R;

// These systems are ambiguous on R
fn system_1(_: ResMut<R>) {}
fn system_2(_: Res<R>) {}

let mut app = App::new();
app.configure_schedules(ScheduleBuildSettings {
  ambiguity_detection: LogLevel::Error,
  ..default()
});
app.insert_resource(R);

app.add_systems(Update, ( system_1, system_2 ));
app.allow_ambiguous_resource::<R>();

// Running the app does not error.
app.update();
```

Bevy is now using this to ignore conflicts between the `Assets<T>` resources. Most of these ambiguities are modifying different assets and thus do not matter.

## Spatial Audio API Ergonomics

<div class="release-feature-authors">authors: @rparrett, @hymm, @mockersf</div>

A simple "stereo" (non-HRTF) spatial audio implementation was heroically [put together](https://bevyengine.org/news/bevy-0-10/#spatial-audio) at the last minute for Bevy 0.10, but the implementation was somewhat bare-bones and not very user-friendly. Users needed to write their own systems to update audio sinks with emitter and listener positions.

Now users can just add a `TransformBundle` to their `AudioBundle`s and Bevy will take care of the rest!

```rust
commands.spawn((
    TransformBundle::default(),
    AudioBundle {
        source: asset_server.load("sounds/bonk.ogg"),
        settings: PlaybackSettings::DESPAWN.with_spatial(true),
    },
));
```

## Added HSL methods to `Color` struct

<div class="release-feature-authors">authors: @idedary</div>

You can now use `h()`, `s()`, `l()` together with their `set_h()`, `set_s()`, `set_l()` and `with_h()`, `with_s()`, `with_l()` variants to manipulate _Hue_, _Saturation_ and _Lightness_ values of a `Color` struct without cloning. Previously you could do that with only RGBA values.

```rust
// Returns HSL component values
let color = Color::ORANGE;
let hue = color.h();
// ...

// Changes the HSL component values
let mut color = Color::PINK;
color.set_s(0.5);
// ...

// Modifies existing colors and returns them
let color = Color::VIOLET.with_l(0.7);
// ...
```

## One-Shot Systems

<div class="release-feature-authors">authors: @alice-i-cecile @pascualex, @Trashtalk217, @Zeenobit</div>

Ordinarily, systems run once per frame, as part of a schedule.
But this isn't always the right fit.
Maybe you're responding to a very rare event like in a complex turn-based game, or simply don't want to clutter your schedule with a new system for every single button.
One-shot systems flip that logic on its head, and provide you the ability to run arbitrary logic on demand, using the powerful and familiar system syntax.

```rust
#[derive(Resource, Default, Debug)]
struct Counter(u8);

fn increment(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("{}", counter.0);
}

fn foo(world: &mut World) {
    world.init_resource::<Counter>();
    let id = world.register_system(increment);
    let _ = world.run_system(id); // prints 1
    let _ = world.run_system(id); // prints 2
}
```

There are three simple steps to using one-shot systems: register a system, store its `SystemId`, and then use either exclusive world access or commands to run the corresponding system.

A lot becomes possible with just that, however `SystemId`s really start showing their power, when they're wrapped into components.

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

One-shot systems can then be attached to UI elements, like buttons, actions in an RPG, or any other entity. You might even feel inspired to implement the bevy scheduling graph with one-shot systems and [`aery`](https://docs.rs/aery/latest/aery/) (let us know how that goes, by the way).

One-shot systems are very flexible.
They can be nested, so you can call `run_system` from within a one-shot system.
It's possible to have multiple instances of one system registered at a time, each with their own `Local` variables and cached system state.
It also plays nice with asset-driven workflows: recording a mapping from a string to an identifier in a serialized callback is much nicer than trying to do so with Rust functions!

Still, one-shot systems are not without their limitations.
Currently, exclusive systems and systems designed for system piping (with either an `In` parameter or a return type) can't be used at all.
You also can't call a one-shot systems from itself, recursion isn't possible.
Lastly, one-shot systems are always evaluated sequentially, rather than in parallel.
While this reduces both complexity and overhead, for certain workloads this can be meaningfully slower than using a schedule with a parallel executor.

However, when you're just prototyping or writing a unit test, it can be a real hassle: two whole functions and some weird identifier?
For these situations, you can use the `World::run_system_once` method.

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

This is great for unit testing systems and queries, and it's both lower overhead and simpler to use. However, there is one caveat. Some systems have state, either in the form of `Local` arguments, change detection, or `EventReader`s. This state isn't saved between two `run_system_once` calls, creating odd behavior. The `Local`s reset every run, while change detection will _always_ detect data as added/changed. Be careful and you'll be alright.

## Unified `Time`

<div class="release-feature-authors">authors: @nakedible @maniwani @alice-i-cecile</div>

Bevy 0.12 brings two major quality of life improvements to `FixedUpdate`.

- `Time` now returns the contextually correct values for systems running in `FixedUpdate`. (As such, `FixedTime` has been removed.)
- `FixedUpdate` can no longer snowball into a "death spiral" (where the app freezes because `FixedUpdate` steps are enqueued faster than it can run them).

The `FixedUpdate` schedule and its companion `FixedTime` resource were introduced in Bevy 0.10, and it soon became apparent that `FixedTime` was lacking. Its methods were different from `Time` and it didn't even track "total time elapsed" like `Time` did, to name a few examples. Having two different "time" APIs also meant you had to write systems to specifically support "fixed timestep" or "variable timestep" and not both. It was desirable to not have this split as it can lead to incompatibilities between plugins down the road (which is sometimes the case with plugins in other game engines).

Now, you can just write systems that read `Time` and schedule them in either context.

```rust
// This system will see a constant delta time if scheduled in `FixedUpdate` or
// a variable delta time if scheduled in `Update`.
fn integrate_velocity(
    mut query: Query<(&mut Transfrom, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
```

Most systems should continue to use `Time`, but behind the scenes, the methods from previous APIs have been refactored into four clocks:

- `Time<Real>`
- `Time<Virtual>`
- `Time<Fixed>`
- `Time<()>`

`Time<Real>` measures the true, unedited frame and app durations. For diagnostics/profiling, use that one. It's also used to derive the others. `Time<Virtual>` can be sped up, slowed down, and paused, and `Time<Fixed>` chases `Time<Virtual>` in fixed increments. Lastly, `Time<()>` is automatically overwritten with the current value of `Time<Fixed>` or `Time<Virtual>` upon entering or exiting `FixedUpdate`. When a system borrows `Time`, it actually borrows `Time<()>`.

Try the new `time` example to get a better feel for these resources.

The fix for the windup problem was limiting how much `Time<Virtual>` can advance from a single frame. This then limits how many times `FixedUpdate` can be queued for the next frame, and so things like frame lag or your computer waking up from a long sleep can no longer cause a death spiral. So now, the app won't freeze, but things happening in `FixedUpdate` will appear to slow down since it'll be running at a temporarily reduced rate.

## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:
