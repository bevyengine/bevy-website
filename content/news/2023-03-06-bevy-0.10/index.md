+++
title = "Bevy 0.10"
date = 2023-03-06
authors = ["Bevy Contributors"]
[extra]
image = "ruins.png"
show_image = true
image_subtitle = "Ruins scene illustrating Bevy's new material blend modes and fog. Based on original scene by Casey Hardy (CC Attribution)"
image_subtitle_link = "https://github.com/coreh/bevy-demo-ruins"
+++

Thanks to **173** contributors, **689** pull requests, community reviewers, and our [**generous sponsors**](/donate), we're happy to announce the **Bevy 0.10** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start/introduction) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevy.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.10**, check out our [0.9 to 0.10 Migration Guide](/learn/migration-guides/0.9-0.10/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **ECS Schedule v3**: Bevy now has much simpler, more flexible scheduling. Systems are now stored in a unified schedule, commands can be applied explicitly via `apply_system_buffers`, and a whole lot of quality of life and bug fixes.
* **Cascaded Shadow Maps**: Higher quality shadow maps that cover larger distances, where the quality follows the camera.
* **Environment Map Lighting**: 360 degree ambient image based lighting that can cheaply and drastically improve the visual quality of a scene.
* **Depth and Normal Prepass**: Render depth and normal textures for a scene prior to the main pass, enabling new effects and (in some cases) improved performance. Shadow mapping uses the prepass shaders, which enables transparent textures to cast shadows.
* **Smooth Skeletal Animation Transitions**: Smoothly transition between two skeletal animations playing at the same time!
* **Improved Android Support**: Bevy now works out of the box on more Android devices (with a couple of caveats)
* **Revamped Bloom**: Bloom now looks better, is easier to control, and has fewer visual artifacts.
* **Distance and Atmospheric Fog**: Add depth and ambiance to your scene with 3D distance and atmospheric fog effects!
* **StandardMaterial Blend Modes**: Achieve a variety of interesting effects with more PBR material blend modes.
* **More Tonemapping Choices**: Choose one of 7 popular tonemapping algorithms for your HDR scenes to achieve the visual style you are looking for.
* **Color Grading**: Control per-camera exposure, gamma, "pre-tonemapping saturation", and "post-tonemapping saturation".
* **Parallel Pipelined Rendering**: App logic and render logic now run in parallel automatically, yielding significant performance wins.
* **Windows as Entities**: Windows are now represented as entities instead of resources, improving the user experience and unlocking new scenarios.
* **Renderer Optimizations**: We spent a _ton_ of effort optimizing the renderer this cycle. Bevy's renderer is snappier than ever!
* **ECS Optimizations**: Likewise, we've turbocharged many common ECS operations. Bevy apps get a nice speed boost!

## ECS Schedule v3

<div class="release-feature-authors">authors: @alice-i-cecile, @maniwani, @WrongShoe, @cart, @jakobhellermann, @JoJoJet, @geieredgar and a whole lot more </div>

Thanks to the fantastic work of our ECS team, the hotly awaited ["stageless" scheduling RFC](https://github.com/bevyengine/rfcs/blob/main/rfcs/45-stageless.md) has been implemented!

**Schedule v3** is the culmination of significant design and implementation work. Scheduling APIs are a central and defining part of the Bevy developer experience, so we had to be very thoughtful and meticulous about this next evolution of the API. In addition to the [RFC PR](https://github.com/bevyengine/rfcs/pull/45), the [initial implementation PR](https://github.com/bevyengine/bevy/pull/6587) by `@maniwani` and the [Bevy Engine internals port PR](https://github.com/bevyengine/bevy/pull/7267) by `@alice-i-cecile` are great places to start if you would like a view into our process and rationale. As we all know, plans and implementations are two different things. Our final implementation is a bit different from the initial RFC (in a good way).

There are a ton of changes, but we've put a lot of care into ensuring the [migration path](/learn/migration-guides/0.9-0.10/#migrate-engine-to-schedule-v3-stageless) for existing applications is relatively straightforward. Don't sweat it!

Let's take a look at what shipped in 0.10!

### A Single Unified Schedule

Have you ever wanted to specify that `system_a` runs before `system_b`, only to be met with confusing warnings that `system_b` isn't found because it's in a different stage?

No more! All systems within a single [`Schedule`] are now stored in a single data structure with a global awareness of what's going on.

This simplifies our internal logic, makes your code more robust to refactoring, and allows plugin authors to specify high-level invariants (e.g. "movement must occur before collision checking") without locking themselves into an exact schedule location.

[![main_schedule_diagram](main_schedule_diagram.svg)](main_schedule_diagram.svg)

This diagram made with [@jakobhellermann's `bevy_mod_debugdump` crate](https://github.com/jakobhellermann/bevy_mod_debugdump) shows a simplified version of Bevy's default schedule.

[`Schedule`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/struct.Schedule.html

### Adding Systems

[`Systems`] (which are just [normal Rust functions!](https://github.com/bevyengine/bevy/tree/v0.10.0/crates/bevy_ecs#systems)) are how you define game logic in Bevy ECS. With **Schedule v3**, you can add systems to your [`App`] just like you did in previous versions:

```rust
app.add_system(gravity)
```

However **Schedule v3** has some new tricks up its sleeve! You can now add multiple systems at once:

```rust
app.add_systems((apply_acceleration, apply_velocity))
```

By default, Bevy runs systems in parallel to each other. In previous versions of Bevy, you ordered systems like this:

```rust
app
    .add_system(walk.before(jump))
    .add_system(jump)
    .add_system(collide.after(jump))
```

You can still do that! But you can now compress this using `add_systems`:

```rust
// much cleaner!
app.add_systems((
    walk.before(jump),
    jump,
    collide.after(jump),
))
```

`before()` and `after()` are definitely useful tools! However, thanks to the new `chain()` function, it is now _much easier_ to run systems in a specific order:

```rust
// This is equivalent to the previous example
app.add_systems((walk, jump, collide).chain())
```

`chain()` will run the systems in the order they were defined. Chaining also pairs with per-system configuration:

```rust
app.add_systems((walk.after(input), jump, collide).chain())
```

[`App`]: https://docs.rs/bevy/0.10.0/bevy/app/struct.App.html
[`Systems`]: https://docs.rs/bevy/0.10.0/bevy/ecs/system/trait.System.html

### Configurable System Sets

In **Schedule v3**, the idea of the "system set" has been redefined to support more natural and flexible control over how systems are run and scheduled. The old "system label" concept has been combined with the "set" concept, resulting in one straightforward but powerful abstraction.

[`SystemSets`] are named collections of systems that share system configuration across all of their members. Ordering systems relative to a [`SystemSet`] applies that ordering to _all_ systems in that set, in addition to any configuration on each individual system.

Let's jump right into what this would look like. You define [`SystemSets`] like this:

```rust
#[derive(SystemSet, Debug, Hash, PartialEq, Eq, Clone)]
enum PhysicsSet {
    Movement,
    CollisionDetection,
}
```

You can add systems to sets by calling the [`in_set`] method:

```rust
app.add_system(gravity.in_set(PhysicsSet::Movement))
```

You can combine this with the new system features mentioned above:

```rust
app.add_systems(
    (apply_acceleration, apply_velocity)
        .chain()
        .in_set(PhysicsSet::Movement)
)
```

Systems can belong to any number of sets:

```rust
app.add_system(
    move_player
        .in_set(MoveSet::Player)
        .in_set(PhysicsSet::Movement)
)
```

Configuration is added to sets like this:

```rust
app.configure_set(
    // Run systems in the Movement set before systems in the CollisionDetection set
    PhysicsSet::Movement.before(PhysicsSet::CollisionDetection)
)
```

Sets can be nested inside other sets, which will cause them to inherit the configuration of their parent set:

```rust
app.configure_set(MoveSet::Enemy.in_set(PhysicsSet::Movement))
```

Sets can be configured multiple times:

```rust
// In PlayerPlugin:
app.configure_set(MoveSet::Player.before(MoveSet::Enemy))

// In PlayerTeleportPlugin
app.configure_set(MoveSet::Player.after(PortalSet::Teleport))
```

Crucially system configuration is strictly additive: you cannot _remove_ rules added elsewhere. This is both an "anti-spaghetti" and "plugin privacy" consideration. When this rule is combined with Rust's robust type privacy rules, plugin authors can make careful decisions about which exact invariants need to be upheld, and reorganize code and systems internally without breaking consumers.

Configuration rules _must be compatible with each other_: any paradoxes (like a system set inside of itself, a system that must run both before and after a set, order cycles, etc) will result in a runtime panic with a helpful error message.

[`SystemSet`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/trait.SystemSet.html
[`SystemSets`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/trait.SystemSet.html
[`in_set`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/trait.IntoSystemConfig.html#method.in_set

### Directly Schedule Exclusive Systems

"Exclusive systems" are [`Systems`] that have mutable direct access to the entire ECS [`World`]. For this reason, they cannot be run in parallel with other [`Systems`].

Since Bevy's inception, Bevy devs have wanted to schedule exclusive systems (and flush commands) relative to normal systems.

Now you can! Exclusive systems can now be scheduled and ordered like any other system.

```rust
app
    .add_system(ordinary_system)
    // This works!
    .add_system(exclusive_system.after(ordinary_system))
```

This is particularly powerful, as **command flushes** (which apply queued-up [`Commands`] added in systems to do things like spawn and despawn entities) are now simply performed in the `apply_system_buffers` exclusive system.

```rust
app.add_systems(
    (
        // This system produces some commands
        system_a,
        // This will apply the queued commands from system_a
        apply_system_buffers,
        // This system will have access to the results of
        // system_a's commands
        system_b,
    // This chain ensures the systems above run in the order
    // they are defined
    ).chain()
)
```

Do be careful with this pattern though: it's easy to quickly end up with many poorly ordered exclusive systems, creating bottlenecks and chaos.

What will you do with this much power? We're keen to find out!

### Managing Complex Control Flow with Schedules

But what if you want to do something _weird_ with your [`Schedule`]? Something non-linear, branching, or looping. What should you reach for?

It turns out, Bevy already _had_ a great tool for this: schedules that run inside of an exclusive system. The idea is pretty simple:

1. Construct a schedule, that stores whatever complex logic you want to run.
2. Store that schedule inside of a resource.
3. In an exclusive system, perform any arbitrary Rust logic you want to decide if and how your schedule runs.
4. Temporarily take the schedule out of the [`World`], run it on the rest of the world to mutate both the schedule and the world, and then put it back in.

With the addition of the new [`Schedules`] resource and the `world.run_schedule()` API it's more ✨ ergonomic ✨ than ever.

```rust
// A Schedule!
let mut my_schedule = Schedule::new();
schedule.add_system(my_system);

// A label for our new Schedule!
#[derive(ScheduleLabel, Debug, Hash, PartialEq, Eq, Clone)]
struct MySchedule;

// An exclusive system to run this schedule
fn run_my_schedule(world: &mut World) {
    while very_complex_logic() {
        world.run_schedule(MySchedule);
    }
}

// Behold the ergonomics!
app
    .add_schedule(MySchedule, my_schedule)
    .add_system(run_my_schedule);
```

Bevy uses this pattern for five rather different things in **Bevy 0.10**:

1. **Startup systems:** these now live in their own schedule, which is run once at the start of the app.
2. **Fixed timestep systems:** another schedule?! The exclusive system that runs this schedule accumulates time, running a while loop that repeatedly runs `CoreSchedule::FixedUpdate` until all of the accumulated time has been spent.
3. **Entering and exiting states:** a bonanza of schedules. Each collection of systems that runs logic to enter and exit a state variant is stored in its own schedule, which is called based on the change in state in the `apply_state_transitions::<S>` exclusive system.
4. **Rendering:** all rendering logic is stored in its own schedule to allow it to run asynchronously relative to gameplay logic.
5. **Controlling the outermost loop:** in order to handle the "startup schedule first, then main schedule" logic, we wrap it all up in a minimal overhead `CoreSchedule::Outer` and then run our schedules as the sole exclusive system there.

Follow the breadcrumbs starting at [`CoreSchedule`](https://docs.rs/bevy/0.10.0/bevy/app/enum.CoreSchedule.html) for more info.

[`Schedules`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/struct.Schedules.html

### Run Conditions

[`Systems`] can have any number of **run conditions**, which are "just" systems that return a `bool`. If the `bool`s returned by _all_ of a system's **run conditions** are `true`, the system will run. Otherwise the system will be skipped for the current run of the schedule:

```rust
// Let's make our own run condition
fn game_win_condition(query: Query<&Player>, score: Res<Score>) -> bool {
    let player = query.single();
    player.is_alive() && score.0 > 9000
}

app.add_system(win_game.run_if(game_win_condition));
```

**Run conditions** also have a number of "combinator" operations, thanks to [@JoJoJet](https://github.com/bevyengine/bevy/pull/7547) and [@Shatur](https://github.com/bevyengine/bevy/pull/7559):

They can be negated with `not()`:

```rust
app.add_system(continue_game.run_if(not(game_win_condition)))
```

They can also be combined with `and_then` and `or_else`:

```rust
app.add_system(move_player.run_if(is_alive.or_else(is_zombie)))
```

Bevy 0.10 is shipping with a lovely collection of built-in [common run conditions](https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/common_conditions/index.html). You can easily run systems if there are events to process, timers that elapsed, resources that changed, input state changes, states that changed, and more (thanks to [`@maniwani`](https://github.com/bevyengine/bevy/pull/6587), [`@inodentry`](https://github.com/bevyengine/bevy/pull/7579), [`@jakobhellermann`](https://github.com/bevyengine/bevy/pull/7806), and [`@jabuwu`](https://github.com/bevyengine/bevy/pull/7866)).

**Run conditions** can also serve as a lightweight optimization tool. Run conditions are evaluated on the main thread, and each run criteria is evaluated exactly once each schedule update, at the time of the first system in the set that relies on it. Systems disabled by run conditions don't spawn a task, which can add up across many systems. Like always though: benchmark!

**Run conditions** have replaced the "run criteria" in previous versions of Bevy. We can finally get rid of the dreaded "looping run criteria"! [`ShouldRun::YesAndCheckAgain`](https://docs.rs/bevy/0.9.1/bevy/ecs/schedule/enum.ShouldRun.html) was not exactly straightforward to reason about, either for engine devs or users. It's always a bad sign when your bool-like enums have four possible values. If you crave more complex control flow: use the "schedules in exclusive systems" pattern in the [section above](#directly-schedule-exclusive-systems). For the other 99% of use cases, enjoy the simpler `bool`-based run conditions!

### Simpler States

**Schedule v3** adds a new, much simpler "state system". [`States`] allow you to easily configure different [`App`] logic to run based on the current "state" of the [`App`].

You define [`States`] like this:

```rust
#[derive(States, PartialEq, Eq, Debug, Clone, Hash, Default)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}
```

Each variant of the enum corresponds to a different state the [`App`] can be in.

You add [`States`] to your [`App`] like this:

```rust
app.add_state::<AppState>()
```

This will setup your [`App`] to use the given state. It adds the [`State`] resource, which can be used to find the current state the [`App`] is in:

```rust
fn check_state(state: Res<State<AppState>>) {
    info!("We are in the {} state", state.0);
}
```

Additionally, `add_state` will create an `OnUpdate` set for each possible value, which you can then add your systems to. These sets run as part of the normal app update, but only when the app is in a given state:

```rust
app
    .add_systems(
        (main_menu, start_game)
            .in_set(OnUpdate(AppState::MainMenu))
    )
    .add_system(fun_gameplay.in_set(OnUpdate(AppState::InGame)));
```

It will also create `OnEnter` and `OnExit` schedules for each state, which will only run when transitioning from one state to another:

```rust
app
    .add_system(load_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
    .add_system(cleanup_main_menu.in_schedule(OnExit(AppState::MainMenu)))
```

`add_state` also adds the [`NextState`] resource, which can be used to queue a state change:

```rust
fn start_game(
    button_query: Query<&Interaction, With<StartGameButton>>,
    mut next_state: ResMut<NextState<AppState>>,
){
    if button_query.single() == Interaction::Pressed {
        next_state.set(AppState::InGame);
    }
}
```

This replaces Bevy's previous state system, which was very hard to deal with. It had state stacks, elaborate queued transitions, and error handling (that most people just unwrapped). The state stack was very complex to learn, very prone to exasperating bugs, and mostly ignored.

As a result, in **Bevy 0.10** states are now "stackless": only one queued state of each type at a time. After lots of alpha testing, we're reasonably confident that this shouldn't be too bad to migrate away from. If you were relying on the state stack, you have plenty of options:

* Build the "stack" logic on top of the core state system
* Split your state into multiple states, which capture orthogonal elements of your app's status
* Build your own state stack abstraction using the same patterns as Bevy's first-party version. None of the new state logic is hard coded! If you build something, [let the rest of the community know](/assets) so you can collaborate!

[`States`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/trait.States.html
[`State`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/struct.State.html
[`NextState`]: https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/struct.NextState.html

### Base Sets: Getting Default Behavior Right

An astute reader may point out that:

1. Bevy automatically runs its systems in parallel.
2. [The order of systems is nondeterministic unless there is an explicit ordering relationship between them](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/nondeterministic_system_order.rs)
3. All of the systems are now stored in a single `Schedule` object with no barriers between them
4. Systems can belong to any number of system sets, each of which can add their own behavior
5. Bevy is a powerful engine with many internal systems.

Won't this lead to utter chaos and tedious spaghetti-flavored work to resolve every last ordering ambiguity?
Many users _liked_ stages, they were helpful for understanding the structure of an [`App`]!

Well, we're glad you asked, rhetorical skeptic. To reduce this chaos (and ease migration), **Bevy 0.10** comes with a brand new collection of system sets provided by [`DefaultPlugins`]: [`CoreSet`](https://docs.rs/bevy/0.10.0/bevy/app/enum.CoreSet.html), [`StartupSet`](https://docs.rs/bevy/0.10.0/bevy/app/enum.StartupSet.html), and [`RenderSet`](https://docs.rs/bevy/0.10.0/bevy/render/enum.RenderSet.html). The similarity of their names to the old [`CoreStage`](https://docs.rs/bevy/0.9.1/bevy/app/enum.CoreStage.html), [`StartupStage`](https://docs.rs/bevy/0.9.1/bevy/app/enum.StartupStage.html), and [`RenderStage`](https://docs.rs/bevy/0.9.1/bevy/render/enum.RenderStage.html) is not a coincidence. Much like stages, there are command flush points between each set, and existing systems have been migrated directly.

Some parts of the stage-centric architecture were appealing: a clear high-level structure, coordination on flush points (to reduce excessive bottlenecks), and good default behavior.
To keep those bits (while excising the frustrating ones), we've introduced the concept of **Base Sets** ([added by @cart](https://github.com/bevyengine/bevy/pull/7466)). **Base Sets** are just normal [`SystemSets`], except:

1. Every system can belong to at most one base set.
2. Systems that do not specify a base set will be added to the default base set for the schedule (if the schedule has one).

```rust
// You define base sets exactly like normal sets, with the
// addition of the system_set(base) attribute
#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
#[system_set(base)]
enum MyBaseSet {
    Early,
    Late,
}

app
    // This ends up in CoreSet::Update by default
    .add_system(no_explicit_base_set)
    // You must use .in_base_set rather than .in_set for explicitness
    // This is a high-impact decision!
    .add_system(post_update.in_base_set(CoreSet::PostUpdate))
    // Look, it works!
    .add_system(custom_base_set.in_base_set(MyBaseSet::Early))
    // Ordering your base sets relative to CoreSet is probably wise
    .configure_set(MyBaseSet::Early.before(CoreSet::Update))
    .configure_set(MyBaseSet::Late.after(CoreSet::Update));
```

Let me tell you a story, set in a world without **Base Sets**:

1. A new user adds the `make_player_run` system to their app.
2. Sometimes this system runs before input handling, leading to randomly dropped inputs. Sometimes it runs after rendering, leading to strange flickers.
3. After much frustration, the user discovers that these are due to "system execution order ambiguities".
4. The user runs a specialized detection tool, digs into the source code of the engine, figures out what order their system should run in relative to the engine's system sets, and then continues on their merry way, doing this for each new system.
5. Bevy (or one of their third-party plugins) updates, breaking all of our poor users system ordering once again.

The clear problem this illustrates is that _most_ gameplay systems should not need to know or care about "internal systems".

We've found that in practice, there are three broad classes of systems: gameplay logic (the majority of all end user systems), stuff that needs to happen before gameplay logic (like event cleanup and input handling), and stuff that needs to happen after gameplay logic (like rendering and audio).

By broadly ordering the schedule via **Base Sets**, Bevy apps can have good default behavior and clear high-level structure without compromising on the scheduling flexibility and explicitness that advanced users crave.
Let us know how it works out for you!

### Improved System Ambiguity Detection

When multiple systems interact with an ECS resource in conflicting ways, but don't have an ordering constraint between them, we call this an "ambiguity". If your [`App`] has ambiguities, this can cause bugs. We've significantly improved our ambiguity reporting, which can be configured in the new [`ScheduleBuildSettings`](https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/struct.ScheduleBuildSettings.html). Check out the docs for more info. If you haven't tried this out on your app yet: you should take a look!

### Single Threaded Execution

You can now easily switch a [`Schedule`] to single-threaded evaluation via the [`SingleThreadedExecutor`](https://docs.rs/bevy/0.10.0/bevy/ecs/schedule/struct.SingleThreadedExecutor.html) for users who don't want or need parallelism.

```rust
schedule.set_executor_kind(ExecutorKind::SingleThreaded);
```

## Cascaded Shadow Maps

<div class="release-feature-authors">authors: @danchia, Rob Swain (@superdump)</div>

Bevy uses "shadow maps" to cast shadows for lights / objects. Previous versions of Bevy used a simple but limited shadow map implementation for directional light sources. For a given light, you would define the resolution of the shadow map _and_ a manual "view projection" that would determine how the shadow is cast. This had a number of downsides:

* The resolution of the shadow map was fixed. You had to choose something between "cover a large area, but have a lower resolution" and "cover a smaller area, but have a higher resolution".
* The resolution didn't adapt to camera positioning. Shadows might look great in one position, but terrible in another position.
* The "shadow projection" had to be manually defined. This made it hard and unapproachable to configure shadows to match a given scene.

**Bevy 0.10** adds "cascaded shadow maps", which breaks up the camera's view frustum into a series of configurable "cascades", which each have their own shadow map. This enables shadows in the cascade "close to the camera" to be highly detailed, while allowing shadows "far from the camera" to cover a wider area with less detail. Because it uses the camera's view frustum to define the shadow projections, the shadow quality remains consistent as the camera moves through the scene. This also means that users don't need to manually configure shadow projections anymore. They are automatically calculated!

<video controls loop><source  src="shadow_cascades.mp4" type="video/mp4"/></video>

Notice how the nearby shadows are highly detailed whereas the shadows in the distance become less detailed as they get farther away (which doesn't matter as much because they are far away).

While shadow cascades solve important problems, they also introduce new ones. How many cascades should you use? What is the minimum and maximum distance from the camera where shadows should appear? How much overlap should there be between cascades? Be sure to dial in these parameters to fit your scenes.

## Environment Map Lighting

<div class="release-feature-authors">authors: @JMS55</div>

Environment maps are a popular and computationally cheap way to significantly improve the quality of a scene's lighting. It uses a cube map texture to provide 360 degree lighting "from all directions". This is especially apparent for reflective surfaces, but it applies to all lit materials.

This is what the PBR material looks like without environment map lighting:

![env map before](env_map_before.png)

And this is what the PBR material looks like with environment map lighting:

![env map after](env_map_after.png)

For scenes that need constant lighting (especially outdoor scenes), environment maps are a great solution. And because environment maps are arbitrary images, artists have a lot of control over the character of the scene's lighting.

## Depth and Normal Prepass

<div class="release-feature-authors">authors: @icesentry, Rob Swain (@superdump), @robtfm, @JMS55</div>

<video controls loop><source  src="force_field.mp4" type="video/mp4"/></video>
<p class="release-feature-authors">This effect uses the depth from the prepass to find the intersection between the ground and the force field</p>

Bevy now has the ability to run a depth and/or normal prepass. This means the depth and normal textures will be generated in a render pass that runs before the main pass and can therefore be used during the main pass. This enables various special effects like Screen Space Ambient Occlusion, Temporal Anti Aliasing, and many more. These are currently being worked on and should be [available in the next release of Bevy](#what-s-next).

![Edge detection](edge_detection.png)
<p class="release-feature-authors">In the image on the right, green lines are edges detected in the normal texture and blue lines are edges detected in the depth texture</p>

![Edge detection prepass](edge_detection_prepass.png)
<p class="release-feature-authors">The depth and normal textures generated by the prepass</p>

Using the prepass essentially means rendering everything twice. The prepass itself is much faster since it does a lot less work than the main pass. The result of the prepass can be used to reduce overdraw in the main pass, but if your scene didn't already suffer from overdraw then enabling the prepass will negatively affect performance.  There are many things that can be done to improve this and we will keep working towards this goal. Like with anything performance related, make sure to measure it for your use case and see if it helps or not.

The prepass is still very useful when working on special effects that require a depth or normal texture, so if you want to use it you can simply add the `DepthPrepass` or `NormalPrepass` components to your camera.

## Shadow Mapping using Prepass Shaders

<div class="release-feature-authors">authors: @geieredgar</div>

Previously, the shader used for shadow mapping was hard-coded and had no knowledge of the material, only meshes. Now in **Bevy 0.10**, a `Material`'s depth prepass shaders are used for shadow mapping. This means that the shaders used to do the shadow mapping for a `Material` are customizable!

As a bonus, the availability of `Material` information during shadow mapping means that we could instantly enable alpha mask shadows allowing foliage to cast shadows according to the alpha values in their texture rather than only based on their geometry.

![Alpha mask shadows](alpha_mask_shadows.png)
<div style="font-size: 1.0rem" class="release-feature-authors"><a href="https://developer.nvidia.com/orca/nvidia-emerald-square">NVIDIA ORCA Emerald Square scene</a> (<a href="https://creativecommons.org/licenses/by-nc-sa/3.0/">CC BY-NC-SA 3.0</a>) with alpha mask shadow support</div>

## Smooth Skeletal Animation Transitions

<div class="release-feature-authors">authors: @smessmer</div>

You can now smoothly transition between two (or more) skeletal animations!

<video controls loop><source  src="animation_transition.mp4" type="video/mp4"/></video>

<div style="font-size: 1.0rem" class="release-feature-authors">Character model and animations are royalty free assets from Mixamo.
</div>

With the new [`play_with_transition`] method on the [`AnimationPlayer`] component, you can now specify a transition duration during which the new animation will be linearly blended with the currently playing animation, whose weight will decrease during that duration until it reaches `0.0`.

```rust
#[derive(Component, Default)]
struct ActionTimer(Timer);

#[derive(Component)]
struct Animations {
    run: Handle<AnimationClip>,
    attack: Handle<AnimationClip>,
}

fn run_or_attack(
    mut query: Query<(&mut AnimationPlayer, &mut ActionTimer, &Animations)>,
    keyboard_input: Res<Input<KeyCode>>,
    animation_clips: Res<Assets<AnimationClip>>,
    time: Res<Time>,
) {
    for (mut animation_player, mut timer, animations) in query.iter_mut() {
        // Trigger the attack animation when pressing <space>
        if keyboard_input.just_pressed(KeyCode::Space) {
            let clip = animation_clips.get(&animations.attack).unwrap();
            // Set a timer for when to restart the run animation
            timer.0 = Timer::new(
                Duration::from_secs_f32(clip.duration() - 0.5),
                TimerMode::Once,
            );
            // Will transition over half a second to the attack animation
            animation_player
                .play_with_transition(animations.attack.clone(), Duration::from_secs_f32(0.5));
        }
        if timer.0.tick(time.delta()).just_finished() {
            // Once the attack animation is finished, restart the run animation
            animation_player
                .play_with_transition(animations.run.clone(), Duration::from_secs_f32(0.5))
                .repeat();
        }
    }
}
```

[`AnimationPlayer`]: https://docs.rs/bevy/0.10.0/bevy/animation/struct.AnimationPlayer.html
[`play_with_transition`]: https://docs.rs/bevy/0.10/bevy/animation/struct.AnimationPlayer.html#method.play_with_transition

## Improved Android Support

<div class="release-feature-authors">authors: @mockersf, @slyedoc</div>

![Android emulator running Bevy](android%20emulator.png)

Bevy now runs out of the box on Android on more devices. This was unlocked by waiting for the [`Resumed`](https://docs.rs/winit/0.28/winit/event/enum.Event.html#variant.Resumed) event to create the window instead of doing it on startup, matching the [`onResume()`](https://developer.android.com/guide/components/activities/activity-lifecycle#onresume) callback on Android.

To follow the recommendations on the [`Suspended`](https://docs.rs/winit/0.28/winit/event/enum.Event.html#variant.Suspended) event, Bevy will now exit on receiving that event. This is a temporary solution until Bevy is able to recreate rendering resources when being resumed.

Please test on your devices and report successes or issues you may encounter! There is a known issue around touch position on some devices with software buttons, as winit doesn't expose ([yet](https://github.com/rust-windowing/winit/issues/2308)) the inset size, only the inner size.

As this brings Bevy closer to full support of Android, there isn't a need anymore for separate examples for Android and iOS. They have been regrouped in one ["mobile" example](https://github.com/bevyengine/bevy/tree/v0.10.0/examples/mobile), and the instructions updated ([for Android](https://github.com/bevyengine/bevy/tree/v0.10.0/examples#android) and [for iOS](https://github.com/bevyengine/bevy/tree/v0.10.0/examples#ios)).

Here is the same example running on iOS!

![iOS emulator running Bevy](ios%20emulator.png)

## Revamped Bloom

<div class="release-feature-authors">authors: @StarLederer, @JMS55</div>

Bloom has undergone some major changes and now looks better, is easier to control, and has fewer visual artifacts.
In combination with the new tonemapping options, bloom has been much improved since the previous release!

1. In Bevy 0.9, bloom looked like this.
2. Switching the tonemapper to something like `AcesFitted` is already a big improvement.
3. In Bevy 0.10, bloom now looks like this. It's much more controlled and less overbearing.
4. To make the bloom stronger, rather than raise the `BloomSettings` intensity,
let's double the `emissive` value of each cube.
5. Finally, if you want more extreme bloom similar to the old algorithm, you can change
`BloomSettings::composite_mode` from `BloomCompositeMode::EnergyConserving` to `BloomCompositeMode::Additive`.
6. Explore the new bloom settings in an interactive playground using the new `bloom_3d` (and `bloom_2d`) examples.

<div style="display: grid; grid-template-columns: repeat(auto-fit, minmax(400px, 1fr)); grid-gap: 16px;">
     <div style="position: relative;">
        <div style="position: absolute; top: 0; left: 0; background-color: black; color: white; border-radius: 10px 0 10px 0; width: 30px; height: 30px; padding: 2px; box-shadow: 2px 2px 4px rgba(0,0,0,0.3); text-align: center; opacity: 0.8;">1</div>
        <img src="old_bloom.png" style="width: 100%; height: auto;"/>
     </div>
     <div style="position: relative;">
        <div style="position: absolute; top: 0; left: 0; background-color: black; color: white; border-radius: 10px 0 10px 0; width: 30px; height: 30px; padding: 2px; box-shadow: 2px 2px 4px rgba(0,0,0,0.3); text-align: center; opacity: 0.8;">2</div>
        <img src="old_bloom_aces.png" style="width: 100%; height: auto;"/>
     </div>
     <div style="position: relative;">
        <div style="position: absolute; top: 0; left: 0; background-color: black; color: white; border-radius: 10px 0 10px 0; width: 30px; height: 30px; padding: 2px; box-shadow: 2px 2px 4px rgba(0,0,0,0.3); text-align: center; opacity: 0.8;">3</div>
        <img src="new_bloom.png" style="width: 100%; height: auto;"/>
     </div>
     <div style="position: relative;">
        <div style="position: absolute; top: 0; left: 0; background-color: black; color: white; border-radius: 10px 0 10px 0; width: 30px; height: 30px; padding: 2px; box-shadow: 2px 2px 4px rgba(0,0,0,0.3); text-align: center; opacity: 0.8;">4</div>
        <img src="new_bloom_double_emission.png" style="width: 100%; height: auto;"/>
     </div>
     <div style="position: relative;">
        <div style="position: absolute; top: 0; left: 0; background-color: black; color: white; border-radius: 10px 0 10px 0; width: 30px; height: 30px; padding: 2px; box-shadow: 2px 2px 4px rgba(0,0,0,0.3); text-align: center; opacity: 0.8;">5</div>
        <img src="new_bloom_additive.png" style="width: 100%; height: auto;"/>
     </div>
     <div style="position: relative;">
        <div style="position: absolute; top: 0; left: 0; background-color: black; color: white; border-radius: 10px 0 10px 0; width: 30px; height: 30px; padding: 2px; box-shadow: 2px 2px 4px rgba(0,0,0,0.3); text-align: center; opacity: 0.8;">6</div>
        <img src="bloom_example.png" style="width: 100%; height: auto;"/>
     </div>
 </div>

## Distance and Atmospheric Fog

<div class="release-feature-authors">author: Marco Buono (@coreh)</div>

Bevy can now render distance and atmospheric fog effects, bringing a heightened sense of _depth_ and _ambiance_ to your scenes by making objects appear dimmer the further away they are from view.

![The new fog example showcases different fog modes and parameters.](fog.png)

Fog is controllable per camera via the new [`FogSettings`] component. Special care has been put into exposing several knobs to give you full artistic control over the look of your fog, including the ability to fade the fog in and out by controlling the alpha channel of the fog color.

```rust
commands.spawn((
    Camera3dBundle::default(),
    FogSettings {
        color: Color::rgba(0.1, 0.2, 0.4, 1.0),
        falloff: FogFalloff::Linear { start: 50.0, end: 100.0 },
    },
));
```

_Exactly how_ fog behaves with regard to distance is controlled via the [`FogFalloff`] enum. All of the “traditional” fog falloff modes from the fixed-function OpenGL 1.x / DirectX 7 days are supported:

`FogFalloff::Linear` increases in intensity linearly from 0 to 1 between `start` and `end` parameters. (This example uses values of 0.8 and 2.2, respectively.)

<svg width="370" height="212" viewBox="0 0 370 212" fill="none">
<path d="M331 151H42V49" stroke="currentColor" stroke-width="2"/>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-family="Inter" font-size="12" letter-spacing="0em"><tspan x="136" y="173.864">1</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-family="Inter" font-size="12" letter-spacing="0em"><tspan x="30" y="53.8636">1</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-family="Inter" font-size="12" letter-spacing="0em"><tspan x="42" y="173.864">0</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-family="Inter" font-size="12" letter-spacing="0em"><tspan x="232" y="173.864">2</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-family="Inter" font-size="12" letter-spacing="0em"><tspan x="332" y="173.864">3</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-family="Inter" font-size="12" letter-spacing="0em"><tspan x="161" y="190.864">distance</tspan></text>
<text font-family="sans-serif" transform="translate(10 132) rotate(-90)" fill="currentColor" style="white-space: pre" font-family="Inter" font-size="12" letter-spacing="0em"><tspan x="0" y="11.8636">fog intensity</tspan></text>
<path d="M43 150H117.227L263 48H331" stroke="#FF00E5"/>
<path d="M118 151V49" stroke="#FF00E5" stroke-dasharray="1 4"/>
<path d="M263 151V49" stroke="#FF00E5" stroke-dasharray="1 4"/>
<text font-family="sans-serif" fill="#FF00E5" style="white-space: pre" font-family="Inter" font-size="10" letter-spacing="0em"><tspan x="121" y="58.6364">start</tspan></text>
<text font-family="sans-serif" fill="#FF00E5" style="white-space: pre" font-family="Inter" font-size="10" letter-spacing="0em"><tspan x="267" y="58.6364">end</tspan></text>
</svg>

`FogFalloff::Exponential` increases according to an (inverse) exponential formula, controlled by a `density` parameter.

<svg width="370" height="212" viewBox="0 0 370 212" fill="none">
<mask id="mask0_3_31" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="42" y="42" width="286" height="108">
<rect x="42" y="42" width="286" height="108" fill="#D9D9D9"/>
</mask>
<g mask="url(#mask0_3_31)">
<path d="M42 150C42 150 98.3894 53 254.825 53L662 53" stroke="#FF003D" stroke-width="1"/>
<path d="M42 150C42 150 139.499 53 409.981 53L1114 53" stroke="#001AFF" stroke-width="1"/>
<path d="M42 150C42 150 206.348 53 662.281 53L1849 53" stroke="#14FF00" stroke-width="1"/>
</g>
<path d="M331 151H42V49" stroke="currentColor" stroke-width="2"/>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="136" y="173.864">1</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="30" y="53.8636">1</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="42" y="173.864">0</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="232" y="173.864">2</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="332" y="173.864">3</tspan></text>
<text font-family="sans-serif" fill="#FF003D" style="white-space: pre" font-size="10" letter-spacing="0em"><tspan x="77" y="64.6364">density = 2</tspan></text>
<text font-family="sans-serif" fill="#001AFF" style="white-space: pre" font-size="10" letter-spacing="0em"><tspan x="236" y="76.6364">density = 1</tspan></text>
<text font-family="sans-serif" fill="#14FF00" style="white-space: pre" font-size="10" letter-spacing="0em"><tspan x="205" y="115.636">density = 0.5</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="161" y="190.864">distance</tspan></text>
<text font-family="sans-serif" transform="translate(10 132) rotate(-90)" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="0" y="11.8636">fog intensity</tspan></text>
</svg>

`FogFalloff::ExponentialSquared` grows according to a slightly modified (inverse) exponential square formula, also controlled by a `density` parameter.

<svg width="370" height="212" viewBox="0 0 370 212" fill="none">
<mask id="mask0_1_3" style="mask-type:alpha" maskUnits="userSpaceOnUse" x="42" y="42" width="286" height="108">
<rect x="42" y="42" width="286" height="108" fill="#D9D9D9"/>
</mask>
<g mask="url(#mask0_1_3)">
<path d="M42 150C75.4552 150 74.9241 53.1724 166.262 53.1724L404 53.1724" stroke="#FF003D" stroke-width="1"/>
<path d="M42 150C107.986 150 106.939 53.1724 287.091 53.1724L756 53.1724" stroke="#001AFF" stroke-width="1"/>
<path d="M42 150C166.394 150 164.42 53.1724 504.035 53.1724L1388 53.1724" stroke="#14FF00" stroke-width="1"/>
</g>
<path d="M331 151H42V49" stroke="currentColor" stroke-width="2"/>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="136" y="173.864">1</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="30" y="53.8636">1</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="42" y="173.864">0</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="232" y="173.864">2</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="332" y="173.864">3</tspan></text>
<text font-family="sans-serif" fill="#FF003D" style="white-space: pre" font-size="10" letter-spacing="0em"><tspan x="61" y="54.6364">density = 2</tspan></text>
<text font-family="sans-serif" fill="#001AFF" style="white-space: pre" font-size="10" letter-spacing="0em"><tspan x="168" y="84.6364">density = 1</tspan></text>
<text font-family="sans-serif" fill="#14FF00" style="white-space: pre" font-size="10" letter-spacing="0em"><tspan x="174" y="121.636">density = 0.5</tspan></text>
<text font-family="sans-serif" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="161" y="190.864">distance</tspan></text>
<text font-family="sans-serif" transform="translate(10 132) rotate(-90)" fill="currentColor" style="white-space: pre" font-size="12" letter-spacing="0em"><tspan x="0" y="11.8636">fog intensity</tspan></text>
</svg>

Additionally, a more sophisticated `FogFalloff::Atmospheric` mode is available which provides _more physically accurate_ results by taking light `extinction` and `inscattering` into account separately.

[`DirectionalLight`] influence is also supported for all fog modes via the `directional_light_color` and `directional_light_exponent` parameters, mimicking the light dispersion effect seen in sunny outdoor environments.

![The new atmospheric_fog example showcases a terrain with atmospheric fog and directional light influence.](atmospheric-fog.png)

Since directly controlling the non-linear fog falloff parameters “by hand” can be tricky to get right, a number of helper functions based on [meteorological visibility](https://en.wikipedia.org/wiki/Visibility) are available, such as [`FogFalloff::from_visibility()`](https://docs.rs/bevy/0.10.0/bevy/pbr/enum.FogFalloff.html#method.from_visibility):

```rust
FogSettings {
    // objects retain visibility (>= 5% contrast) for up to 15 units
    falloff: FogFalloff::from_visibility(15.0),
    ..default()
}
```

Fog is applied “forward rendering-style” on the PBR fragment shader, instead of as a post-processing effect, which allows it to properly handle semi-transparent meshes.

The atmospheric fog implementation is largely based on [this great article](https://iquilezles.org/articles/fog/) by Inigo Quilez, Shadertoy co-creator, and computer graphics legend. _Thanks for the great write up and inspiration!_

[`FogSettings`]: https://docs.rs/bevy/0.10.0/bevy/pbr/struct.FogSettings.html
[`FogFalloff`]: https://docs.rs/bevy/0.10.0/bevy/pbr/enum.FogFalloff.html
[`DirectionalLight`]: https://docs.rs/bevy/0.10.0/bevy/pbr/struct.DirectionalLight.html

## StandardMaterial Blend Modes

<div class="release-feature-authors">author: Marco Buono (@coreh)</div>

The [`AlphaMode`] enum has been extended in **Bevy 0.10**, bringing support for _additive and multiplicative blending_ to the [`StandardMaterial`]. These two blend modes are staples of the “classic” (non physically-based) computer graphics toolbelt, and are commonly used to achieve a variety of effects.

<video controls loop><source src="demo-ruins.mp4" type="video/mp4"/></video>

_Demo showcasing the use of blend modes to create stained glass and fire effects._ (<a href="https://github.com/coreh/bevy-demo-ruins">Source Code</a>)

Additionally, support for semi-transparent textures with [premultiplied alpha](https://en.wikipedia.org/wiki/Alpha_compositing#Straight_versus_premultiplied) has been added, via a dedicated alpha mode.

Here's a high-level overview of the new modes:

* [`AlphaMode::Add`](https://docs.rs/bevy/0.10.0/bevy/pbr/enum.AlphaMode.html#variant.Add) — Combines the colors of the fragments with the colors behind them in an additive process, (i.e. like light) producing **brighter** results. Useful for effects like fire, holograms, ghosts, lasers and other energy beams. Also known as _Linear Dodge_ in graphics software.
* [`AlphaMode::Multiply`](https://docs.rs/bevy/0.10.0/bevy/pbr/enum.AlphaMode.html#variant.Multiply) — Combines the colors of the fragments with the colors behind them in a multiplicative process, (i.e. like pigments) producing **darker** results. Useful for effects approximating partial light transmission like stained glass, window tint film and some colored liquids.
* [`AlphaMode::Premultiplied`](https://docs.rs/bevy/0.10.0/bevy/pbr/enum.AlphaMode.html#variant.Premultiplied) — Behaves very similarly to [`AlphaMode::Blend`](https://docs.rs/bevy/0.10.0/bevy/pbr/enum.AlphaMode.html#variant.Blend), but assumes the color channels have **premultiplied alpha**. Can be used to avoid discolored “outline” artifacts that can occur when using plain alpha-blended textures, or to cleverly create materials that combine additive and regular alpha blending in a single texture, thanks to the fact that for otherwise constant RGB values, `Premultiplied` behaves more like `Blend` for alpha values closer to 1.0, and more like `Add` for alpha values closer to 0.0.

![The new blend_modes example.](blend-modes.png)

**Note:** Meshes using the new blend modes are drawn on the existing `Transparent3d` render phase, and therefore the same _z-sorting considerations/limitations_ from `AlphaMode::Blend` apply.

[`AlphaMode`]: https://docs.rs/bevy/0.10.0/bevy/pbr/enum.AlphaMode.html
[`StandardMaterial`]: https://docs.rs/bevy/0.10.0/bevy/pbr/struct.StandardMaterial.html

## More Tonemapping Choices

<div class="release-feature-authors">authors: @DGriffin91, @JMS55</div>

Tonemapping is the process of transforming raw High Dynamic Range (HDR) information into actual "screen colors" using a "display rendering transform" (DRT). In previous versions of Bevy you had exactly two tonemapping options: Reinhard Luminance or none at all. In **Bevy 0.10** we've added a ton of choices!

### No Tonemapping

This is generally not recommended as HDR lighting is not intended to be used as color.

![no tonemapping](tm_none.png)

### Reinhard

A simple method that adapts to the color in a scene: `r = color / (1.0 + color)`. Lots of hue shifting, brights don't desaturate naturally. Bright primaries and secondaries don't desaturate at all.

![reinhard](tm_reinhard.png)

### Reinhard Luminance

A popular method similar to normal Reinhard that incorporates luminance. It adapts to the amount of light in a scene. This is what we had in previous versions of Bevy. It is still our default algorithm, but this will likely change in the future. Hues shift. Brights don't desaturate much at all across the spectrum.

![reinhard luminance](tm_reinhard_luminance.png)

### ACES Fitted

An extremely popular algorithm used in film and industry (ex: ACES is the default Unreal tonemapping algorithm). When people say "filmic", this is often what they mean.

Not neutral, has a very specific aesthetic, intentional and dramatic hue shifting.
Bright greens and reds turn orange. Bright blues turn magenta. Significantly increased contrast. Brights desaturate across the spectrum.

![aces](tm_aces.png)

### AgX

Very neutral. Image is somewhat desaturated when compared to other transforms. Little to no hue shifting. Subtle [Abney shifting](https://en.wikipedia.org/wiki/Abney_effect). [Created by Troy Sobotka](https://github.com/sobotka/AgX)

![agx](tm_agx.png)

### Somewhat Boring Display Transform

Has little hue shifting in the darks and mids, but lots in the brights. Brights desaturate across the spectrum.
Is sort of between Reinhard and Reinhard Luminance. Conceptually similar to reinhard-jodie.
Designed as a compromise if you want e.g. decent skin tones in low light, but can't afford to re-do your
VFX to look good without hue shifting. Created by Tomasz Stachowiak.

![SomewhatBoringDisplayTransform](tm_sbdt.png)

### TonyMcMapface

Very neutral. Subtle but intentional hue shifting. Brights desaturate across the spectrum.

From the author: Tony is a display transform intended for real-time applications such as games.
It is intentionally boring, does not increase contrast or saturation, and stays close to the
input stimulus where compression isn't necessary.
Brightness-equivalent luminance of the input stimulus is compressed. The non-linearity resembles Reinhard.
Color hues are preserved during compression, except for a deliberate [Bezold–Brücke shift](https://en.wikipedia.org/wiki/Bezold%E2%80%93Br%C3%BCcke_shift).
To avoid posterization, selective desaturation is employed, with care to avoid the [Abney effect](https://en.wikipedia.org/wiki/Abney_effect). [Created by Tomasz Stachowiak](https://github.com/h3r2tic/tony-mc-mapface)

![TonyMcMapface](tm_tonymcmapface.png)

### Blender Filmic

Default Filmic Display Transform from Blender. Somewhat neutral. Hues shift. Brights desaturate across the spectrum.

![Blender Filmic](tm_blender_filmic.png)

## Color Grading Control

<div class="release-feature-authors">authors: @DGriffin91</div>

We've added some basic control over color grading parameters such as exposure, gamma, "pre-tonemapping saturation", and "post-tonemapping saturation". These can be configured per camera using the new [`ColorGrading`] component.

[`ColorGrading`]: https://docs.rs/bevy/0.10.0/bevy/render/view/struct.ColorGrading.html

### 0.5 Exposure

![0.5 exposure](exposure_005.png)

### 2.25 Exposure

![2.25 exposure](exposure_225.png)

## Parallel Pipelined Rendering

<div class="release-feature-authors">authors: @hymm, @james7132</div>

![Trace with Pipelined Rendering](pipelined-rendering-trace.png)

On multithreaded platforms, **Bevy 0.10** will now run significantly faster by running simulation and
rendering in parallel. The renderer was rearchitected in [Bevy 0.6](https://bevy.org/news/bevy-0-6/#pipelined-rendering-extract-prepare-queue-render)
to enable this, but the final step of actually running them in parallel was not done until now.
There was a bit of tricky work to figure out. The render world has a system that has to run on
the main thread, but the task pool only had the ability to run on the world's thread. So, when we send
the render world to another thread we need to accommodate still running render systems on the main
thread. To accomplish this, we added the ability to spawn tasks onto the main thread in addition to the world's thread.

![Histogram of Many Foxes Frame Time](pipelined-rendering-histogram.png)

In testing different Bevy examples, the gains were typically in the 10% to 30% range.
As seen in the above histogram, the mean frame time of the "many foxes" stress test
is 1.8ms faster than before.

To use pipelined rendering, you just need to add the [`PipelinedRenderingPlugin`]. If you're
using [`DefaultPlugins`] then it will automatically be added for you on all platforms except
wasm. Bevy does not currently support multithreading on wasm which is needed for this
feature to work. If you are not using [`DefaultPlugins`] you can add the plugin manually.

[`PipelinedRenderingPlugin`]: https://docs.rs/bevy/0.10.0/bevy/render/pipelined_rendering/struct.PipelinedRenderingPlugin.html
[`DefaultPlugins`]: https://docs.rs/bevy/0.10.0/bevy/struct.DefaultPlugins.html

## Windows as Entities

<div class="release-feature-authors">authors: @aceeri, @Weibye, @cart</div>

In previous versions of Bevy, [`Window`] was represented as an ECS resource (contained in the `Windows` resource). In **Bevy 0.10** [`Window`] is now a component (and therefore windows are represented as entities).

This accomplishes a number of goals:

* It opens the doors to representing Windows in Bevy's scene system
* It exposes `Windows` to Bevy's powerful ECS queries
* It provides granular per-window change detection
* Improves the readability/discoverability of creating, using, and closing windows
* Changing the properties of a window is the same for both initializing and modifying. No more `WindowDescriptor` fuss!
* It allows Bevy developers and users to easily attach new component data to windows

```rust
fn create_window(mut commands: Commands) {
    commands.spawn(Window {
        title: "My window :D".to_string(),
        ..default()
    });
}

fn modify_windows(mut windows: Query<&mut Window>) {
    for window in &mut windows {
        window.title = "My changed window! :D".to_string();
    }
}

fn close_windows(mut commands: Commands, windows: Query<Entity, With<Window>>) {
    for entity in &windows {
        commands.entity(entity).despawn();
    }
}
```

[`Window`]: https://docs.rs/bevy/0.10.0/bevy/window/struct.Window.html

## Renderer Optimizations

<div class="release-feature-authors">authors: @danchia, Rob Swain (@superdump), james7132, @kurtkuehnert, @robfm</div>

Bevy's renderer was ripe for optimization. So we optimized it!

The biggest bottleneck when rendering anything in Bevy is the final render stage, where we collect all of the data in the render world to issue draw calls to the GPU. The core loops here are extremely hot and any extra overhead is noticeable. In **Bevy 0.10**, we've thrown the kitchen sink at this problem and have attacked it from every angle. Overall, these following optimizations should make the render stage **2-3 times faster** than it was in 0.9:

* In [#7639](https://github.com/bevyengine/bevy/pull/7639) by @danchia, we found that even disabled logging has a strong impact on hot loops, netting us 20-50% speedups in the stage.
* In [#6944](https://github.com/bevyengine/bevy/pull/6944) by @james7132, we shrank the core data structures involved in the stage, reducing memory fetches and netting us 9% speedups.
* In [#6885](https://github.com/bevyengine/bevy/pull/6885) by @james7132, we rearchitected our `PhaseItem` and `RenderCommand` infrastructure to combine common operations when fetching component data from the `World`, netting us a 7% speedup.
* In [#7053](https://github.com/bevyengine/bevy/pull/7053) by @james7132, we changed `TrackedRenderPass`'s allocation patterns to minimize branching within these loops, netting a 6% speedup.
* In [#7084](https://github.com/bevyengine/bevy/pull/7084) by @james7132, we altered how we're fetching resources from the World to minimize the use of atomics in the stage, netting a 2% speedup.
* In [#6988](https://github.com/bevyengine/bevy/pull/6988) by @kurtkuehnert, we changed our internal resource IDs to use atomically incremented counters instead of UUIDs, reducing the comparison cost of some of the branches in the stage.

One other ongoing development is enabling the render stage to properly parallelize command encoding across multiple threads. Following [#7248](https://github.com/bevyengine/bevy/pull/7248) by @james7132, we now support ingesting externally created `CommandBuffer`s into the render graph, which should allow users to encode GPU commands in parallel and import them into the render graph. This is currently blocked by wgpu, which locks the GPU device when encoding render passes, but we should be able to support parallel command encoding as soon as that's addressed.

On a similar note, we've made steps to enable higher parallelism in other stages of the rendering pipeline. `PipelineCache` has been a resource that almost every Queue stage system needed to access mutably, but also only rarely needed to be written to. In [#7205](https://github.com/bevyengine/bevy/pull/7205), @danchia changed this to use internal mutability to allow for these systems to parallelize. This doesn't fully allow every system in this stage to parallelize just yet, as there still remain a few common blockers, but it should allow non-conflicting render phases to queue commands at the same time.

Optimization isn't all about CPU time! We've also improved memory usage, compile times, and GPU performance as well!

* We've also reduced the memory usage of `ComputedVisibility` by 50% thanks to @james7132. This was done by replacing the internal storage with a set of bitflags instead of multiple booleans.
* @robfm also used type erasure as a work-around a [rustc performance regression](https://github.com/rust-lang/rust/issues/99188) to ensure that rendering related crates have better compile times, with some of the crates compiling **up to 60% faster**! Full details can be seen in [#5950](https://github.com/bevyengine/bevy/pull/5950).
* In [#7069](https://github.com/bevyengine/bevy/pull/7069), Rob Swain (@superdump) reduced the number of active registers used on the GPU to prevent register spilling, significantly improving GPU-side performance.

Finally, we have made some improvements on specific usage scenarios:

* In [#6833](https://github.com/bevyengine/bevy/pull/6833), @james7132 improved the extraction of bones for mesh skinning by 40-50% by omitting an unnecessary buffer copy.
* In [#7311](https://github.com/bevyengine/bevy/pull/7311), @james7132 improved UI extraction by 33% by lifting a common computation out of a hot loop.

## Parallelized Transform Propagation and Animation Kinematics

<div class="release-feature-authors">authors: @james7132</div>

Transform propagation is one of the core systems of any game engine. If you move a parent entity, you expect its children to move in worldspace. Bevy's transform propagation system happens to be one of the largest bottlenecks for multiple systems: rendering, UI, physics, animation, etc. cannot run until it's complete. It's imperative that transform propagation is fast to avoid blocking all of these systems. In **Bevy 0.9** and before, transform propagation has always been single-threaded and always requires a full hierarchy traversal. As worlds got larger, so did the time spent in this key bottleneck. In **Bevy 0.10**, transform propagation leverages the structure of a well-formed hierarchy to fully run over multiple threads. The full performance benefits entirely depend on how the hierarchy is structured and how many CPU cores are available. In our testing, this has made transform propagation in our `many_foxes` benchmark **4 times faster** on our testing hardware.

If transform propagation can be parallelized, so can forward kinematics for animation. We leveraged the same guaranteed structure of well formed hierarchies to fully parallelize playing skeletal animations. We also enabled a basic entity-path cache lookup to reduce the extra lookups the system was doing. Altogether, we were able to make the animation player system on the same `many_foxes` benchmark **10 times faster**.

Combined with all of the other optimizations seen in this release, our tests on the `many_foxes` benchmark has sped up from ~10ms per frame (~100 FPS) to ~2.3ms per frame (~434 FPS), a near 5x speedup!

## ECS Optimizations

<div class="release-feature-authors">authors: @james7132, @JoJoJet</div>

ECS underlies the entire engine, so eliminating overhead in the ECS results in engine-wide speedups. In **Bevy 0.10**, we've found quite a few areas where we were able to massively reduce the overhead and improve CPU utilization for the entire engine.

In [#6547](https://github.com/bevyengine/bevy/pull/6547), we enabled [autovectorization](https://en.wikipedia.org/wiki/Automatic_vectorization) when using `Query::for_each`, and its parallel variants. Depending on the target architecture the engine is being compiled for, this can result in a 50-87.5% speed up in query iteration time. In 0.11, we may be extending this optimization to all iterator combinators based on `Iterator::fold`, such as `Iterator::count`. See [this PR](https://github.com/bevyengine/bevy/pull/6773) for more details.

In [#6681](https://github.com/bevyengine/bevy/pull/6681), by tightly packing entity location metadata and avoiding extra memory lookups, we've significantly reduced the overhead when making random query lookups via `Query::get`, seeing up to a 43% reduction in the overhead spent in `Query::get` and `World::get`.

In [#6800](https://github.com/bevyengine/bevy/pull/6800) and [#6902](https://github.com/bevyengine/bevy/pull/6902), we've found that rustc can optimize out compile-time constant branches across function boundaries, moving the branch from runtime to compile time, has resulted in up to a 50% reduction in overhead when using `EntityRef::get`, `EntityMut::insert`, `EntityMut::remove`, and their variants.

In [#6391](https://github.com/bevyengine/bevy/pull/6391), we've reworked `CommandQueue`'s internals to be more CPU-cache friendly, which has shown up to a 37% speedup when encoding and applying commands.

## `SystemParam` Improvements

<div class="release-feature-authors">authors: @JoJoJet</div>

Central to Bevy's ECS are `SystemParam`s: these types, such as `Query` and `Res`, dictate what a system can and can't do.
Previously, manually creating one required implementing a family of four inseparable traits.
In **Bevy 0.10**, we've [used generic associated types](https://github.com/bevyengine/bevy/pull/6865) to [reduce this to just two traits](https://github.com/bevyengine/bevy/pull/6919): `SystemParam` and `ReadOnlySystemParam`.

Additionally, the `#[derive(SystemParam)]` macro has received a host of miscellaneous usability improvements:

* **More Flexible**: you are no longer forced to declare lifetimes you don't use. Tuple structs are now allowed, and const generics don't break things.
* **Encapsulated**: a long-standing bug has been fixed that leaked the types of private fields. Now, `SystemParam`s can properly encapsulate private world data.
* **Limitless**: the 16-field limit has been lifted, so you can make your params as ridiculously complex as you want. This is most useful for generated code.

## Deferred World Mutations

<div class="release-feature-authors">authors: @JoJoJet</div>

You probably know that when you send a `Command`, it doesn't mutate the world right away. The command gets stored in the system and applied later on
in the schedule. Deferring mutations in this way has a few benefits:

* Minimizing world accesses: unlike mutable queries (and resources), deferred mutations are free from data access conflicts, which affords greater parallelizability to systems using this pattern.
* Order independence: when performing idempotent operations (like setting a global flag), deferred mutations allow you to not worry about system execution order.
* Structural mutations: deferred mutations are able to change the structure of the world in ways that `Query` and `ResMut` cannot, such as adding components or spawning and despawning entities.

**Bevy 0.10** adds first-class support for this pattern via the `Deferred` system parameter, which accepts a [`SystemBuffer`] trait impl. This lets you create systems with custom deferred mutation behavior while skipping the overhead associated with `Commands`!

```rust
/// Sends events with a delay, but can run in parallel with other event writers.
pub struct EventBuffer<E>(Vec<E>);

// The `SystemBuffer` trait controls how deferred mutations get applied to the world.
impl<E> SystemBuffer for EventBuffer<E> { ... }

fn my_system(mut events: Deferred<EventBuffer<MyEvent>>) {
    // Queue up an event to get sent when commands are applied.
    events.0.push(MyEvent);
}
```

Note that this feature should be used with care -- despite the potential performance benefits, inappropriate usage can actually _worsen_ performance. Any time you perform an optimization, make sure you check that it actually speeds things up!

[`SystemBuffer`]: https://docs.rs/bevy/0.10.0/bevy/ecs/system/trait.SystemBuffer.html

## Ref&lt;T&gt; Queries

<div class="release-feature-authors">authors: @Guvante, @JoJoJet</div>

Since Bevy 0.1, `Mut<T>` has been used to enable change detection (along with related types like `ResMut<T>`). It's a simple wrapper type that provides mutable access to a component alongside its change tick metadata, automatically marking a change when the value is mutated.

In **Bevy 0.10**, the change detection family has grown with `Ref<T>`, the immutable variant of `Mut<T>`. Like its mutable sibling, it allows you to react to changes made outside of the current system.

```rust
use bevy::prelude::*;

fn inspect_changes_system<T: Component + Debug>(q: Query<Ref<T>>) {
    // Iterate over each component of type `T` and log its changed status.
    for val in &q {
        if val.is_changed() {
            println!("Value `{val:?}` was last changed at tick {}.", val.last_changed());
        } else {
            println!("Value `{val:?}` is unchanged.");
        }
    }
}
```

We are also deprecating `ChangeTrackers<T>`, which is the old way of inspecting a component's change ticks. This type will be removed in the next version of Bevy.

## Cubic Curves

<div class="release-feature-authors">authors: @aevyrie</div>

<video controls loop><source  src="cubic_curves.mp4" type="video/mp4"/></video>
<p class="release-feature-authors">This video shows four kinds of cubic curves being smoothly animated with bezier easing. The curve itself is white, green is velocity, red is acceleration, and blue are the control points that determine the shape of the curve.</p>

In preparation for UI animation and hand-tweaked animation curves, cubic curves have been added to `bevy_math`. The implementation provides multiple curves out of the box, useful in various applications:

* `Bezier`: user-drawn splines, and cubic-bezier animation easing for UI - helper methods are provided for cubic animation easing as demonstrated in the above video.
* `Hermite`: smooth interpolation between two points in time where you know both the position and velocity, such as network prediction.
* `Cardinal`: easy interpolation between any number of control points, automatically computing tangents; Catmull-Rom is a type of Cardinal spline.
* `B-Spline`: acceleration-continuous motion, particularly useful for camera paths where a smooth change in velocity (acceleration) is important to prevent harsh jerking motion.

The `CubicGenerator` trait is public, allowing you to define your own custom splines that generate `CubicCurve`s!

### Performance

The position, velocity, and acceleration of a `CubicCurve` can be evaluated at any point. These evaluations all have the same performance cost, regardless of the type of cubic curve being used. On a modern CPU, these evaluations take 1-2 ns, and animation easing - which is an iterative process - takes 15-20 ns.

## AccessKit integration into `bevy_ui`

<div class="release-feature-authors">authors: @ndarilek</div>

Games are for everyone: and the way they're built should reflect that.
Accessible games are rare, and proper support is often an afterthought, both at an engine and a game level.
By building our UI solution with accessibility in mind, we hope to fix that.

Bevy has [joined `egui`](https://github.com/emilk/egui/pull/2294) in making the first steps towards cross-platform accessibility-by-default, with the help of the outstanding [AccessKit](https://github.com/AccessKit/accesskit) crate.
To our knowledge, this makes Bevy the first general purpose game engine with first-party accessibility support.

We've exposed Bevy's UI hierarchy and text elements to screen readers and other assistive devices, managed by the new on-by-default `bevy_a11y` crate.
This is ultimately powered by the new [`AccessibilityNode`](https://docs.rs/bevy/0.10.0/bevy/a11y/struct.AccessibilityNode.html) component, which combines with the existing hierarchy to expose this information directly to AccessKit and the [`Focus`](https://docs.rs/bevy/0.10.0/bevy/a11y/struct.Focus.html) resource, which stores the entity that has keyboard focus.

There's still a lot more to be done here: integrating the focus system with a [gamepad-driven UI controls](https://github.com/bevyengine/rfcs/pull/41) solution, cleaning up the data model to [make sure "accessible by default" is a reality](https://github.com/bevyengine/bevy/issues/7862)), and adding support for remaining features in AccessKit.

Special thanks to `@mwcampbell`, the lead author of AccessKit, for reviewing our integration and working with us to reduce the number of dependencies upstream, [substantially improving both compile times and final executable size](https://github.com/bevyengine/bevy/pull/6874#issuecomment-1440978453). This is [still a serious challenge on Linux](https://github.com/bevyengine/bevy/pull/6874#issuecomment-1432144117), and so the `accesskit_unix` feature flag is [disabled by default for now](https://github.com/bevyengine/bevy/pull/6874#issuecomment-1433896811).

## Spatial Audio

<div class="release-feature-authors">authors: @mockersf, @DGriffin91, @harudagondi, @alice-i-cecile</div>

The library Bevy uses for audio, [`rodio`], contains support for spatial audio. Bevy 0.10 exposes basic spatial audio. There are still a few caveats, like no HRTF and no first class support for `Emitter` and `Listener` components.

Interestingly, during the development of this specific feature, `@harudagondi` found a [bug][reverse-channels-bug] where the audio channels reverse when running the app in either debug or release mode. This turns out to be a `rodio` issue, and this also affects previous versions of Bevy. Thanks to `@dis-da-moe`, the bug has been [fixed upstream][rodio-pr]. See the linked PR for interesting details about audio programming quirks and performance issues.

You can now have spatial audio in your game! Clone the `bevy` repository and invoke `cargo run --example spatial_audio_3d --release` in the command line for a showcase of 3D spatial audio in Bevy.

[`rodio`]: https://crates.io/crates/rodio
[reverse-channels-bug]: https://github.com/RustAudio/rodio/issues/444
[rodio-pr]: https://github.com/RustAudio/rodio/pull/455

## Custom Audio Sources

<div class="release-feature-authors">authors: @dis-da-moe</div>

Bevy supports custom audio sources through the [`Decodable`] trait, but the way to register to the bevy app is very boilerplatey and sparsely documented. In **Bevy 0.10**, a new extension trait for `App` is added and the documentation for [`Decodable`] has vastly improved.

As such, instead of doing this:

```rust
struct MyCustomAudioSource { /* ... */ }

app.add_asset::<MyCustomAudioSource>()
    .init_resource::<Audio<MyCustomAudioSource>>()
    .init_resource::<AudioOutput<MyCustomAudioSource>>()
    .add_system(play_queued_audio_system::<MyCustomAudioSource>.in_base_set(CoreSet::PostUpdate))
```

You only have to do this:

```rust
app.add_audio_source::<MyCustomAudioSource>()
```

Much cleaner!

[`Decodable`]: https://docs.rs/bevy_audio/latest/bevy_audio/trait.Decodable.html

## ShaderDef Values

<div class="release-feature-authors">authors: @mockersf</div>

Bevy's shader processor now supports ShaderDefs with values, using the new [`ShaderDefVal`]. This allows developers to pass constant values into their shaders:

```rust
let shader_defs = vec![
    ShaderDefVal::Int("MAX_DIRECTIONAL_LIGHTS".to_string(), 10),
];
```

These can be used in `#if` statements to selectively enable shader code based on the value:

```rust
#if MAX_DIRECTIONAL_LIGHTS >= 10
let color = vec4<f32>(1.0, 0.0, 0.0, 1.0);
#else
let color = vec4<f32>(0.0, 1.0, 0.0, 1.0);
#endif
```

ShaderDef values can be inlined into shaders:

```rust
for (var i: u32 = 0u; i < #{MAX_DIRECTIONAL_LIGHTS}; i = i + 1u) {
}
```

They can also be defined inline in shaders:

```rust
#define MAX_DIRECTIONAL_LIGHTS 10
```

ShaderDefs defined in shaders override values passed in from Bevy.

[`ShaderDefVal`]: https://docs.rs/bevy/0.10.0/bevy/render/render_resource/enum.ShaderDefVal.html

## `#else ifdef` Chains in Shaders

<div class="release-feature-authors">authors: @torsteingrindvik</div>

Bevy's shader processor now also supports `#else ifdef` chains like this:

```rust
#ifdef FOO
// foo code
#else ifdef BAR
// bar code
#else ifdef BAZ
// baz code
#else
// fallback code
#endif
```

## New Shader Imports: Global and View

<div class="release-feature-authors">authors: @torsteingrindvik</div>

The `Global` and `View` structs are now importable in shaders using `#import bevy_render::globals` and `#import bevy_render::view`. Bevy's internal shaders now use these imports (saving a lot of redundancy). Previously you either needed to re-define in each shader or import the larger `bevy_pbr::mesh_view_types` (which wasn't always what was needed).

Previously this was needed:

```rust
struct View {
    view_proj: mat4x4<f32>,
    inverse_view_proj: mat4x4<f32>,
    view: mat4x4<f32>,
    inverse_view: mat4x4<f32>,
    projection: mat4x4<f32>,
    inverse_projection: mat4x4<f32>,
    world_position: vec3<f32>,
    // viewport(x_origin, y_origin, width, height)
    viewport: vec4<f32>,
};
```

Now you can just do this!

```rust
#import bevy_render::view
```

## Adaptive Batching for Parallel Query Iteration

<div class="release-feature-authors">authors: @james7132</div>

`Query::par_for_each` has been the tool everyone reaches for when their queries get too big to run single-threaded. Got 100,000 entities running around on your screen? No problem, `Query::par_for_each` chunks it up into smaller batches and distributes the workload over multiple threads. However, in **Bevy 0.9** and before, `Query::par_for_each` required callers to provide a batch size to help tune these batches for maximum performance. This rather opaque knob often resulted in users just randomly picking a value and rolling with it, or fine tuning the value based on their development machines. Unfortunately, the most effective value is dependent on the runtime environment (i.e. how many logical cores does a player's computer have) and the state of the ECS World (i.e. how many entities are matched?). Ultimately most users of the API just chose a flat number and lived with the results, good or bad.

```rust
// 0.9
const QUERY_BATCH_SIZE: usize = 32;

query.par_for_each(QUERY_BATCH_SIZE, |mut component| {
   // ...
});
```

In 0.10, you no longer need to provide a batch size! If you use [`Query::par_iter`], Bevy will automatically evaluate the state of the World and task pools and select a batch size [using a heuristic](https://github.com/bevyengine/bevy/blob/43ea6f239deefd7a497da6ef581a05a63a278605/crates/bevy_ecs/src/query/par_iter.rs#L24) to ensure sufficient parallelism, without incurring too much overhead. This makes parallel queries as easy to use as normal single-threaded queries! While great for most typical use cases, these heuristics may not be suitable for every workload, so we've provided an escape hatch for those who need finer control over the workload distribution. In the future, we may further tune the backing heuristics to try to get the default to be closer to optimal in these workloads.

```rust
// 0.10
query.par_iter().for_each(|component| {
   // ...
});

// Fairly easy to convert from a single-threaded for_each. Just change iter to par_iter!
query.iter().for_each(|component| {
   // ...
});
```

You can also use [`BatchingStrategy`] for more control over batching:

```rust
query
    .par_iter_mut()
    // run with batches of 100
    .batching_strategy(BatchingStrategy::fixed(100))
    .for_each(|mut component| { /* ... */ });
```

See the [`BatchingStrategy`] docs for more info.

[`Query::par_iter`]: https://docs.rs/bevy/0.10.0/bevy/ecs/system/struct.Query.html#method.par_iter
[`BatchingStrategy`]: https://docs.rs/bevy/0.10.0/bevy/ecs/query/struct.BatchingStrategy.html

## `UnsafeWorldCell` and `UnsafeEntityCell`

<div class="release-feature-authors">authors: @jakobhellermann, @BoxyUwU and @JoJoJet</div>

`UnsafeWorldCell` and `UnsafeEntityCell` allow shared mutable access to parts of the world via unsafe code. It serves a similar purpose as `UnsafeCell`, allowing people to build interior mutability abstractions such as `Cell` `Mutex` `Channel` etc. In bevy `UnsafeWorldCell` will be used to support the scheduler and system param implementations as these are interior mutability abstractions for `World`, it also currently is used to implement `WorldCell`. We're planning to use `UnsafeEntityCell` to implement versions of `EntityRef`/`EntityMut` that only have access to the components on the entity rather than the entire world.

These abstractions were introduced in [#6404](https://github.com/bevyengine/bevy/pull/6404), [#7381](https://github.com/bevyengine/bevy/pull/7381) and [#7568](https://github.com/bevyengine/bevy/pull/7568).

## Cylinder Shape

<div class="release-feature-authors">authors: @JayPavlina, @rparrett, @davidhof</div>

The cylinder shape primitive has joined our zoo of built-in shapes!

![primitive shapes](primitive_shapes.png)

## Subdividable Plane Shape

<div class="release-feature-authors">authors: @woodroww</div>

Bevy's [`Plane`] shape can now be subdivided any number of times.

![plane](plane.png)

[`Plane`]: https://docs.rs/bevy/0.10.0/bevy/prelude/shape/struct.Plane.html

## Camera Output Modes

<div class="release-feature-authors">authors: @cart, @robtfm</div>

The [camera-driven](/news/bevy-0-8/#camera-driven-rendering) post-processing features [added in Bevy 0.9](/news/bevy-0-9/#hdr-post-processing-tonemapping-and-bloom) add intuitive control over post-processing across multiple cameras in a scene, but there [were a few corner cases](https://github.com/bevyengine/bevy/pull/7490) that didn't _quite_ fit into the hard-coded camera output model. And there were some bugs and limitations related to double-buffered target texture sources of truth being incorrect across cameras and MSAA's sampled texture not containing what it should under some circumstances.

**Bevy 0.10** adds a [`CameraOutputMode`] field to [`Camera`], which gives Bevy app developers the ability to manually configure exactly how (and if) a [`Camera`]'s render results should be written to the final output texture:

```rust
// Configure the camera to write to the final output texture
camera.output_mode = CameraOutputMode::Write {
    // Do not blend with the current state of the output texture
    blend_state: None,
    // Clear the output texture
    color_attachment_load_op: LoadOp::Clear(Default::default()),
};

// Configure the camera to skip writing to the final output texture
// This can save a pass when there are multiple cameras, and can be useful for
// some post-processing situations
camera.output_mode = CameraOutputMode::Skip;
```

_Most_ single-camera and multi-camera setups will not need to touch this setting at all. But if you need it, it will be waiting for you!

MSAA requires an extra intermediate "multisampled" texture, which gets resolved to the "actual" unsampled texture. In some corner case multi-camera setups that render to the same texture, this can create weird / inconsistent results based on whether or not MSAA is enabled or disabled. We've added a new `Camera::msaa_writeback` `bool` field which (when enabled) will write the current state of the unsampled texture to the intermediate MSAA texture (if a previous camera has already rendered to the target on a given frame). This ensures that the state is consistent regardless of MSAA configuration. This defaults to true, so you only need to think about this if you have a multi-camera setup and you _don't_ want MSAA writeback.

[`CameraOutputMode`]: https://docs.rs/bevy/0.10.0/bevy/render/camera/enum.CameraOutputMode.html
[`Camera`]: https://docs.rs/bevy/0.10.0/bevy/render/camera/struct.Camera.html

## Configurable Visibility Component

<div class="release-feature-authors">authors: @ickk</div>

The [`Visibility`] component controls whether or not an [`Entity`] should be rendered. **Bevy 0.10** reworked the type definition: rather than having a single `is_visible: bool` field, we now use an enum with an additional mode:

```rust
pub enum Visibility {
  Hidden,    // unconditionally hidden
  Visible,   // unconditionally visible
  Inherited, // inherit visibility from parent
}
```

Much easier to understand! In previous Bevy versions, "inherited visibility" and "hidden" were essentially the only two options. Now entities can opt to be visible, even if their parent is hidden!

[`Visibility`]: https://docs.rs/bevy/0.10.0/bevy/render/view/enum.Visibility.html
[`Entity`]: https://docs.rs/bevy/0.10.0/bevy/ecs/entity/index.html

## `AsBindGroup` Storage Buffers

<div class="release-feature-authors">authors: @IceSentry, @AndrewB330</div>

[`AsBindGroup`] is a useful Bevy trait that [makes it very easy to pass data into shaders](/news/bevy-0-8/#new-material-system).

**Bevy 0.10** expands this with support for "storage buffer bindings", which are very useful when passing in large / unbounded chunks of data:

```rust
#[derive(AsBindGroup)]
struct CoolMaterial {
    #[uniform(0)]
    color: Color,
    #[texture(1)]
    #[sampler(2)]
    color_texture: Handle<Image>,
    #[storage(3)]
    values: Vec<f32>,
    #[storage(4, read_only, buffer)]
    buffer: Buffer,
}
```

[`AsBindGroup`]: https://docs.rs/bevy/0.10.0/bevy/render/render_resource/trait.AsBindGroup.html

## `ExtractComponent` Derive

<div class="release-feature-authors">authors: @torsteingrindvik</div>

To pass component data from the "main app" to the "render app" for [pipelined rendering](#parallel-pipelined-rendering), we run an "extract step". The [`ExtractComponent`] trait is used to copy data over. In previous versions of Bevy, you had to implement it manually, but now you can derive it!

```rust
#[derive(Component, Clone, ExtractComponent)]
pub struct Car {
    pub wheels: usize,
}
```

This expands to this:

```rust
impl ExtractComponent for Car
{
    type Query = &'static Self;
    type Filter = ();
    type Out = Self;
    fn extract_component(item: QueryItem<'_, Self::Query>) -> Option<Self::Out> {
        Some(item.clone())
    }
}
```

It also supports filters!

```rust
#[derive(Component, Clone, ExtractComponent)]
#[extract_component_filter(With<Fuel>)]
pub struct Car {
    pub wheels: usize,
}
```

[`ExtractComponent`]: https://docs.rs/bevy/0.10.0/bevy/render/extract_component/trait.ExtractComponent.html

## Upgraded wgpu to 0.15

<div class="release-feature-authors">authors: @Elabajaba</div>

**Bevy 0.10** now uses the latest and greatest [`wgpu`](https://github.com/gfx-rs/wgpu) (our choice of low level graphics layer). In addition to [a number of nice API improvements and bug fixes](https://github.com/gfx-rs/wgpu/releases/tag/v0.15.0), `wgpu` now uses the DXC shader compiler for DX12, which is faster, less buggy, and allows for new features.

## Enabled OpenGL Backend by Default

<div class="release-feature-authors">authors: @wangling12</div>

Bevy has supported `wgpu`'s OpenGL backend for a while now, but it was opt-in. This caused Bevy to fail to start up on some machines that don't support modern apis like Vulkan. In **Bevy 0.10** the OpenGL backend is enabled by default, which means machines will automatically fall back to OpenGL if no other API is available.

## Exposed Non-Uniform Indexing Support (Bindless)

<div class="release-feature-authors">authors: @cryscan</div>

**Bevy 0.10** wired up initial support for non-uniform indexing of textures and storage buffers. This is an important step toward modern ["bindless / gpu-driven rendering"](https://vkguide.dev/docs/gpudriven/gpu_driven_engines/), which can unlock significant performance on platforms that support it. Note that this is just making the feature available to render plugin developers. Bevy's core rendering features do not (yet) use the bindless approach.

We've added [a new example](https://github.com/bevyengine/bevy/blob/v0.10.0/examples/shader/texture_binding_array.rs) illustrating how to use this feature:

![texture binding array](texture_binding_array.png)

## Gamepad API Improvements

<div class="release-feature-authors">authors: @DevinLeamy</div>

The [`GamepadEventRaw`] type has been removed in favor of separate [`GamepadConnectionEvent`], [`GamepadAxisChangedEvent`], and [`GamepadButtonChangedEvent`], and the internals have been reworked to accommodate this.

This allows for simpler, more granular event access without filtering down the general [`GamepadEvent`] type. Nice!

```rust
fn system(mut events: EventReader<GamepadConnectionEvent>)
    for event in events.iter() {
    }
}
```

[`GamepadEventRaw`]: https://docs.rs/bevy/0.9.0/bevy/input/gamepad/struct.GamepadEventRaw.html
[`GamepadConnectionEvent`]: https://docs.rs/bevy/0.10.0/bevy/input/gamepad/struct.GamepadConnectionEvent.html
[`GamepadAxisChangedEvent`]: https://docs.rs/bevy/0.10.0/bevy/input/gamepad/struct.GamepadAxisChangedEvent.html
[`GamepadButtonChangedEvent`]: https://docs.rs/bevy/0.10.0/bevy/input/gamepad/struct.GamepadButtonChangedEvent.html
[`GamepadEvent`]: https://docs.rs/bevy/0.10.0/bevy/input/gamepad/enum.GamepadEvent.html

## Input Method Editor (IME) Support

<div class="release-feature-authors">authors: @mockersf</div>

[`Window`] can now configure IME support using `ime_enabled` and `ime_position`, which enables the use of "dead keys", which add support for French, Pinyin, etc:

<video controls loop><source  src="ime.mp4" type="video/mp4"/></video>

## Reflection Paths: Enums and Tuples

<div class="release-feature-authors">authors: @MrGVSV</div>

Bevy's "reflection paths" enable navigating Rust values using a simple (and dynamic) string syntax. **Bevy 0.10** expands this system by adding support for tuples and enums in reflect paths:

```rust
#[derive(Reflect)]
struct MyStruct {
  data: Data,
  some_tuple: (u32, u32),
}

#[derive(Reflect)]
enum Data {
  Foo(u32, u32),
  Bar(bool)
}

let x = MyStruct {
  data: Data::Foo(123),
  some_tuple: (10, 20),
};

assert_eq!(*x.path::<u32>("data.1").unwrap(), 123);
assert_eq!(*x.path::<u32>("some_tuple.0").unwrap(), 10);
```

## Pre-Parsed Reflection Paths

<div class="release-feature-authors">authors: @MrGVSV, @james7132 </div>

Reflection paths enable a lot of interesting and dynamic editor scenarios, but they do have a downside: calling `path()` requires parsing strings every time. To solve this problem we added [`ParsedPath`], which enables pre-parsing paths and then reusing those results on each access:

```rust
let parsed_path = ParsedPath::parse("foo.bar[0]").unwrap();
let element = parsed_path.element::<usize>(&some_value);
```

Much more suitable for repeated access, such as doing the same lookup every frame!

[`ParsedPath`]: https://docs.rs/bevy/0.10.0/bevy/reflect/struct.ParsedPath.html

## `ReflectFromReflect`

<div class="release-feature-authors">authors: @MrGVSV</div>

When using Bevy's Rust reflection system, we sometimes end up in a scenario where we have a "dynamic reflect value" representing a certain type `MyType` (even though under the hood, it isn't really that type). Such scenarios happen when we call `Reflect::clone_value`, use the reflection deserializers, or create the dynamic value ourselves. Unfortunately, we can't just call `MyType::from_reflect` as we do not have knowledge of the concrete `MyType` at runtime.

[`ReflectFromReflect`] is a new "type data" struct in the [`TypeRegistry`] that enables `FromReflect` trait operations without any concrete references to a given type. Very cool!

```rust
#[derive(Reflect, FromReflect)]
#[reflect(FromReflect)] // <- Register `ReflectFromReflect`
struct MyStruct(String);

let type_id = TypeId::of::<MyStruct>();

// Register our type
let mut registry = TypeRegistry::default();
registry.register::<MyStruct>();

// Create a concrete instance
let my_struct = MyStruct("Hello world".to_string());

// `Reflect::clone_value` will generate a `DynamicTupleStruct` for tuple struct types
// Note that this is _not_ a MyStruct instance
let dynamic_value: Box<dyn Reflect> = my_struct.clone_value();

// Get the `ReflectFromReflect` type data from the registry
let rfr: &ReflectFromReflect = registry
  .get_type_data::<ReflectFromReflect>(type_id)
  .unwrap();

// Call `FromReflect::from_reflect` on our Dynamic value
let concrete_value: Box<dyn Reflect> = rfr.from_reflect(&dynamic_value);
assert!(concrete_value.is::<MyStruct>());
```

[`ReflectFromReflect`]: https://docs.rs/bevy/0.10.0/bevy/reflect/struct.ReflectFromReflect.html
[`TypeRegistry`]: https://docs.rs/bevy/0.10.0/bevy/reflect/struct.TypeRegistry.html

## Other Reflection Improvements

<div class="release-feature-authors">authors: @james7132, @soqb, @cBournhonesque, @SkiFire13</div>

* [`Reflect`] is now implemented for [`std::collections::VecDeque`]
* Reflected [`List`] types now have `insert` and `remove` operations
* Reflected [`Map`] types now have the `remove` operation
* Reflected generic types now automatically implement [`Reflect`] if the generics also implement Reflect. No need to add manual `T: Reflect` bounds!
* Component Reflection now uses [`EntityRef`] / [`EntityMut`] instead of both [`World`] and [`Entity`], which allows it to be used in more scenarios
* The Reflection deserializer now avoids unnecessarily cloning strings in some scenarios!

[`std::collections::VecDeque`]: https://doc.rust-lang.org/std/collections/vec_deque/struct.VecDeque.html
[`List`]: https://docs.rs/bevy/0.10.0/bevy/reflect/trait.List.html
[`Map`]: https://docs.rs/bevy/0.10.0/bevy/reflect/trait.Map.html
[`Reflect`]: https://docs.rs/bevy/0.10.0/bevy/reflect/trait.Reflect.html
[`EntityRef`]: https://docs.rs/bevy/0.10.0/bevy/ecs/world/struct.EntityRef.html
[`EntityMut`]: https://docs.rs/bevy/0.10.0/bevy/ecs/world/struct.EntityMut.html
[`World`]: https://docs.rs/bevy/0.10.0/bevy/ecs/world/struct.World.html

## Upgraded Taffy To 0.3

<div class="release-feature-authors">authors: @ickshonpe, @rparret</div>

[Taffy](https://crates.io/crates/taffy) is the library we use to compute layouts for `bevy_ui`. Taffy 0.2 significantly improves the performance of nested UIs (our `many_buttons` example is now 8% faster and more deeply nested UIs should see even bigger gains!). It also brings support for the [gap](https://developer.mozilla.org/en-US/docs/Web/CSS/gap) property which makes it easier to creates UI with evenly spaced items. Taffy 0.3 adds some nice API tweaks (and also a grid layout feature, which we have disabled for now as it still needs some integration work).

## Relative Cursor Position

<div class="release-feature-authors">authors: @Pietrek14</div>

We've added a new [`RelativeCursorPosition`] UI component, which when added to a UI entity tracks the cursor position relative to the node. `Some((0, 0))` represents the top-left corner of the node, `Some((1,1))` represents the bottom-right corner of the node, and `None` represents the cursor being "outside of the node".

```rust
commands.spawn((
    NodeBundle::default(),
    RelativeCursorPosition::default(),
));
```

[`RelativeCursorPosition`]: https://docs.rs/bevy/0.10.0/bevy/ui/struct.RelativeCursorPosition.html

## Const Bevy UI Defaults

<div class="release-feature-authors">authors: @james-j-obrien</div>

Bevy uses the [`Default`] trait a lot to make it easy to construct types. Bevy UI types generally implement [`Default`]. However, it has one downside (which is fundamental to Rust): [`Default`] cannot be used in `const` contexts ([yet!](https://blog.rust-lang.org/inside-rust/2022/07/27/keyword-generics.html)). To enable UI layout config to be defined as constants, we've added `DEFAULT` associated constants to most of the Bevy UI types. For example, you can use `Style::DEFAULT` to define a const style:

```rust
const COOL_STYLE: Style = Style {
    size: Size::width(Val::Px(200.0)),
    border: UiRect::all(Val::Px(2.0)),
    ..Style::DEFAULT
};
```

[`Default`]: https://doc.rust-lang.org/std/default/trait.Default.html

## Iterating through a World's Entities

<div class="release-feature-authors">authors: @james7132</div>

In **Bevy 0.9**, `World::iter_entities` allows users to get an iterator over all of the entities in the `World` in `Entity` form. In **Bevy 0.10**, this has been changed to be an iterator over `EntityRef`, which gives full read-only access to all of the entity's components instead of just getting its ID. Its new implementation should also be significantly faster than fetching the `EntityRef` by hand (though note that a `Query` will still be faster if you know the exact components you're looking for). This gives users free rein to arbitrarily read any entity data from the World, and may see use in scripting language integrations and reflection-heavy workflows.

```rust
// Bevy 0.9
for entity in world.iter_entities() {
   if let Some(entity_ref) = world.get_entity(entity) {
      if let Some(component) = entity_ref.get::<MyComponent>() {
         ...
      }
   }
}

// Bevy 0.10
for entity_ref in world.iter_entities() {
   if let Some(component) = entity_ref.get::<MyComponent>() {
      ...
   }
}
```

In the future, we may have a `World::iter_entities_mut` that exposes this functionality, but gives arbitrary mutable access to all entities in the `World`. We avoided implementing this for now due to the potential safety concerns of returning an iterator of `EntityMut`. For more details, see this [GitHub issue](https://github.com/bevyengine/bevy/issues/5504).

## LCH Color Space

<div class="release-feature-authors">authors: @ldubos</div>

Bevy's [`Color`] type now supports the LCH color space (Lightness, Chroma, Hue). LCH has a lot of arguments for it, including that it provides access to about 50% more colors over sRGB. Check out [this article](https://lea.verou.me/2020/04/lch-colors-in-css-what-why-and-how/) for more information.

```rust
Color::Lcha {
    lightness: 1.0,
    chroma: 0.5,
    hue: 200.0,
    alpha: 1.0,
}
```

[`Color`]: https://docs.rs/bevy/0.10.0/bevy/render/color/enum.Color.html

## Optimized `Color::hex` Performance

<div class="release-feature-authors">authors: @wyhaya</div>

[`Color::hex`](https://docs.rs/bevy/0.10.0/bevy/render/color/enum.Color.html#method.hex) is now a `const` function, which brought the runtime of `hex` from ~14ns to ~4ns!

## Split Up `CorePlugin`

<div class="release-feature-authors">authors: @targrub</div>

`CorePlugin` has historically been a bit of a "kitchen sink plugin". "Core" things that didn't fit anywhere else ended up there. This isn't a great organizational strategy, so we broke it up into individual pieces: [`TaskPoolPlugin`], [`TypeRegistrationPlugin`], and [`FrameCountPlugin`].

[`TaskPoolPlugin`]: https://docs.rs/bevy/0.10.0/bevy/core/struct.TaskPoolPlugin.html
[`TypeRegistrationPlugin`]: https://docs.rs/bevy/0.10.0/bevy/core/struct.TypeRegistrationPlugin.html
[`FrameCountPlugin`]: https://docs.rs/bevy/0.10.0/bevy/core/struct.FrameCountPlugin.html

## `EntityCommand`s

<div class="release-feature-authors">authors: @JoJoJet</div>

[`Commands`] are "deferred ECS" operations. They enable developers to define custom ECS operations that are applied after a parallel system has finished running. Many [`Commands`] ran on individual entities, but this pattern was a bit cumbersome:

```rust
struct MyCustomCommand(Entity);

impl Command for MyCustomCommand {
    fn write(self, world: &mut World) {
        // do something with the entity at self.0
    }
}

let id = commands.spawn(SpriteBundle::default()).id();
commands.add(MyCustomCommand(id));
```

To solve this, in **Bevy 0.10** we added the [`EntityCommand`] trait. This allows the command to be ergonomically applied to spawned entities:

```rust
struct MyCustomCommand;

impl EntityCommand for MyCustomCommand {
    fn write(self, id: Entity, world: &mut World) {
        // do something with the given entity id
    }
}

commands.spawn(SpriteBundle::default()).add(MyCustomCommand);
```

[`EntityCommand`]: https://docs.rs/bevy/0.10.0/bevy/ecs/system/trait.EntityCommand.html
[`Commands`]: https://docs.rs/bevy/0.10.0/bevy/ecs/system/struct.Commands.html

## Pixel Perfect Example

<div class="release-feature-authors">authors: @Ian-Yy</div>

We now have a new ["pixel perfect" example](https://github.com/bevyengine/bevy/blob/v0.10.0/examples/2d/pixel_perfect.rs) that illustrates how to set up pixel-perfect sprites. It uses a cute new Bevy logo sprite!

![pixel perfect](pixel_perfect.png)

## UI Text Layout Example

<div class="release-feature-authors">authors: @ickshonpe</div>

We've added a nice ["text layout" example](https://github.com/bevyengine/bevy/blob/v0.10.0/examples/ui/text_layout.rs) that illustrates the various Bevy UI text layout settings:

![text layout](text_layout.png)

## CI Improvements

<div class="release-feature-authors">authors: @mockersf</div>

We take CI pretty seriously in Bevy land and we're always on the lookout for new ways to make our lives better. We made a number of nice improvements this cycle:

* We now set an MSRV (minimum supported Rust version) for the `bevy` crate and we have a CI job that checks the MSRV
* CI gives new contributors a friendly welcome message!
* CI now asks for a migration guide when a PR is labeled as a breaking change and no migration guide is present

## The First Subject Matter Expert Release

This was our first release using our new [Subject Matter Expert (SME) system](/news/scaling-bevy-development/). We merged an absolutely massive amount of changes, and this was _despite_ our Project Lead `@cart` being away for about a month for Christmas and snowboarding vacations. We maintained a high quality bar and built amazing things. Suffice it to say the future is looking bright (and sustainable)! Stay tuned for more SME appointments in more areas.

## <a name="what-s-next"></a>What's Next?

* **Asset System Evolution**: We've made good progress on the [next iteration of the Bevy Asset System](https://github.com/bevyengine/bevy/discussions/3972), which will add the ability to preprocess assets and improve the flexibility and usability of the asset system.
* **Kicking off the Bevy Editor Effort**: We are ready to start shifting our focus to building out the Bevy Editor! We've started [gathering requirements](https://github.com/bevyengine/bevy/discussions/7100) and would like to start the initial design phase in the **Bevy 0.11** cycle.  
* **Temporal Anti-Aliasing (TAA)**: We've largely implemented TAA, which uses motion vectors and time to produce a very popular screen space anti-aliasing effect.
* **Screen Space Ambient Occlusion (SSAO)**: This is a popular, relatively cheap illumination technique that can make scenes look much more natural. It builds on the Depth Prepass work.
* **Automated Render Batching and Instancing**: Automatically cut down draw calls by combining geometry or using instancing. This will enable Bevy to render hundreds of thousands of objects without grinding to a halt. We technically already support this, but it must be implemented manually outside of our standard pipeline. This will bring batching and instancing wins "for free" in our built-in render pipeline.
* **One-shot systems:** Run arbitrary systems in a [push-based fashion via commands](https://github.com/bevyengine/bevy/issues/2192), and store them as callback components for ultra-flexible behavior customization.
* **Better plugins:** Clearer and more standardized tools for [adapting third-party plugins to your app's unique architecture](https://github.com/bevyengine/bevy/issues/2160), eliminating [order-dependence in their initialization](https://github.com/bevyengine/bevy/issues/1255) and defining [dependencies](https://github.com/bevyengine/bevy/issues/69) between them.
* **Pull `!Send` data out of the `World`:** storing non thread-safe data in a structure designed to be sent across threads has caused us no end of headaches. We plan on pulling these out into the `App`, resolving a major blocker for a first-class [multiple worlds](https://github.com/bevyengine/rfcs/pull/43) design.
* **Timestamp window and input events:** As discussed in [#5984](https://github.com/bevyengine/bevy/issues/5984), tracking the exact timing of input events is essential to ensuring that event ordering and timing can be precisely reconstructed.
* **Opt-out change detection:** improve performance for tiny components by [turning off change detection at compile or run-time](https://github.com/bevyengine/bevy/issues/4882).
* **Comprehensive Animation Composition:** Supporting non-transitional animation composition (i.e. arbitrary weighted blending of animations). For more complete information, see the [RFC](https://github.com/bevyengine/rfcs/pull/51).

Check out the [**Bevy 0.11 Milestone**](https://github.com/bevyengine/bevy/milestone/11) for an up-to-date list of current work being considered for **Bevy 0.11**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the 173 contributors that made this release (and associated docs) possible! In random order:

* @killercup
* @torsteingrindvik
* @utilForever
* @garychia
* @lewiszlw
* @myreprise1
* @tomaspecl
* @jinleili
* @nicopap
* @edgarssilva
* @aevyrie
* @laundmo
* @AxiomaticSemantics
* @polygon
* @SkiFire13
* @SludgePhD
* @abnormalbrain
* @Testare
* @ldubos
* @SpeedRoll
* @rodolphito
* @hymm
* @rdbo
* @AndrewB330
* @13ros27
* @lupan
* @iwek7
* @ErickMVdO
* @kerkmann
* @davidhof
* @Pietrek14
* @Guvante
* @lidong63
* @Tirthnp
* @x-52
* @Suficio
* @pascualex
* @xgbwei
* @BoxyUwU
* @superdump
* @TheRawMeatball
* @wackbyte
* @StarLederer
* @MrGunflame
* @akimakinai
* @doup
* @komadori
* @darthdeus
* @phuocthanhdo
* @DanielJin21
* @LiamGallagher737
* @oliviacrain
* @IceSentry
* @Vrixyz
* @johanhelsing
* @Dessix
* @woodroww
* @SDesya74
* @alphastrata
* @wyhaya
* @foxzool
* @DasLixou
* @nakedible
* @soqb
* @Dorumin
* @maniwani
* @Elabajaba
* @geieredgar
* @stephenmartindale
* @TimJentzsch
* @holyfight6
* @targrub
* @smessmer
* @redwarp
* @LoopyAshy
* @mareq
* @bjrnt
* @slyedoc
* @kurtkuehnert
* @Charles Bournhonesque
* @cryscan
* @A-Walrus
* @JMS55
* @cBournhonesque
* @SpecificProtagonist
* @Shatur
* @VitalyAnkh
* @aktaboot
* @dis-da-moe
* @chrisjuchem
* @wilk10
* @2ne1ugly
* @zeroacez
* @jabuwu
* @Aceeri
* @coreh
* @SuperSodaSea
* @DGriffin91
* @DanielHZhang
* @mnmaita
* @elbertronnie
* @Zeenobit
* @oCaioOliveira
* @Sjael
* @JonahPlusPlus
* @devmitch
* @alice-i-cecile
* @remiCzn
* @Sasy00
* @sQu1rr
* @Ptipiak
* @zardini123
* @alradish
* @adam-shih
* @LinusKall
* @jakobhellermann
* @Andrii Borziak
* @figsoda
* @james7132
* @l1npengtul
* @danchia
* @AjaxGb
* @VVishion
* @CatThingy
* @zxygentoo
* @nfagerlund
* @silvestrpredko
* @ameknite
* @shuoli84
* @CrystaLamb
* @Nanox19435
* @james-j-obrien
* @mockersf
* @R2Boyo25
* @NeoRaider
* @MrGVSV
* @GuillaumeGomez
* @wangling12
* @AndrewJakubowicz
* @rick68
* @RedMachete
* @tbillington
* @ndarilek
* @Ian-Yy
* @Edwox
* @DevinLeamy
* @TehPers
* @cart
* @mvlabat
* @NiklasEi
* @ItsDoot
* @JayPavlina
* @ickk
* @Molot2032
* @devil-ira
* @inodentry
* @MinerSebas
* @JoJoJet
* @Neo-Zhixing
* @rparrett
* @djeedai
* @Pixelstormer
* @iiYese
* @harudagondi
* @1e1001
* @ickshonpe
* @rezural
* @arewerage
* @ld000
* @imustend
* @robtfm
* @frewsxcv

## Full Changelog

### Added

* [Accessibility: Added `Label` for marking text specifically as a label for UI controls.][6874]
* [Accessibility: Integrate with and expose AccessKit accessibility.][6874]
* [App: `App::setup`][7586]
* [App: `SubApp::new`][7290]
* [App: Bevy apps will now log system information on startup by default][5454]
* [Audio Expose symphonia features from rodio in bevy_audio and bevy][6388]
* [Audio: Basic spatial audio][6028]
* [ECS: `bevy_ptr::dangling_with_align`: creates a well-aligned dangling pointer to a type whose alignment is not known at compile time.][6618]
* [ECS: `Column::get_added_ticks`][6547]
* [ECS: `Column::get_column_ticks`][6547]
* [ECS: `DetectChanges::set_if_neq`: triggering change detection when the new and previous values are equal. This will work on both components and resources.][6853]
* [ECS: `SparseSet::get_added_ticks`][6547]
* [ECS: `SparseSet::get_column_ticks`][6547]
* [ECS: `Tick`, a wrapper around a single change detection tick.][6547]
* [ECS: `UnsafeWorldCell::world_mut` now exists and can be used to get a `&mut World` out of `UnsafeWorldCell`][7381]
* [ECS: `WorldId` now implements the `FromWorld` trait.][7726]
* [ECS: A `core::fmt::Pointer` impl to `Ptr`, `PtrMut` and `OwnedPtr`.][6980]
* [ECS: Add `bevy_ecs::schedule_v3` module][6587]
* [ECS: Add `EntityMap::iter()`][6935]
* [ECS: Add `Ref` to the prelude][7392]
* [ECS: Add `report_sets` option to `ScheduleBuildSettings`][7756]
* [ECS: add `Resources::iter` to iterate over all resource IDs][6592]
* [ECS: add `UnsafeWorldCell` abstraction][6404]
* [ECS: Add `World::clear_resources` & `World::clear_all`][3212]
* [ECS: Add a basic example for system ordering][7017]
* [ECS: Add a missing impl of `ReadOnlySystemParam` for `Option<NonSend<>>`][7245]
* [ECS: add a spawn_on_external method to allow spawning on the scope’s thread or an external thread][7415]
* [ECS: Add const `Entity::PLACEHOLDER`][6761]
* [ECS: Add example to show how to use `apply_system_buffers`][7793]
* [ECS: Add logging variants of system piping][6751]
* [ECS: Add safe constructors for untyped pointers `Ptr` and `PtrMut`][6539]
* [ECS: Add unit test with system that panics][7491]
* [ECS: Add wrapping_add to change_tick][7146]
* [ECS: Added “base sets” and ported CoreSet to use them.][7466]
* [ECS: Added `as_mut` and `as_ref` methods to `MutUntyped`.][7009]
* [ECS: Added `bevy::ecs::system::assert_is_read_only_system`.][7547]
* [ECS: Added `Components::resource_id`.][7284]
* [ECS: Added `DebugName` world query for more human friendly debug names of entities.][7186]
* [ECS: Added `distributive_run_if` to `IntoSystemConfigs` to enable adding a run condition to each system when using `add_systems`.][7724]
* [ECS: Added `EntityLocation::table_id`][6681]
* [ECS: Added `EntityLocation::table_row`.][6681]
* [ECS: Added `IntoIterator` implementation for `EventReader` so you can now do `&mut reader` instead of `reader.iter()` for events.][7720]
* [ECS: Added `len`, `is_empty`, `iter` methods on SparseSets.][7638]
* [ECS: Added `ManualEventReader::clear()`][7471]
* [ECS: Added `MutUntyped::with_type` which allows converting into a `Mut<T>`][7113]
* [ECS: Added `new_for_test` on `ComponentInfo` to make test code easy.][7638]
* [ECS: Added `not` condition.][7559]
* [ECS: Added `on_timer` and `on_fixed_timer` run conditions][7866]
* [ECS: Added `OwningPtr::read_unaligned`.][7039]
* [ECS: Added `ReadOnlySystem`, which is implemented for any `System` type whose parameters all implement `ReadOnlySystemParam`.][7547]
* [ECS: Added `Ref` which allows inspecting change detection flags in an immutable way][7097]
* [ECS: Added `shrink` and `as_ref` methods to `PtrMut`.][7009]
* [ECS: Added `SystemMeta::name`][6900]
* [ECS: Added `SystemState::get_manual_mut`][7084]
* [ECS: Added `SystemState::get_manual`][7084]
* [ECS: Added `SystemState::update_archetypes`][7084]
* [ECS: Added a large number of methods on `App` to work with schedules ergonomically][7267]
* [ECS: Added conversions from `Ptr`, `PtrMut`, and `OwningPtr` to `NonNull<u8>`.][7181]
* [ECS: Added rore common run conditions: `on_event`, resource change detection, `state_changed`, `any_with_component`][7579]
* [ECS: Added support for variants of `bevy_ptr` types that do not require being correctly aligned for the pointee type.][7151]
* [ECS: Added the `CoreSchedule` enum][7267]
* [ECS: Added the `SystemParam` type `Deferred<T>`, which can be used to defer `World` mutations. Powered by the new trait `SystemBuffer`.][6817]
* [ECS: Added the extension methods `.and_then(...)` and `.or_else(...)` to run conditions, which allows combining run conditions with short-circuiting behavior.][7605]
* [ECS: Added the marker trait `BaseSystemSet`, which is distinguished from a `FreeSystemSet`. These are both subtraits of `SystemSet`.][7863]
* [ECS: Added the method `reborrow` to `Mut`, `ResMut`, `NonSendMut`, and `MutUntyped`.][7114]
* [ECS: Added the private `prepare_view_uniforms` system now has a public system set for scheduling purposes, called `ViewSet::PrepareUniforms`][7267]
* [ECS: Added the trait `Combine`, which can be used with the new `CombinatorSystem` to create system combinators with custom behavior.][7605]
* [ECS: Added the trait `EntityCommand`. This is a counterpart of `Command` for types that execute code for a single entity.][7015]
* [ECS: introduce EntityLocation::INVALID const and adjust Entities::get comment][7623]
* [ECS: States derive macro][7535]
* [ECS: support for tuple structs and unit structs to the `SystemParam` derive macro.][6957]
* [Hierarchy: Add `Transform::look_to`][6692]
* [Hierarchy: Added `add_child`, `set_parent` and `remove_parent` to `EntityMut`][6926]
* [Hierarchy: Added `clear_children(&mut self) -> &mut Self` and `replace_children(&mut self, children: &[Entity]) -> &mut Self` function in `BuildChildren` trait][6035]
* [Hierarchy: Added `ClearChildren` and `ReplaceChildren` struct][6035]
* [Hierarchy: Added `push_and_replace_children_commands` and `push_and_clear_children_commands` test][6035]
* [Hierarchy: Added the `BuildChildrenTransformExt` trait][7024]
* [Input: add Input Method Editor support][7325]
* [Input: Added `Axis<T>::devices`][5400]
* [INput: Added common run conditions for `bevy_input`][7806]
* [Macro: add helper for macro to get either bevy::x or bevy_x depending on how it was imported][7164]
* [Math: `CubicBezier2d`, `CubicBezier3d`, `QuadraticBezier2d`, and `QuadraticBezier3d` types with methods for sampling position, velocity, and acceleration. The generic `Bezier` type is also available, and generic over any degree of Bezier curve.][7653]
* [Math: `CubicBezierEasing`, with additional methods to allow for smooth easing animations.][7653]
* [Math: Added a generic cubic curve trait, and implementation for Cardinal splines (including Catmull-Rom), B-Splines, Beziers, and Hermite Splines. 2D cubic curve segments also implement easing functionality for animation.][7683]
* [New reflection path syntax: struct field access by index (example syntax: `foo#1`)][7321]
* [Reflect  `State` generics other than just `RandomState` can now be reflected for both `hashbrown::HashMap` and `collections::HashMap`][7782]
* [Reflect: `Aabb` now implements `FromReflect`.][7396]
* [Reflect: `derive(Reflect)` now supports structs and enums that contain generic types][7364]
* [Reflect: `ParsedPath` for cached reflection paths][7321]
* [Reflect: `std::collections::HashMap` can now be reflected][7782]
* [Reflect: `std::collections::VecDeque` now implements `Reflect` and all relevant traits.][6831]
* [Reflect: Add reflection path support for `Tuple` types][7324]
* [Reflect: Added `ArrayIter::new`.][7449]
* [Reflect: Added `FromReflect::take_from_reflect`][6566]
* [Reflect: Added `List::insert` and `List::remove`.][7063]
* [Reflect: Added `Map::remove`][6564]
* [Reflect: Added `ReflectFromReflect`][6245]
* [Reflect: Added `TypeRegistrationDeserializer`, which simplifies getting a `&TypeRegistration` while deserializing a string.][7094]
* [Reflect: Added methods to `List` that were previously provided by `Array`][7467]
* [Reflect: Added support for enums in reflection paths][6560]
* [Reflect: Added the `bevy_reflect_compile_fail_tests` crate for testing compilation errors][7041]
* [Reflect: bevy_reflect: Add missing primitive registrations][7815]
* [Reflect: impl `Reflect` for `&'static Path`][6755]
* [Reflect: implement `Reflect` for `Fxaa`][7527]
* [Reflect: implement `TypeUuid` for primitives and fix multiple-parameter generics having the same `TypeUuid`][6633]
* [Reflect: Implemented `Reflect` + `FromReflect` for window events and related types. These types are automatically registered when adding the `WindowPlugin`.][6235]
* [Reflect: Register Hash for glam types][6786]
* [Reflect: Register missing reflected types for `bevy_render`][6811]
* [Render: A pub field `extras` to `GltfNode`/`GltfMesh`/`GltfPrimitive` which store extras][6973]
* [Render: A pub field `material_extras` to `GltfPrimitive` which store material extras][6973]
* [Render: Add 'Color::as_lcha' function (#7757)][7766]
* [Render: Add `Camera::viewport_to_world_2d`][6557]
* [Render: Add a more familiar hex color entry][7060]
* [Render: add ambient lighting hook][5428]
* [Render: Add bevy logo to the lighting example to demo alpha mask shadows][7895]
* [Render: Add Box::from_corners method][6672]
* [Render: add OpenGL and DX11 backends][7481]
* [Render: Add orthographic camera support back to directional shadows][7796]
* [Render: add standard material depth bias to pipeline][7847]
* [Render: Add support for Rgb9e5Ufloat textures][6781]
* [Render: Added buffer usage field to buffers][7423]
* [Render: can define a value from inside a shader][7518]
* [Render: EnvironmentMapLight support for WebGL2][7737]
* [Render: Implement `ReadOnlySystemParam` for `Extract<>`][7182]
* [Render: Initial tonemapping options][7594]
* [Render: ShaderDefVal: add an `UInt` option][6881]
* [Render: Support raw buffers in AsBindGroup macro][7701]
* [Rendering: `Aabb` now implements `Copy`.][7401]
* [Rendering: `ExtractComponent` can specify output type, and outputting is optional.][6699]
* [Rendering: `Mssaa::samples`][7292]
* [Rendering: Add `#else ifdef` to shader preprocessing.][7431]
* [Rendering: Add a field `push_constant_ranges` to RenderPipelineDescriptor and ComputePipelineDescriptor][7681]
* [Rendering: Added  `Material::prepass_vertex_shader()` and `Material::prepass_fragment_shader()` to control the prepass from the `Material`][6284]
* [Rendering: Added `BloomSettings:lf_boost`, `BloomSettings:lf_boost_curvature`, `BloomSettings::high_pass_frequency` and `BloomSettings::composite_mode`.][6677]
* [Rendering: Added `BufferVec::extend`][6833]
* [Rendering: Added `BufferVec::truncate`][6833]
* [Rendering: Added `Camera::msaa_writeback` which can enable and disable msaa writeback.][7671]
* [Rendering: Added `CascadeShadowConfigBuilder` to help with creating `CascadeShadowConfig`][7456]
* [Rendering: Added `DepthPrepass` and `NormalPrepass` component to control which textures will be created by the prepass and available in later passes.][6284]
* [Rendering: Added `Draw<T>::prepare` optional trait function.][6885]
* [Rendering: Added `DrawFunctionsInternals::id()`][6745]
* [Rendering: Added `FallbackImageCubemap`.][7051]
* [Rendering: Added `FogFalloff` enum for selecting between three widely used “traditional” fog falloff modes: `Linear`, `Exponential` and `ExponentialSquared`, as well as a more advanced `Atmospheric` fog;][6412]
* [Rendering: Added `get_input_node`][6720]
* [Rendering: Added `Lcha` member to `bevy_render::color::Color` enum][7483]
* [Rendering: Added `MainTaret::main_texture_other`][7343]
* [Rendering: Added `PhaseItem::entity`][6885]
* [Rendering: Added `prepass_enabled` flag to the `MaterialPlugin` that will control if a material uses the prepass or not.][6284]
* [Rendering: Added `prepass_enabled` flag to the `PbrPlugin` to control if the StandardMaterial uses the prepass. Currently defaults to false.][6284]
* [Rendering: Added `PrepassNode` that runs before the main pass][6284]
* [Rendering: Added `PrepassPlugin` to extract/prepare/queue the necessary data][6284]
* [Rendering: Added `RenderCommand::ItemorldQuery` associated type.][6885]
* [Rendering: Added `RenderCommand::ViewWorldQuery` associated type.][6885]
* [Rendering: Added `RenderContext::add_command_buffer`][7248]
* [Rendering: Added `RenderContext::begin_tracked_render_pass`.][7053]
* [Rendering: Added `RenderContext::finish`][7248]
* [Rendering: Added `RenderContext::new`][7248]
* [Rendering: Added `SortedCameras`, exposing information that was previously internal to the camera driver node.][7671]
* [Rendering: Added `try_add_node_edge`][6720]
* [Rendering: Added `try_add_slot_edge`][6720]
* [Rendering: Added `with_r`, `with_g`, `with_b`, and `with_a` to `Color`.][6899]
* [Rendering: Added 2x and 8x sample counts for MSAA.][7684]
* [Rendering: Added a `#[storage(index)]` attribute to the derive `AsBindGroup` macro.][6129]
* [Rendering: Added an `EnvironmentMapLight` camera component that adds additional ambient light to a scene.][7051]
* [Rendering: Added argument to `ScalingMode::WindowSize` that specifies the number of pixels that equals one world unit.][6201]
* [Rendering: Added cylinder shape][6809]
* [Rendering: Added example `shaders/texture_binding_array`.][6995]
* [Rendering: Added new capabilities for shader validation.][6995]
* [Rendering: Added specializable `BlitPipeline` and ported the upscaling node to use this.][7671]
* [Rendering: Added subdivisions field to shape::Plane][7546]
* [Rendering: Added support for additive and multiplicative blend modes in the PBR `StandardMaterial`, via `AlphaMode::Add` and `AlphaMode::Multiply`;][6644]
* [Rendering: Added support for distance-based fog effects for PBR materials, controllable per-camera via the new `FogSettings` component;][6412]
* [Rendering: Added support for KTX2 `R8_SRGB`, `R8_UNORM`, `R8G8_SRGB`, `R8G8_UNORM`, `R8G8B8_SRGB`, `R8G8B8_UNORM` formats by converting to supported wgpu formats as appropriate][4594]
* [Rendering: Added support for premultiplied alpha in the PBR `StandardMaterial`, via `AlphaMode::Premultiplied`;][6644]
* [Rendering: Added the ability to `#[derive(ExtractComponent)]` with an optional filter.][7399]
* [Rendering: Added: `bevy_render::color::LchRepresentation` struct][7483]
* [Rendering: Clone impl for MaterialPipeline][7548]
* [Rendering: Implemented `Clone` for all pipeline types.][6653]
* [Rendering: Smooth Transition between Animations][6922]
* [Support optional env variable `BEVY_ASSET_ROOT` to explicitly specify root assets directory.][5346]
* [Task: Add thread create/destroy callbacks to TaskPool][6561]
* [Tasks: Added `ThreadExecutor` that can only be ticked on one thread.][7087]
* [the extension methods `in_schedule(label)` and  `on_startup()` for configuring the schedule a system belongs to.][7790]
* [Transform: Added `GlobalTransform::reparented_to`][7020]
* [UI: `Size::new` is now `const`][6602]
* [UI: Add const to methods and const defaults to bevy_ui][5542]
* [UI: Added `all`, `width` and `height` functions to `Size`.][7468]
* [UI: Added `Anchor` component to `Text2dBundle`][6807]
* [UI: Added `CalculatedSize::preserve_aspect_ratio`][6825]
* [UI: Added `Component` derive to `Anchor`][6807]
* [UI: Added `RelativeCursorPosition`, and an example showcasing it][7199]
* [UI: Added `Text::with_linebreak_behaviour`][7283]
* [UI: Added `TextBundle::with_linebreak_behaviour`][7283]
* [UI: Added a `BackgroundColor` component to `TextBundle`.][7596]
* [UI: Added a helper method `with_background_color` to `TextBundle`.][7596]
* [UI: Added the `SpaceEvenly` variant to `AlignContent`.][7859]
* [UI: Added the `Start` and `End` variants to `AlignItems`, `AlignSelf`, `AlignContent` and `JustifyContent`.][7859]
* [UI: Adds `flip_x` and `flip_y` fields to `ExtractedUiNode`.][6292]
* [Utils: Added `SyncCell::read`, which allows shared access to values that already implement the `Sync` trait.][7718]
* [Utils: Added the guard type `bevy_utils::OnDrop`.][7181]
* [Window: Add `Windows::get_focused(_mut)`][6571]
* [Window: add span to winit event handler][6612]
* [Window: Transparent window on macos][7617]
* [Windowing: `WindowDescriptor` renamed to `Window`.][5589]
* [Windowing: Added `hittest` to `WindowAttributes`][6664]
* [Windowing: Added `Window::prevent_default_event_handling` . This allows bevy apps to not override default browser behavior on hotkeys like F5, F12, Ctrl+R etc.][7304]
* [Windowing: Added `WindowDescriptor.always_on_top` which configures a window to stay on top.][6527]
* [Windowing: Added an example `cargo run --example fallthrough`][6664]
* [Windowing: Added the `hittest`’s setters/getters][6664]
* [Windowing: Modifed the `WindowDescriptor`’s `Default` impl.][6664]
* [Windowing: Modified the `WindowBuilder`][6664]

### Changed

* [Animation: `AnimationPlayer` that are on a child or descendant of another entity with another player will no longer be run.][6785]
* [Animation: Animation sampling now runs fully multi-threaded using threads from `ComputeTaskPool`.][6785]
* [App: Adapt path type of dynamically_load_plugin][6734]
* [App: Break CorePlugin into TaskPoolPlugin, TypeRegistrationPlugin, FrameCountPlugin.][7083]
* [App: Increment FrameCount in CoreStage::Last.][7477]
* [App::run() will now panic when called from Plugin::build()][4241]
* [Asset: `AssetIo::watch_path_for_changes` allows watched path and path to reload to differ][6797]
* [Asset: make HandleUntyped::id private][7076]
* [Audio: `AudioOutput` is now a `Resource`. It's no longer `!Send`][6436]
* [Audio: AudioOutput is actually a normal resource now, not a non-send resource][7262]
* [ECS: `.label(SystemLabel)` is now referred to as `.in_set(SystemSet)`][7267]
* [ECS: `App::add_default_labels` is now `App::add_default_sets`][7267]
* [ECS: `App::add_system_set` was renamed to `App::add_systems`][7267]
* [ECS: `Archetype` indices and `Table` rows have been newtyped as `ArchetypeRow` and `TableRow`.][4878]
* [ECS: `ArchetypeGeneration` now implements `Ord` and `PartialOrd`.][6742]
* [ECS: `bevy_pbr::add_clusters` is no longer an exclusive system][7267]
* [ECS: `Bundle::get_components` now takes a `FnMut(StorageType, OwningPtr)`. The provided storage type must be correct for the component being fetched.][6902]
* [ECS: `ChangeTrackers<T>` has been deprecated. It will be removed in Bevy 0.11.][7306]
* [ECS: `Command` closures no longer need to implement the marker trait `std::marker::Sync`.][7014]
* [ECS: `CoreStage` and `StartupStage` enums are now `CoreSet` and `StartupSet`][7267]
* [ECS: `EntityMut::world_scope` now allows returning a value from the immediately-computed closure.][7385]
* [ECS: `EntityMut`: rename `remove_intersection` to `remove` and `remove` to `take`][7810]
* [ECS: `EventReader::clear` now takes a mutable reference instead of consuming the event reader.][6851]
* [ECS: `EventWriter::send_batch` will only log a TRACE level log if the batch is non-empty.][7753]
* [ECS: `oldest_id` and `get_event` convenience methods added to `Events<T>`.][5735]
* [ECS: `OwningPtr::drop_as` will now panic in debug builds if the pointer is not aligned.][7117]
* [ECS: `OwningPtr::read` will now panic in debug builds if the pointer is not aligned.][7117]
* [ECS: `Ptr::deref` will now panic in debug builds if the pointer is not aligned.][7117]
* [ECS: `PtrMut::deref_mut` will now panic in debug builds if the pointer is not aligned.][7117]
* [ECS: `Query::par_for_each(_mut)` has been changed to `Query::par_iter(_mut)` and will now automatically try to produce a batch size for callers based on the current `World` state.][4777]
* [ECS: `RemovedComponents` now internally uses an `Events<RemovedComponentsEntity>` instead of an `Events<Entity>`][7503]
* [ECS: `SceneSpawnerSystem` now runs under `CoreSet::Update`, rather than `CoreStage::PreUpdate.at_end()`.][7267]
* [ECS: `StartupSet` is now a base set][7574]
* [ECS: `System::default_labels` is now `System::default_system_sets`.][7267]
* [ECS: `SystemLabel` trait was replaced by `SystemSet`][7267]
* [ECS: `SystemParamState::apply` now takes a `&SystemMeta` parameter in addition to the provided `&mut World`.][6900]
* [ECS: `SystemTypeIdLabel<T>` was replaced by `SystemSetType<T>`][7267]
* [ECS: `tick_global_task_pools_on_main_thread` is no longer run as an exclusive system. Instead, it has been replaced by `tick_global_task_pools`, which uses a `NonSend` resource to force running on the main thread.][7267]
* [ECS: `Tick::is_older_than` was renamed to `Tick::is_newer_than`. This is not a functional change, since that was what was always being calculated, despite the wrong name.][7561]
* [ECS: `UnsafeWorldCell::world` is now used to get immutable access to the whole world instead of just the metadata which can now be done via `UnsafeWorldCell::world_metadata`][7381]
* [ECS: `World::init_non_send_resource` now returns the generated `ComponentId`.][7284]
* [ECS: `World::init_resource` now returns the generated `ComponentId`.][7284]
* [ECS: `World::iter_entities` now returns an iterator of `EntityRef` instead of `Entity`.][6843]
* [ECS: `World`s can now only hold a maximum of 2^32 - 1 tables.][6681]
* [ECS: `World`s can now only hold a maximum of 2^32- 1 archetypes.][6681]
* [ECS: `WorldId` now implements `SystemParam` and will return the id of the world the system is running in][7741]
* [ECS: Adding rendering extraction systems now panics rather than silently failing if no subapp with the `RenderApp` label is found.][7267]
* [ECS: Allow adding systems to multiple sets that share the same base set][7709]
* [ECS: change `is_system_type() -> bool` to `system_type() -> Option<TypeId>`][7715]
* [ECS: changed some `UnsafeWorldCell` methods to take `self` instead of `&self`/`&mut self` since there is literally no point to them doing that][7381]
* [ECS: Changed: `Query::for_each(_mut)`, `QueryParIter` will now leverage autovectorization to speed up query iteration where possible.][6547]
* [ECS: Default to using ExecutorKind::SingleThreaded on wasm32][7717]
* [ECS: Ensure `Query` does not use the wrong `World`][7150]
* [ECS: Exclusive systems may now be used with system piping.][7023]
* [ECS: expose `ScheduleGraph` for use in third party tools][7522]
* [ECS: extract topsort logic to a new method, one pass to detect cycles and …][7727]
* [ECS: Fixed time steps now use a schedule (`CoreSchedule::FixedTimeStep`) rather than a run criteria.][7267]
* [ECS: for disconnected, use Vec instead of HashSet to reduce insert overhead][7744]
* [ECS: Implement `SparseSetIndex` for `WorldId`][7125]
* [ECS: Improve the panic message for schedule build errors][7860]
* [ECS: Lift the 16-field limit from the `SystemParam` derive][6867]
* [ECS: Make `EntityRef::new` unsafe][7222]
* [ECS: Make `Query` fields private][7149]
* [ECS: make `ScheduleGraph::initialize` public][7723]
* [ECS: Make boxed conditions read-only][7786]
* [ECS: Make RemovedComponents mirror EventReaders API surface][7713]
* [ECS: Mark TableRow and TableId as repr(transparent)][7166]
* [ECS: Most APIs returning `&UnsafeCell<ComponentTicks>` now returns `TickCells` instead, which contains two separate `&UnsafeCell<Tick>` for either component ticks.][6547]
* [ECS: Move MainThreadExecutor for stageless migration.][7444]
* [ECS: Move safe operations out of `unsafe` blocks in `Query`][7851]
* [ECS: Optimize `.nth()` and `.last()` for event iterators][7530]
* [ECS: Optimize `Iterator::count` for event iterators][7582]
* [ECS: Provide public `EntityRef::get_change_ticks_by_id` that takes `ComponentId`][6683]
* [ECS: refactor: move internals from `entity_ref` to `World`, add `SAFETY` comments][6402]
* [ECS: Rename `EntityId` to `EntityIndex`][6732]
* [ECS: Rename `UnsafeWorldCellEntityRef` to `UnsafeEntityCell`][7568]
* [ECS: Rename schedule v3 to schedule][7519]
* [ECS: Rename state_equals condition to in_state][7677]
* [ECS: Replace `World::read_change_ticks` with `World::change_ticks` within `bevy_ecs` crate][6816]
* [ECS: Replaced the trait `ReadOnlySystemParamFetch` with `ReadOnlySystemParam`.][6865]
* [ECS: Simplified the `SystemParamFunction` and `ExclusiveSystemParamFunction` traits.][7675]
* [ECS: Speed up `CommandQueue` by storing commands more densely][6391]
* [ECS: Stageless: move final apply outside of spawned executor][7445]
* [ECS: Stageless: prettier cycle reporting][7463]
* [ECS: Systems without `Commands` and  `ParallelCommands` will no longer show a `system_commands` span when profiling.][6900]
* [ECS: The `ReportHierarchyIssue` resource now has a public constructor (`new`), and implements `PartialEq`][7267]
* [ECS: The `StartupSchedule` label is now defined as part of the `CoreSchedules` enum][7267]
* [ECS: The `SystemParam` derive is now more flexible, allowing you to omit unused lifetime parameters.][6694]
* [ECS: the top level `bevy_ecs::schedule` module was replaced with `bevy_ecs::scheduling`][7267]
* [ECS: Use `World` helper methods for sending `HierarchyEvent`s][6921]
* [ECS: Use a bounded channel in the multithreaded executor][7829]
* [ECS: Use a default implementation for `set_if_neq`][7660]
* [ECS: Use consistent names for marker generics][7788]
* [ECS: Use correct terminology for a `NonSend` run condition panic][7841]
* [ECS: Use default-implemented methods for `IntoSystemConfig<>`][7870]
* [ECS: use try_send to replace send.await, unbounded channel should always b…][7745]
* [General: The MSRV of the engine is now 1.67.][7379]
* [Input: Bump gilrs version to 0.10][6558]
* [IOS, Android... same thing][7493]
* [Math: Update `glam` to `0.23`][7883]
* [Math: use `Mul<f32>` to double the value of `Vec3`][6607]
* [Reflect: bevy_reflect now uses a fixed state for its hasher, which means the output of `Reflect::reflect_hash` is now deterministic across processes.][7583]
* [Reflect: Changed function signatures of `ReflectComponent` methods, `apply`, `remove`, `contains`, and `reflect`.][7206]
* [Reflect: Changed the `List::push` and `List::pop` to have default implementations.][7063]
* [Reflect: Registered `SmallVec<[Entity; 8]>` in the type registry][6578]
* [Renamed methods on `GetPath`:][7321]
  * `path` -> `reflect_path`
  * `path_mut` -> `reflect_path_mut`
  * `get_path` -> `path`
  * `get_path_mut` -> `path_mut`
* [Render: Allow prepass in webgl][7537]
* [Render: bevy_pbr: Avoid copying structs and using registers in shaders][7069]
* [Render: bevy_pbr: Clear fog DynamicUniformBuffer before populating each frame][7432]
* [Render: bevy_render: Run calculate_bounds in the end-of-update exclusive systems][7127]
* [Render: Change the glTF loader to use `Camera3dBundle`][7890]
* [Render: Changed &mut PipelineCache to &PipelineCache][7598]
* [Render: Intepret glTF colors as linear instead of sRGB][6828]
* [Render: Move 'startup' Resource `WgpuSettings`  into the `RenderPlugin`][6946]
* [Render: Move prepass functions to prepass_utils][7354]
* [Render: Only compute sprite color once per quad][7498]
* [Render: Only execute `#define` if current scope is accepting lines][7798]
* [Render: Pipelined Rendering][6503]
* [Render: Refactor Globals and View structs into separate shaders][7512]
* [Render: Replace UUID based IDs with a atomic-counted ones][6988]
* [Render: run clear trackers on render world][6878]
* [Render: set cull mode: None for Mesh2d][7514]
* [Render: Shader defs can now have a value][5900]
* [Render: Shrink ComputedVisibility][6305]
* [Render: Use prepass shaders for shadows][7784]
* [Rendering: `add_node_edge` is now infallible (panics on error)][6720]
* [Rendering: `add_slot_edge` is now infallible (panics on error)][6720]
* [Rendering: `AsBindGroup` is now object-safe.][6937]
* [Rendering: `BloomSettings::knee` renamed to `BloomPrefilterSettings::softness`.][6677]
* [Rendering: `BloomSettings::threshold` renamed to `BloomPrefilterSettings::threshold`.][6677]
* [Rendering: `HexColorError::Hex` has been renamed to `HexColorError::Char`][6940]
* [Rendering: `input_node` now panics on `None`][6720]
* [Rendering: `ktx2` and `zstd` are now part of bevy’s default enabled features][7696]
* [Rendering: `Msaa` is now enum][7292]
* [Rendering: `PipelineCache` no longer requires mutable access in order to queue render / compute pipelines.][7205]
* [Rendering: `RenderContext::command_encoder` is now private. Use the accessor `RenderContext::command_encoder()` instead.][7248]
* [Rendering: `RenderContext::render_device` is now private. Use the accessor `RenderContext::render_device()` instead.][7248]
* [Rendering: `RenderContext` now supports adding external `CommandBuffer`s for inclusion into the render graphs. These buffers can be encoded outside of the render graph (i.e. in a system).][7248]
* [Rendering: `scale` is now applied before updating `area`. Reading from it will take `scale` into account.][6201]
* [Rendering: `SkinnedMeshJoints::build` now takes a `&mut BufferVec` instead of a `&mut Vec` as a parameter.][6833]
* [Rendering: `StandardMaterial` now defaults to a dielectric material (0.0 `metallic`) with 0.5 `perceptual_roughness`.][7664]
* [Rendering: `TrackedRenderPass` now requires a `&RenderDevice` on construction.][7053]
* [Rendering: `Visibility` is now an enum][6320]
* [Rendering: Bloom now looks different.][6677]
* [Rendering: Directional lights now use cascaded shadow maps for improved shadow quality.][7064]
* [Rendering: ExtractedMaterials, extract_materials and prepare_materials are now public][7548]
* [Rendering: For performance reasons, some detailed renderer trace logs now require the use of cargo feature `detailed_trace` in addition to setting the log level to `TRACE` in order to be shown.][7639]
* [Rendering: Made cameras with the same target share the same `main_texture` tracker, which ensures continuity across cameras.][7671]
* [Rendering: Renamed `ScalingMode::Auto` to `ScalingMode::AutoMin`.][6496]
* [Rendering: Renamed `ScalingMode::None` to `ScalingMode::Fixed`][6201]
* [Rendering: Renamed `window_origin` to `viewport_origin`][6201]
* [Rendering: Renamed the `priority` field on `Camera` to `order`.][6908]
* [Rendering: Replaced `left`, `right`, `bottom`, and `top` fields with a single `area: Rect`][6201]
* [Rendering: StandardMaterials will now appear brighter and more saturated at high roughness, due to internal material changes. This is more physically correct.][7051]
* [Rendering: The `layout` field of `RenderPipelineDescriptor` and `ComputePipelineDescriptor` is now mandatory.][7681]
* [Rendering: The `rangefinder` module has been moved into the `render_phase` module.][7016]
* [Rendering: The bloom example has been renamed to bloom_3d and improved. A bloom_2d example was added.][6677]
* [Rendering: the SubApp Extract stage has been separated from running the sub app schedule.][7046]
* [Rendering: To enable multiple `RenderPhases` to share the same `TrackedRenderPass`, the `RenderPhase::render` signature has changed.][7043]
* [Rendering: update its `Transform` in order to preserve its `GlobalTransform` after the parent change][7024]
* [Rendering: Updated to wgpu 0.15, wgpu-hal 0.15.1, and naga 0.11][7356]
* [Rendering: Users can now use the DirectX Shader Compiler (DXC) on Windows with DX12 for faster shader compilation and ShaderModel 6.0+ support (requires `dxcompiler.dll` and `dxil.dll`)][7356]
* [Rendering: You can now set up the rendering code of a `RenderPhase` directly using the `RenderPhase::render` method, instead of implementing it manually in your render graph node.][7013]
* [Scenes: `SceneSpawner::spawn_dynamic` now returns `InstanceId` instead of `()`.][6663]
* [Shape: Change `From<Icosphere>` to `TryFrom<Icosphere>`][6484]
* [Tasks: `Scope` now uses `FallibleTask` to await the cancellation of all remaining tasks when it’s dropped.][6696]
* [Time: `Time::set_relative_speed_fXX` now allows a relative speed of -0.0.][7740]
* [UI: `FocusPolicy` default has changed from `FocusPolicy::Block` to `FocusPolicy::Pass`][7161]
* [UI: `TextPipeline::queue_text` and `GlyphBrush::compute_glyphs` now need a TextLineBreakBehaviour argument, in order to pass through the new field.][7283]
* [UI: `update_image_calculated_size_system` sets `preserve_aspect_ratio` to true for nodes with images.][6825]
* [UI: Added `Changed<Node>` to the change detection query of `text_system`. This ensures that any change in the size of a text node will cause any text it contains to be recomputed.][7674]
* [UI: Changed `Size::height` so it sets the `width` to `Val::AUTO`.][7626]
* [UI: Changed `Size::width` so it sets the `height` to `Val::AUTO`.][7626]
* [UI: Changed `TextAlignment` into an enum with `Left`, `Center`, and `Right` variants.][6807]
* [UI: Changed extract_uinodes to extract the flip_x and flip_y values from UiImage.][6292]
* [UI: Changed prepare_uinodes to swap the UV coordinates as required.][6292]
* [UI: Changed Taffy version to 0.3.3 and disabled its `grid` feature.][7859]
* [UI: Changed the `Size` `width` and `height` default values to `Val::Auto`][7475]
* [UI: Changed the `size` field of `CalculatedSize` to a Vec2.][7641]
* [UI: Changed UiImage derefs to texture field accesses.][6292]
* [UI: Changed UiImage to a struct with texture, flip_x, and flip_y fields.][6292]
* [UI: Modified the `text2d` example to show both linebreaking behaviours.][7283]
* [UI: Renamed `image_node_system` to `update_image_calculated_size_system`][6674]
* [UI: Renamed the `background_color` field of `ExtractedUiNode` to `color`.][7452]
* [UI: Simplified the UI examples. Replaced numeric values with the Flex property enums or elided them where possible, and removed the remaining use of auto margins.][7626]
* [UI: The `MeasureFunc` only preserves the aspect ratio when `preserve_aspect_ratio` is true.][6825]
* [UI: Updated `from_style` for Taffy 0.3.3.][7859]
* [UI: Upgraded to Taffy 0.2, improving UI layout performance significantly and adding the flexbox `gap` property and `AlignContent::SpaceEvenly`.][6743]
* [UI: Use `f32::INFINITY` instead of `f32::MAX` to represent unbounded text in Text2dBounds][6807]
* [Window: expose cursor position with scale][7297]
* [Window: Make WindowId::primary() const][6582]
* [Window: revert stage changed for window closing][7296]
* [Windowing: `WindowId` is now `Entity`.][5589]
* [Windowing: Moved `changed_window` and `despawn_window` systems to `CoreStage::Last` to avoid systems making changes to the `Window` between `changed_window` and the end of the frame as they would be ignored.][7517]
* [Windowing: Requesting maximization/minimization is done on the [`Window::state`] field.][5589]
* [Windowing: Width/height consolidated into a `WindowResolution` component.][5589]

### Removed

* [App: Removed `App::add_sub_app`][7290]
* [App: Rename dynamic feature][7340]
* [ECS: Remove .on_update method to improve API consistency and clarity][7667]
* [ECS: Remove `BuildWorldChildren` impl from `WorldChildBuilder`][6727]
* [ECS: Remove a duplicate lookup in `apply_state_transitions`][7800]
* [ECS: Remove an incorrect impl of `ReadOnlySystemParam` for `NonSendMut`][7243]
* [ECS: Remove APIs deprecated in 0.9][6801]
* [ECS: Remove broken `DoubleEndedIterator` impls on event iterators][7469]
* [ECS: Remove duplicate lookups from `Resource` initialization][7174]
* [ECS: Remove useless access to archetype in `UnsafeWorldCell::fetch_table`][7665]
* [ECS: Removed `AddBundle`. `Edges::get_add_bundle` now returns `Option<ArchetypeId>`][6742]
* [ECS: Removed `Archetype::new` and `Archetype::is_empty`.][6742]
* [ECS: Removed `ArchetypeComponentId::new` and `ArchetypeComponentId::value`.][6742]
* [ECS: Removed `ArchetypeGeneration::value`][6742]
* [ECS: Removed `ArchetypeId::new` and `ArchetypeId::value`.][6742]
* [ECS: Removed `ArchetypeIdentity`.][6742]
* [ECS: Removed `Archetypes`’s `Default` implementation.][6742]
* [ECS: Removed `AsSystemLabel` trait][7267]
* [ECS: Removed `Entities::alloc_at_without_replacement` and `AllocAtWithoutReplacement`.][6740]
* [ECS: Removed `Entities`’s `Default` implementation.][6740]
* [ECS: Removed `EntityMeta`][6740]
* [ECS: Removed `on_hierarchy_reports_enabled` run criteria (now just uses an ad hoc resource checking run condition)][7267]
* [ECS: Removed `RunCriteriaLabel`][7267]
* [ECS: Removed `RunCriteriaLabel`][7267]
* [ECS: Removed `SystemParamFetch`, its functionality has been moved to `SystemParamState`.][6865]
* [ECS: Removed `Table::component_capacity`][4928]
* [ECS: Removed `transform_propagate_system_set`: this was a nonstandard pattern that didn’t actually provide enough control. The systems are already `pub`: the docs have been updated to ensure that the third-party usage is clear.][7267]
* [ECS: removed `UnsafeWorldCell::storages` since that is probably unsound since storages contains the actual component/resource data not just metadata][7381]
* [ECS: Removed stages, and all code that mentions stages][7267]
* [ECS: Removed states have been dramatically simplified, and no longer use a stack][7267]
* [ECS: Removed systems in `RenderSet/Stage::Extract` no longer warn when they do not read data from the main world][7267]
* [ECS: Removed the bound `T: Sync` from `Local<T>` when used as an `ExclusiveSystemParam`.][7040]
* [ECS: Removed the method `ExclusiveSystemParamState::apply`.][7489]
* [ECS: Removed the trait `ExclusiveSystemParamState`, merging its functionality into `ExclusiveSystemParam`.][6919]
* [ECS: Removed the trait `SystemParamState`, merging its functionality into `SystemParam`.][6919]
* [ECS: Support `SystemParam` types with const generics][7001]
* [ECS: Use T::Storage::STORAGE_TYPE to optimize out unused branches][6800]
* [Hierarchy: Expose transform propagate systems][7145]
* [Hierarchy: Make adding children idempotent][6763]
* [Hierarchy: Remove `EntityCommands::add_children`][6942]
* [Input: Gamepad events refactor][6965]
* [Reflect: Make proc macros hygienic in bevy_reflect_derive][6752]
* [Reflect: Removed `#[module]` helper attribute for `Reflect` derives (this is not currently used)][7148]
* [Reflect: Removed `Array` as supertrait of `List`][7467]
* [Reflect: Removed `PixelInfo` and get `pixel_size` from wgpu][6820]
* [Reflect: Removed `ReflectSerialize` and `ReflectDeserialize` registrations from most glam types][6580]
* [Remove unnecessary `Default` impl of HandleType][7472]
* [Remove warning about missed events due to false positives][6730]
* [Render: Make Core Pipeline Graph Nodes Public][6605]
* [Render: Optimize color computation in prepare_uinodes][7311]
* [Render: Organized scene_viewer into plugins for reuse and organization][6936]
* [Render: put `update_frusta::<Projection>` in `UpdateProjectionFrusta` set][7526]
* [Render: Remove dependency on the mesh struct in the pbr function][7597]
* [Render: remove potential ub in render_resource_wrapper][7279]
* [Render: Remove redundant bitwise OR `TEXTURE_ADAPTER_SPECIFIC_FORMAT_FEATURES`][7033]
* [Render: Remove the early exit to make sure the prepass textures are cleared][7891]
* [Render: remove the image loaded check for nodes without images in extract_uinodes][7280]
* [Render: Remove unnecessary alternate create_texture path in prepare_asset for Image][6671]
* [Render: remove unused var in fxaa shader][7509]
* [Render: set AVAILABLE_STORAGE_BUFFER_BINDINGS to the actual number of buffers available][6787]
* [Render: Use `Time` `resource` instead of `Extract`ing `Time`][7316]
* [Render: use better set inheritance in render systems][7524]
* [Render: use blendstate blend for alphamode::blend][7899]
* [Render: Use Image::default for 1 pixel white texture directly][7884]
* [Rendering: Removed `bevy_render::render_phase::DrawState`. It was not usable in any form outside of `bevy_render`.][7053]
* [Rendering: Removed `BloomSettings::scale`.][6677]
* [Rendering: Removed `EntityPhaseItem` trait][6885]
* [Rendering: Removed `ExtractedJoints`.][6833]
* [Rendering: Removed `SetShadowViewBindGroup`, `queue_shadow_view_bind_group()`, and `LightMeta::shadow_view_bind_group` in favor of reusing the prepass view bind group.][7875]
* [Rendering: Removed the `render` feature group.][6912]
* [Scene: scene viewer: can select a scene from the asset path][6859]
* [Text: Warn instead of erroring when max_font_atlases is exceeded][6673]
* [Transform: Removed `GlobalTransform::translation_mut`][7134]
* [UI: Re-enable taffy send+sync assert][7769]
* [UI: Remove `TextError::ExceedMaxTextAtlases(usize)` variant][6796]
* [UI: Remove needless manual default impl of ButtonBundle][6970]
* [UI: Removed `HorizontalAlign` and `VerticalAlign`.][6807]
* [UI: Removed `ImageMode`.][6674]
* [UI: Removed `QueuedText`][7414]
* [UI: Removed the `image_mode` field from `ImageBundle`][6674]
* [UI: Removed the `Val` <-> `f32`  conversion for  `CalculatedSize`.][7641]
* [Update toml_edit to 0.18][7370]
* [Update tracing-chrome requirement from 0.6.0 to 0.7.0][6709]
* [Window: Remove unnecessary windows.rs file][7277]
* [Windowing: `window.always_on_top` has been removed, you can now use `window.window_level`][7480]
* [Windowing: Removed `ModifiesWindows` system label.][7517]

### Fixed

* [Asset: Fix asset_debug_server hang. There should be at most one ThreadExecut…][7825]
* [Asset: fix load_internal_binary_asset with debug_asset_server][7246]
* [Assets: Hot reloading for `LoadContext::read_asset_bytes`][6797]
* [Diagnostics: Console log messages now show when the `trace_tracy` feature was enabled.][6955]
* [ECS: Fix `last_changed()` and `set_last_changed()` for `MutUntyped`][7619]
* [ECS: Fix a miscompilation with `#[derive(SystemParam)]`][7105]
* [ECS: Fix get_unchecked_manual using archetype index instead of table row.][6625]
* [ECS: Fix ignored lifetimes in `#[derive(SystemParam)]`][7458]
* [ECS: Fix init_non_send_resource overwriting previous values][7261]
* [ECS: fix mutable aliases for a very short time if `WorldCell` is already borrowed][6639]
* [ECS: Fix partially consumed `QueryIter` and `QueryCombinationIter` having invalid `size_hint`][5214]
* [ECS: Fix PipeSystem panicking with exclusive systems][6698]
* [ECS: Fix soundness bug with `World: Send`. Dropping a `World` that contains a `!Send` resource on the wrong thread will now panic.][6534]
* [ECS: Fix Sparse Change Detection][6896]
* [ECS: Fix trait bounds for run conditions][7688]
* [ECS: Fix unsoundnes in `insert` `remove` and `despawn`][7805]
* [ECS: Fix unsoundness in `EntityMut::world_scope`][7387]
* [ECS: Fixed `DetectChanges::last_changed` returning the wrong value.][7560]
* [ECS: Fixed `DetectChangesMut::set_last_changed` not actually updating the `changed` tick.][7560]
* [ECS: Fixed `Res` and `Query` parameter never being mutually exclusive.][5105]
* [ECS: Fixed a bug that caused `#[derive(SystemParam)]` to leak the types of private fields.][7056]
* [ECS: schedule_v3: fix default set for systems not being applied][7350]
* [ECS: Stageless: close the finish channel so executor doesn't deadlock][7448]
* [ECS: Stageless: fix unapplied systems][7446]
* [Hierarchy: don't error when sending HierarchyEvents when Event type not registered][7031]
* [Hierarchy: Fix unsoundness for `propagate_recursive`][7003]
* [Hierarchy: Fixed missing `ChildAdded` events][6926]
* [Input: Avoid triggering change detection for inputs][6847]
* [Input: Fix `AxisSettings::new` only accepting invalid bounds][7233]
* [Input: Fix incorrect behavior of `just_pressed` and `just_released` in `Input<GamepadButton>`][7238]
* [Input: Removed Mobile Touch event y-axis flip][6597]
* [Reflect: bevy_reflect: Fix misplaced impls][6829]
* [Reflect: Fix bug where deserializing unit structs would fail for non-self-describing formats][6722]
* [Reflect: Fix bug where scene deserialization using certain readers could fail (e.g. `BufReader`, `File`, etc.)][6894]
* [Reflect: fix typo in bevy_reflect::impls::std GetTypeRegistration for vec like…][7520]
* [Reflect: Retain `::` after `>`, `)` or bracket when shortening type names][7755]
* [Render: bevy_core_pipeline: Fix prepass sort orders][7539]
* [Render: Cam scale cluster fix][7078]
* [Render: fix ambiguities in render schedule][7725]
* [Render: fix bloom viewport][6802]
* [Render: Fix dependency of shadow mapping on the optional `PrepassPlugin`][7878]
* [Render: Fix feature gating in texture_binding_array example][7425]
* [Render: Fix material alpha_mode in example global_vs_local_translation][6658]
* [Render: fix regex for shader define: must have at least one whitespace][7754]
* [Render: fix shader_instancing][7305]
* [Render: fix spot dir nan again][7176]
* [Render: Recreate tonemapping bind group if view uniforms buffer has changed][7904]
* [Render: Shadow render phase - pass the correct view entity][7048]
* [Render: Text2d doesn't recompute text on changes to the text's bounds][7846]
* [Render: wasm: pad globals uniform also in 2d][6643]
* [Rendering: Emission strength is now correctly interpreted by the `StandardMaterial` as linear instead of sRGB.][7897]
* [Rendering: Fix deband dithering intensity for non-HDR pipelines.][6707]
* [Rendering: Fixed StandardMaterial occlusion being incorrectly applied to direct lighting.][7051]
* [Rendering: Fixed the alpha channel of the `image::DynamicImage::ImageRgb32F` to `bevy_render::texture::Image` conversion in `bevy_render::texture::Image::from_dynamic()`.][6914]
* [Scene: Cleanup dynamic scene before building][6254]
* [Task: Fix panicking on another scope][6524]
* [UI: `Size::height` sets `width` not `height`][7478]
* [UI: Don't ignore UI scale for text][7510]
* [UI: Fix `bevy_ui` compile error without `bevy_text`][7877]
* [UI: Fix overflow scaling for images][7142]
* [UI: fix upsert_leaf not setting a MeasureFunc for new leaf nodes][7351]
* [Window: Apply `WindowDescriptor` settings in all modes][6934]
* [Window: break feedback loop when moving cursor][7298]
* [Window: create window as soon as possible][7668]
* [Window: Fix a typo on `Window::set_minimized`][7276]
* [Window: Fix closing window does not exit app in desktop_app mode][7628]
* [Window: fix cursor grab issue][7010]
* [Window: Fix set_cursor_grab_mode to try an alternative mode before giving an error][6599]

[3212]: https://github.com/bevyengine/bevy/pull/3212
[4241]: https://github.com/bevyengine/bevy/pull/4241
[4594]: https://github.com/bevyengine/bevy/pull/4594
[4777]: https://github.com/bevyengine/bevy/pull/4777
[4878]: https://github.com/bevyengine/bevy/pull/4878
[4928]: https://github.com/bevyengine/bevy/pull/4928
[5105]: https://github.com/bevyengine/bevy/pull/5105
[5214]: https://github.com/bevyengine/bevy/pull/5214
[5346]: https://github.com/bevyengine/bevy/pull/5346
[5400]: https://github.com/bevyengine/bevy/pull/5400
[5428]: https://github.com/bevyengine/bevy/pull/5428
[5454]: https://github.com/bevyengine/bevy/pull/5454
[5542]: https://github.com/bevyengine/bevy/pull/5542
[5589]: https://github.com/bevyengine/bevy/pull/5589
[5735]: https://github.com/bevyengine/bevy/pull/5735
[5900]: https://github.com/bevyengine/bevy/pull/5900
[6028]: https://github.com/bevyengine/bevy/pull/6028
[6035]: https://github.com/bevyengine/bevy/pull/6035
[6129]: https://github.com/bevyengine/bevy/pull/6129
[6201]: https://github.com/bevyengine/bevy/pull/6201
[6235]: https://github.com/bevyengine/bevy/pull/6235
[6245]: https://github.com/bevyengine/bevy/pull/6245
[6254]: https://github.com/bevyengine/bevy/pull/6254
[6284]: https://github.com/bevyengine/bevy/pull/6284
[6292]: https://github.com/bevyengine/bevy/pull/6292
[6305]: https://github.com/bevyengine/bevy/pull/6305
[6320]: https://github.com/bevyengine/bevy/pull/6320
[6388]: https://github.com/bevyengine/bevy/pull/6388
[6391]: https://github.com/bevyengine/bevy/pull/6391
[6402]: https://github.com/bevyengine/bevy/pull/6402
[6404]: https://github.com/bevyengine/bevy/pull/6404
[6412]: https://github.com/bevyengine/bevy/pull/6412
[6436]: https://github.com/bevyengine/bevy/pull/6436
[6484]: https://github.com/bevyengine/bevy/pull/6484
[6496]: https://github.com/bevyengine/bevy/pull/6496
[6503]: https://github.com/bevyengine/bevy/pull/6503
[6524]: https://github.com/bevyengine/bevy/pull/6524
[6527]: https://github.com/bevyengine/bevy/pull/6527
[6534]: https://github.com/bevyengine/bevy/pull/6534
[6539]: https://github.com/bevyengine/bevy/pull/6539
[6547]: https://github.com/bevyengine/bevy/pull/6547
[6557]: https://github.com/bevyengine/bevy/pull/6557
[6558]: https://github.com/bevyengine/bevy/pull/6558
[6560]: https://github.com/bevyengine/bevy/pull/6560
[6561]: https://github.com/bevyengine/bevy/pull/6561
[6564]: https://github.com/bevyengine/bevy/pull/6564
[6566]: https://github.com/bevyengine/bevy/pull/6566
[6571]: https://github.com/bevyengine/bevy/pull/6571
[6578]: https://github.com/bevyengine/bevy/pull/6578
[6580]: https://github.com/bevyengine/bevy/pull/6580
[6582]: https://github.com/bevyengine/bevy/pull/6582
[6587]: https://github.com/bevyengine/bevy/pull/6587
[6592]: https://github.com/bevyengine/bevy/pull/6592
[6597]: https://github.com/bevyengine/bevy/pull/6597
[6599]: https://github.com/bevyengine/bevy/pull/6599
[6602]: https://github.com/bevyengine/bevy/pull/6602
[6605]: https://github.com/bevyengine/bevy/pull/6605
[6607]: https://github.com/bevyengine/bevy/pull/6607
[6612]: https://github.com/bevyengine/bevy/pull/6612
[6618]: https://github.com/bevyengine/bevy/pull/6618
[6625]: https://github.com/bevyengine/bevy/pull/6625
[6633]: https://github.com/bevyengine/bevy/pull/6633
[6639]: https://github.com/bevyengine/bevy/pull/6639
[6643]: https://github.com/bevyengine/bevy/pull/6643
[6644]: https://github.com/bevyengine/bevy/pull/6644
[6653]: https://github.com/bevyengine/bevy/pull/6653
[6658]: https://github.com/bevyengine/bevy/pull/6658
[6663]: https://github.com/bevyengine/bevy/pull/6663
[6664]: https://github.com/bevyengine/bevy/pull/6664
[6671]: https://github.com/bevyengine/bevy/pull/6671
[6672]: https://github.com/bevyengine/bevy/pull/6672
[6673]: https://github.com/bevyengine/bevy/pull/6673
[6674]: https://github.com/bevyengine/bevy/pull/6674
[6677]: https://github.com/bevyengine/bevy/pull/6677
[6681]: https://github.com/bevyengine/bevy/pull/6681
[6683]: https://github.com/bevyengine/bevy/pull/6683
[6692]: https://github.com/bevyengine/bevy/pull/6692
[6694]: https://github.com/bevyengine/bevy/pull/6694
[6696]: https://github.com/bevyengine/bevy/pull/6696
[6698]: https://github.com/bevyengine/bevy/pull/6698
[6699]: https://github.com/bevyengine/bevy/pull/6699
[6707]: https://github.com/bevyengine/bevy/pull/6707
[6709]: https://github.com/bevyengine/bevy/pull/6709
[6720]: https://github.com/bevyengine/bevy/pull/6720
[6722]: https://github.com/bevyengine/bevy/pull/6722
[6727]: https://github.com/bevyengine/bevy/pull/6727
[6730]: https://github.com/bevyengine/bevy/pull/6730
[6732]: https://github.com/bevyengine/bevy/pull/6732
[6734]: https://github.com/bevyengine/bevy/pull/6734
[6740]: https://github.com/bevyengine/bevy/pull/6740
[6742]: https://github.com/bevyengine/bevy/pull/6742
[6743]: https://github.com/bevyengine/bevy/pull/6743
[6745]: https://github.com/bevyengine/bevy/pull/6745
[6751]: https://github.com/bevyengine/bevy/pull/6751
[6752]: https://github.com/bevyengine/bevy/pull/6752
[6755]: https://github.com/bevyengine/bevy/pull/6755
[6761]: https://github.com/bevyengine/bevy/pull/6761
[6763]: https://github.com/bevyengine/bevy/pull/6763
[6781]: https://github.com/bevyengine/bevy/pull/6781
[6785]: https://github.com/bevyengine/bevy/pull/6785
[6786]: https://github.com/bevyengine/bevy/pull/6786
[6787]: https://github.com/bevyengine/bevy/pull/6787
[6796]: https://github.com/bevyengine/bevy/pull/6796
[6797]: https://github.com/bevyengine/bevy/pull/6797
[6800]: https://github.com/bevyengine/bevy/pull/6800
[6801]: https://github.com/bevyengine/bevy/pull/6801
[6802]: https://github.com/bevyengine/bevy/pull/6802
[6807]: https://github.com/bevyengine/bevy/pull/6807
[6809]: https://github.com/bevyengine/bevy/pull/6809
[6811]: https://github.com/bevyengine/bevy/pull/6811
[6816]: https://github.com/bevyengine/bevy/pull/6816
[6817]: https://github.com/bevyengine/bevy/pull/6817
[6820]: https://github.com/bevyengine/bevy/pull/6820
[6825]: https://github.com/bevyengine/bevy/pull/6825
[6828]: https://github.com/bevyengine/bevy/pull/6828
[6829]: https://github.com/bevyengine/bevy/pull/6829
[6831]: https://github.com/bevyengine/bevy/pull/6831
[6833]: https://github.com/bevyengine/bevy/pull/6833
[6843]: https://github.com/bevyengine/bevy/pull/6843
[6847]: https://github.com/bevyengine/bevy/pull/6847
[6851]: https://github.com/bevyengine/bevy/pull/6851
[6853]: https://github.com/bevyengine/bevy/pull/6853
[6859]: https://github.com/bevyengine/bevy/pull/6859
[6865]: https://github.com/bevyengine/bevy/pull/6865
[6867]: https://github.com/bevyengine/bevy/pull/6867
[6874]: https://github.com/bevyengine/bevy/pull/6874
[6878]: https://github.com/bevyengine/bevy/pull/6878
[6881]: https://github.com/bevyengine/bevy/pull/6881
[6885]: https://github.com/bevyengine/bevy/pull/6885
[6894]: https://github.com/bevyengine/bevy/pull/6894
[6896]: https://github.com/bevyengine/bevy/pull/6896
[6899]: https://github.com/bevyengine/bevy/pull/6899
[6900]: https://github.com/bevyengine/bevy/pull/6900
[6902]: https://github.com/bevyengine/bevy/pull/6902
[6908]: https://github.com/bevyengine/bevy/pull/6908
[6912]: https://github.com/bevyengine/bevy/pull/6912
[6914]: https://github.com/bevyengine/bevy/pull/6914
[6919]: https://github.com/bevyengine/bevy/pull/6919
[6921]: https://github.com/bevyengine/bevy/pull/6921
[6922]: https://github.com/bevyengine/bevy/pull/6922
[6926]: https://github.com/bevyengine/bevy/pull/6926
[6934]: https://github.com/bevyengine/bevy/pull/6934
[6935]: https://github.com/bevyengine/bevy/pull/6935
[6936]: https://github.com/bevyengine/bevy/pull/6936
[6937]: https://github.com/bevyengine/bevy/pull/6937
[6940]: https://github.com/bevyengine/bevy/pull/6940
[6942]: https://github.com/bevyengine/bevy/pull/6942
[6946]: https://github.com/bevyengine/bevy/pull/6946
[6955]: https://github.com/bevyengine/bevy/pull/6955
[6957]: https://github.com/bevyengine/bevy/pull/6957
[6965]: https://github.com/bevyengine/bevy/pull/6965
[6970]: https://github.com/bevyengine/bevy/pull/6970
[6973]: https://github.com/bevyengine/bevy/pull/6973
[6980]: https://github.com/bevyengine/bevy/pull/6980
[6988]: https://github.com/bevyengine/bevy/pull/6988
[6995]: https://github.com/bevyengine/bevy/pull/6995
[7001]: https://github.com/bevyengine/bevy/pull/7001
[7003]: https://github.com/bevyengine/bevy/pull/7003
[7009]: https://github.com/bevyengine/bevy/pull/7009
[7010]: https://github.com/bevyengine/bevy/pull/7010
[7013]: https://github.com/bevyengine/bevy/pull/7013
[7014]: https://github.com/bevyengine/bevy/pull/7014
[7015]: https://github.com/bevyengine/bevy/pull/7015
[7016]: https://github.com/bevyengine/bevy/pull/7016
[7017]: https://github.com/bevyengine/bevy/pull/7017
[7020]: https://github.com/bevyengine/bevy/pull/7020
[7023]: https://github.com/bevyengine/bevy/pull/7023
[7024]: https://github.com/bevyengine/bevy/pull/7024
[7031]: https://github.com/bevyengine/bevy/pull/7031
[7033]: https://github.com/bevyengine/bevy/pull/7033
[7039]: https://github.com/bevyengine/bevy/pull/7039
[7040]: https://github.com/bevyengine/bevy/pull/7040
[7041]: https://github.com/bevyengine/bevy/pull/7041
[7043]: https://github.com/bevyengine/bevy/pull/7043
[7046]: https://github.com/bevyengine/bevy/pull/7046
[7048]: https://github.com/bevyengine/bevy/pull/7048
[7051]: https://github.com/bevyengine/bevy/pull/7051
[7053]: https://github.com/bevyengine/bevy/pull/7053
[7056]: https://github.com/bevyengine/bevy/pull/7056
[7060]: https://github.com/bevyengine/bevy/pull/7060
[7063]: https://github.com/bevyengine/bevy/pull/7063
[7064]: https://github.com/bevyengine/bevy/pull/7064
[7069]: https://github.com/bevyengine/bevy/pull/7069
[7076]: https://github.com/bevyengine/bevy/pull/7076
[7078]: https://github.com/bevyengine/bevy/pull/7078
[7083]: https://github.com/bevyengine/bevy/pull/7083
[7084]: https://github.com/bevyengine/bevy/pull/7084
[7087]: https://github.com/bevyengine/bevy/pull/7087
[7094]: https://github.com/bevyengine/bevy/pull/7094
[7097]: https://github.com/bevyengine/bevy/pull/7097
[7105]: https://github.com/bevyengine/bevy/pull/7105
[7113]: https://github.com/bevyengine/bevy/pull/7113
[7114]: https://github.com/bevyengine/bevy/pull/7114
[7117]: https://github.com/bevyengine/bevy/pull/7117
[7125]: https://github.com/bevyengine/bevy/pull/7125
[7127]: https://github.com/bevyengine/bevy/pull/7127
[7134]: https://github.com/bevyengine/bevy/pull/7134
[7142]: https://github.com/bevyengine/bevy/pull/7142
[7145]: https://github.com/bevyengine/bevy/pull/7145
[7146]: https://github.com/bevyengine/bevy/pull/7146
[7148]: https://github.com/bevyengine/bevy/pull/7148
[7149]: https://github.com/bevyengine/bevy/pull/7149
[7150]: https://github.com/bevyengine/bevy/pull/7150
[7151]: https://github.com/bevyengine/bevy/pull/7151
[7161]: https://github.com/bevyengine/bevy/pull/7161
[7164]: https://github.com/bevyengine/bevy/pull/7164
[7166]: https://github.com/bevyengine/bevy/pull/7166
[7174]: https://github.com/bevyengine/bevy/pull/7174
[7176]: https://github.com/bevyengine/bevy/pull/7176
[7181]: https://github.com/bevyengine/bevy/pull/7181
[7182]: https://github.com/bevyengine/bevy/pull/7182
[7186]: https://github.com/bevyengine/bevy/pull/7186
[7199]: https://github.com/bevyengine/bevy/pull/7199
[7205]: https://github.com/bevyengine/bevy/pull/7205
[7206]: https://github.com/bevyengine/bevy/pull/7206
[7222]: https://github.com/bevyengine/bevy/pull/7222
[7233]: https://github.com/bevyengine/bevy/pull/7233
[7238]: https://github.com/bevyengine/bevy/pull/7238
[7243]: https://github.com/bevyengine/bevy/pull/7243
[7245]: https://github.com/bevyengine/bevy/pull/7245
[7246]: https://github.com/bevyengine/bevy/pull/7246
[7248]: https://github.com/bevyengine/bevy/pull/7248
[7261]: https://github.com/bevyengine/bevy/pull/7261
[7262]: https://github.com/bevyengine/bevy/pull/7262
[7267]: https://github.com/bevyengine/bevy/pull/7267
[7276]: https://github.com/bevyengine/bevy/pull/7276
[7277]: https://github.com/bevyengine/bevy/pull/7277
[7279]: https://github.com/bevyengine/bevy/pull/7279
[7280]: https://github.com/bevyengine/bevy/pull/7280
[7283]: https://github.com/bevyengine/bevy/pull/7283
[7284]: https://github.com/bevyengine/bevy/pull/7284
[7290]: https://github.com/bevyengine/bevy/pull/7290
[7292]: https://github.com/bevyengine/bevy/pull/7292
[7296]: https://github.com/bevyengine/bevy/pull/7296
[7297]: https://github.com/bevyengine/bevy/pull/7297
[7298]: https://github.com/bevyengine/bevy/pull/7298
[7304]: https://github.com/bevyengine/bevy/pull/7304
[7305]: https://github.com/bevyengine/bevy/pull/7305
[7306]: https://github.com/bevyengine/bevy/pull/7306
[7311]: https://github.com/bevyengine/bevy/pull/7311
[7316]: https://github.com/bevyengine/bevy/pull/7316
[7321]: https://github.com/bevyengine/bevy/pull/7321
[7324]: https://github.com/bevyengine/bevy/pull/7324
[7325]: https://github.com/bevyengine/bevy/pull/7325
[7340]: https://github.com/bevyengine/bevy/pull/7340
[7343]: https://github.com/bevyengine/bevy/pull/7343
[7350]: https://github.com/bevyengine/bevy/pull/7350
[7351]: https://github.com/bevyengine/bevy/pull/7351
[7354]: https://github.com/bevyengine/bevy/pull/7354
[7356]: https://github.com/bevyengine/bevy/pull/7356
[7364]: https://github.com/bevyengine/bevy/pull/7364
[7370]: https://github.com/bevyengine/bevy/pull/7370
[7379]: https://github.com/bevyengine/bevy/pull/7379
[7381]: https://github.com/bevyengine/bevy/pull/7381
[7385]: https://github.com/bevyengine/bevy/pull/7385
[7387]: https://github.com/bevyengine/bevy/pull/7387
[7392]: https://github.com/bevyengine/bevy/pull/7392
[7396]: https://github.com/bevyengine/bevy/pull/7396
[7399]: https://github.com/bevyengine/bevy/pull/7399
[7401]: https://github.com/bevyengine/bevy/pull/7401
[7414]: https://github.com/bevyengine/bevy/pull/7414
[7415]: https://github.com/bevyengine/bevy/pull/7415
[7423]: https://github.com/bevyengine/bevy/pull/7423
[7425]: https://github.com/bevyengine/bevy/pull/7425
[7431]: https://github.com/bevyengine/bevy/pull/7431
[7432]: https://github.com/bevyengine/bevy/pull/7432
[7444]: https://github.com/bevyengine/bevy/pull/7444
[7445]: https://github.com/bevyengine/bevy/pull/7445
[7446]: https://github.com/bevyengine/bevy/pull/7446
[7448]: https://github.com/bevyengine/bevy/pull/7448
[7449]: https://github.com/bevyengine/bevy/pull/7449
[7452]: https://github.com/bevyengine/bevy/pull/7452
[7456]: https://github.com/bevyengine/bevy/pull/7456
[7458]: https://github.com/bevyengine/bevy/pull/7458
[7463]: https://github.com/bevyengine/bevy/pull/7463
[7466]: https://github.com/bevyengine/bevy/pull/7466
[7467]: https://github.com/bevyengine/bevy/pull/7467
[7468]: https://github.com/bevyengine/bevy/pull/7468
[7469]: https://github.com/bevyengine/bevy/pull/7469
[7471]: https://github.com/bevyengine/bevy/pull/7471
[7472]: https://github.com/bevyengine/bevy/pull/7472
[7475]: https://github.com/bevyengine/bevy/pull/7475
[7477]: https://github.com/bevyengine/bevy/pull/7477
[7478]: https://github.com/bevyengine/bevy/pull/7478
[7480]: https://github.com/bevyengine/bevy/pull/7480
[7481]: https://github.com/bevyengine/bevy/pull/7481
[7483]: https://github.com/bevyengine/bevy/pull/7483
[7489]: https://github.com/bevyengine/bevy/pull/7489
[7491]: https://github.com/bevyengine/bevy/pull/7491
[7493]: https://github.com/bevyengine/bevy/pull/7493
[7498]: https://github.com/bevyengine/bevy/pull/7498
[7503]: https://github.com/bevyengine/bevy/pull/7503
[7509]: https://github.com/bevyengine/bevy/pull/7509
[7510]: https://github.com/bevyengine/bevy/pull/7510
[7512]: https://github.com/bevyengine/bevy/pull/7512
[7514]: https://github.com/bevyengine/bevy/pull/7514
[7517]: https://github.com/bevyengine/bevy/pull/7517
[7518]: https://github.com/bevyengine/bevy/pull/7518
[7519]: https://github.com/bevyengine/bevy/pull/7519
[7520]: https://github.com/bevyengine/bevy/pull/7520
[7522]: https://github.com/bevyengine/bevy/pull/7522
[7524]: https://github.com/bevyengine/bevy/pull/7524
[7526]: https://github.com/bevyengine/bevy/pull/7526
[7527]: https://github.com/bevyengine/bevy/pull/7527
[7530]: https://github.com/bevyengine/bevy/pull/7530
[7535]: https://github.com/bevyengine/bevy/pull/7535
[7537]: https://github.com/bevyengine/bevy/pull/7537
[7539]: https://github.com/bevyengine/bevy/pull/7539
[7546]: https://github.com/bevyengine/bevy/pull/7546
[7547]: https://github.com/bevyengine/bevy/pull/7547
[7548]: https://github.com/bevyengine/bevy/pull/7548
[7559]: https://github.com/bevyengine/bevy/pull/7559
[7560]: https://github.com/bevyengine/bevy/pull/7560
[7561]: https://github.com/bevyengine/bevy/pull/7561
[7568]: https://github.com/bevyengine/bevy/pull/7568
[7574]: https://github.com/bevyengine/bevy/pull/7574
[7579]: https://github.com/bevyengine/bevy/pull/7579
[7582]: https://github.com/bevyengine/bevy/pull/7582
[7583]: https://github.com/bevyengine/bevy/pull/7583
[7586]: https://github.com/bevyengine/bevy/pull/7586
[7594]: https://github.com/bevyengine/bevy/pull/7594
[7596]: https://github.com/bevyengine/bevy/pull/7596
[7597]: https://github.com/bevyengine/bevy/pull/7597
[7598]: https://github.com/bevyengine/bevy/pull/7598
[7605]: https://github.com/bevyengine/bevy/pull/7605
[7617]: https://github.com/bevyengine/bevy/pull/7617
[7619]: https://github.com/bevyengine/bevy/pull/7619
[7623]: https://github.com/bevyengine/bevy/pull/7623
[7626]: https://github.com/bevyengine/bevy/pull/7626
[7628]: https://github.com/bevyengine/bevy/pull/7628
[7638]: https://github.com/bevyengine/bevy/pull/7638
[7639]: https://github.com/bevyengine/bevy/pull/7639
[7641]: https://github.com/bevyengine/bevy/pull/7641
[7653]: https://github.com/bevyengine/bevy/pull/7653
[7660]: https://github.com/bevyengine/bevy/pull/7660
[7664]: https://github.com/bevyengine/bevy/pull/7664
[7665]: https://github.com/bevyengine/bevy/pull/7665
[7667]: https://github.com/bevyengine/bevy/pull/7667
[7668]: https://github.com/bevyengine/bevy/pull/7668
[7671]: https://github.com/bevyengine/bevy/pull/7671
[7674]: https://github.com/bevyengine/bevy/pull/7674
[7675]: https://github.com/bevyengine/bevy/pull/7675
[7677]: https://github.com/bevyengine/bevy/pull/7677
[7681]: https://github.com/bevyengine/bevy/pull/7681
[7683]: https://github.com/bevyengine/bevy/pull/7683
[7684]: https://github.com/bevyengine/bevy/pull/7684
[7688]: https://github.com/bevyengine/bevy/pull/7688
[7696]: https://github.com/bevyengine/bevy/pull/7696
[7701]: https://github.com/bevyengine/bevy/pull/7701
[7709]: https://github.com/bevyengine/bevy/pull/7709
[7713]: https://github.com/bevyengine/bevy/pull/7713
[7715]: https://github.com/bevyengine/bevy/pull/7715
[7717]: https://github.com/bevyengine/bevy/pull/7717
[7718]: https://github.com/bevyengine/bevy/pull/7718
[7720]: https://github.com/bevyengine/bevy/pull/7720
[7723]: https://github.com/bevyengine/bevy/pull/7723
[7724]: https://github.com/bevyengine/bevy/pull/7724
[7725]: https://github.com/bevyengine/bevy/pull/7725
[7726]: https://github.com/bevyengine/bevy/pull/7726
[7727]: https://github.com/bevyengine/bevy/pull/7727
[7737]: https://github.com/bevyengine/bevy/pull/7737
[7740]: https://github.com/bevyengine/bevy/pull/7740
[7741]: https://github.com/bevyengine/bevy/pull/7741
[7744]: https://github.com/bevyengine/bevy/pull/7744
[7745]: https://github.com/bevyengine/bevy/pull/7745
[7753]: https://github.com/bevyengine/bevy/pull/7753
[7754]: https://github.com/bevyengine/bevy/pull/7754
[7755]: https://github.com/bevyengine/bevy/pull/7755
[7756]: https://github.com/bevyengine/bevy/pull/7756
[7766]: https://github.com/bevyengine/bevy/pull/7766
[7769]: https://github.com/bevyengine/bevy/pull/7769
[7782]: https://github.com/bevyengine/bevy/pull/7782
[7784]: https://github.com/bevyengine/bevy/pull/7784
[7786]: https://github.com/bevyengine/bevy/pull/7786
[7788]: https://github.com/bevyengine/bevy/pull/7788
[7790]: https://github.com/bevyengine/bevy/pull/7790
[7793]: https://github.com/bevyengine/bevy/pull/7793
[7796]: https://github.com/bevyengine/bevy/pull/7796
[7798]: https://github.com/bevyengine/bevy/pull/7798
[7800]: https://github.com/bevyengine/bevy/pull/7800
[7805]: https://github.com/bevyengine/bevy/pull/7805
[7806]: https://github.com/bevyengine/bevy/pull/7806
[7810]: https://github.com/bevyengine/bevy/pull/7810
[7815]: https://github.com/bevyengine/bevy/pull/7815
[7825]: https://github.com/bevyengine/bevy/pull/7825
[7829]: https://github.com/bevyengine/bevy/pull/7829
[7841]: https://github.com/bevyengine/bevy/pull/7841
[7846]: https://github.com/bevyengine/bevy/pull/7846
[7847]: https://github.com/bevyengine/bevy/pull/7847
[7851]: https://github.com/bevyengine/bevy/pull/7851
[7859]: https://github.com/bevyengine/bevy/pull/7859
[7860]: https://github.com/bevyengine/bevy/pull/7860
[7863]: https://github.com/bevyengine/bevy/pull/7863
[7866]: https://github.com/bevyengine/bevy/pull/7866
[7870]: https://github.com/bevyengine/bevy/pull/7870
[7875]: https://github.com/bevyengine/bevy/pull/7875
[7877]: https://github.com/bevyengine/bevy/pull/7877
[7878]: https://github.com/bevyengine/bevy/pull/7878
[7883]: https://github.com/bevyengine/bevy/pull/7883
[7884]: https://github.com/bevyengine/bevy/pull/7884
[7890]: https://github.com/bevyengine/bevy/pull/7890
[7891]: https://github.com/bevyengine/bevy/pull/7891
[7895]: https://github.com/bevyengine/bevy/pull/7895
[7897]: https://github.com/bevyengine/bevy/pull/7897
[7899]: https://github.com/bevyengine/bevy/pull/7899
[7904]: https://github.com/bevyengine/bevy/pull/7904
