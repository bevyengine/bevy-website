+++
title = "Testing Bevy Apps"
insert_anchor_links = "right"
[extra]
weight = 4
+++

Games and graphical applications are notoriously challenging to test, but that doesn't mean you shouldn't try.
Even in a rapidly-changing project, spending a couple of minutes to write tests can save you hours of debugging later.

Bevy does its best to help you with this, and we've collected some best practices from production projects here.
This chapter is intended as a crash course for users who are new to testing in Rust, or for more seasoned devs looking for Bevy-specific advice on effective patterns.
For a fuller treatment of testing in Rust, see the [Rust Book], [Rust by Example], or the [rustc docs on testing].

[Rust Book]: https://doc.rust-lang.org/book/ch11-00-testing.html
[Rust by Example]: https://doc.rust-lang.org/rust-by-example/testing.html
[rustc docs on testing]: https://doc.rust-lang.org/rustc/tests/index.html

## Unit Tests

Unit tests check that specific small bits of functionality work as expected.
They are particularly useful when you have complex logic, tricky math, or critical invariants to uphold.

When making a game, you might want to write unit tests to ensure that the current life is always between `0` and `MAX` life, that your data can be losslessly serialized and deserialized, or that looking up tiles never panics even if it's out-of-bounds.
You should try to write a new unit test whenever you fix a tricky bug or whenever you're not confident about the correctness of some behavior.

In Rust, unit tests are placed in the file containing the code needing to be tested, usually in a separate module called `tests`.
We gate this module behind a special flag, `test`, to ensure that the code is only compiled while testing.

Inside this module, we write our tests.
Each test should be a descriptively named function that gets its own `#[test]` annotation.
Tests can panic, usually via `.unwrap`, `.expect`, `assert!` or `assert_eq!` calls.
A panic means that the test failed:

```rust
#[cfg(test)]
mod test {
	#[test]
	fn one_plus_one_is_two(){
		assert_eq!(1 + 1, 2)
	}
}
```

## Documentation Tests

Rust allows you to embed code inside of your in-line API documentation directly.
You can write a doc test like so:

```rust
/// The life of a creature.
/// 
/// # Usage
/// 
/// ```rust
/// use my_crate::Life;
/// 
/// let mut life = Life::new(100);
/// life += Life(3);
/// 
/// assert_eq!(life.current, life.max);
/// ```
struct Life {
	current: u32,
	max: u32,
}
```

In doc tests specifically, the assertion is optional.
Simply compiling the code is often good enough.

Doc tests serve three purposes:

1. Teaching the reader about the intended usage of the API.
2. Serving as a canary for when code changes cause your documentation to go out of date.
3. Acting as quick unit tests to assert invariants.

Unsurprisingly, these are extremely useful for teaching users about libraries!
They're less compelling within a game or application though.

You will probably want to gradually clean up your code as your project settles, especially if you are working in a team.
Docs and tests are an important part of this.

However, doc tests demonstrating usage patterns are less useful when "go to usages" points the reader to dozens of real-world applications of the idea.

Ultimately, unit tests do a better job when we're evaluating the tools _solely_ as tests.
They don't interfere with the flow of reading, and IDE support is often better.
Save doc tests for teaching about usage and important invariants, and use unit tests for when your goal is primarily testing.

## Integration Tests

Integration tests are used to ensure that all of the parts of your project fit together and produce a desired end result.

In Rust, integration tests are conventionally written by creating a `.rs` file with `#[test]` functions inside of your project's `tests` folder.
Unlike unit tests, these tests can only access your crate's public API.

For games specifically, integration tests present some serious problems:

1. Most games require graphics to run, and will simply panic uninformatively if this fails. Most CI providers do not offer GPUs or monitors.
2. Most games require user input to advance.
3. Failure conditions can be subtle, often involving human judgement or visual processing, and rigidly encoding desired states can lead to fragile tests that need to be constantly rewritten.

On small teams, these hurdles often aren't worth overcoming — unit tests and manual testing may be all you need.

## Speeding Up Manual Testing

Regardless of the complexity of your automated testing, you should be regularly testing your project by hand to catch unexpected bugs and judge it as a whole.

You will be doing this a *lot*, so you should take the time to make this process both fast and painless.
Take advantage of existing developer tools, make your own, and make it easy to jump to problematic game states.

You should create a dedicated `devtools` feature flag for your application (and enable it by default) which will turn on all of the testing and debugging utilities that your project might need.
Reproducing tricky bugs can be incredibly time-consuming; you don't want to have to recompile your game to turn on these tools.
Then when you are [building for release](../releasing-projects/release-builds.md), turn your `devtools` feature flag off.

Rust's examples are a surprisingly powerful testing tool, letting you create secondary binaries with customized setups.
You might configure one to open the game with the settings menu open, another to take a command line argument to jump you to the supplied level, and a third to start a new run of the game using a prebuilt character.

Adding systems to these examples can be fairly straightforward.
A useful pattern is to directly mutate the desired [state](../control-flow/states.md) to bring you to a specific point in your game.
The [`run_once`] run condition can also be particularly useful to avoid surprising behavior once setup is complete.
You can even add asserts to verify that your setup steps have done what you wanted.

Over time you might want to integrate this sort of workflow directly into your save game infrastructure, like quickly loading and saving snapshots of interesting game states for example.
This can be very effective, but you should think hard about migration strategies.
Save games don't have nice compiler errors, and automatically upgrading them is a hard problem.

[`run_once`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/common_conditions/fn.run_once.html

## Testing With The ECS

When testing Bevy applications in a more integrated way, most of what we care about lives in the ECS.
To test that our game logic is working, we need to set up some world state and then assert something about it.

The tools below are presented in order of increasing realism and complexity.
You can mix-and-match, but you should prefer simpler methods wherever possible.

### The Best ECS Is No ECS

Just because your data lives inside of the ECS doesn't mean that you need to use the ECS in your tests.
This is an obvious insight, but surprisingly easy to overlook.

If you have a method on a component or resource, you can construct that type directly and test the method without ceremony:

```rust
#[derive(Component)]
struct Life {
    current: u32,
    max: u32,
}

impl Life {
    fn new(max: u32) -> Self {
        Life { current: max, max }
    }

    fn heal(&mut self, amount: u32) {
        self.current = (self.current + amount).min(self.max);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn healing_is_clamped_to_max() {
        let mut life = Life { current: 90, max: 100 };
        life.heal(50);
        assert_eq!(life.current, 100);
    }
}
```

These are the fastest, simplest tests you can write, and should be your default whenever the logic you care about is isolated to a single component or resource.
If the function you are testing doesn't need a `World`, neither does your test.

[`World`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html
[`App`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html

### Testing Using A Raw `World`

When you need to test helper functions that set up entities or resources, you can create a [`World`] directly and make assertions against it.

Note that the `spawn_enemy` helper below takes `&mut World` directly.
This is a common pattern for test utilities and setup code, but not something you'd typically do in actual gameplay systems (which should use `Commands` instead):

```rust
fn spawn_enemy(world: &mut World, health: u32) -> Entity {
    world.spawn((Enemy, Life::new(health))).id()
}

#[test]
fn enemies_spawn_at_full_health() {
    let mut world = World::new();
    let goblin = spawn_enemy(&mut world, 20);
    let dragon = spawn_enemy(&mut world, 500);

    assert_eq!(world.get::<Life>(goblin).unwrap().current, 20);
    assert_eq!(world.get::<Life>(dragon).unwrap().current, 500);
}
```

When using this pattern, the following `World` methods are extremely helpful:

- [`World::get`] and [`get_mut`] to quickly read and write to components on a single stored entity.
- [`World::resource`] and [`resource_mut`] to do the same for resources.
- [`World::query`] to iterate through multiple entities.
- [`World::write_message`] to send messages.
- [`World::trigger`] to send events.

This approach is a good fit for testing setup functions and simple world-reliant logic.
However, working directly with the `World` can be heavy on both creating boilerplate code and encountering borrow-checker headaches.
If you find yourself frustrated, or want to test your game's systems directly, you'll want one of the approaches below.

[`World::get`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.get
[`get_mut`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.get_mut
[`World::resource`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.resource
[`resource_mut`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.resource_mut
[`World::query`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.query
[`World::write_message`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.write_message
[`World::trigger`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.trigger

### Manually Running Systems

[`World::run_system_once`] runs a single system against the `World` and returns.
All you have to do is set up some data, run the system, check the results:

```rust
#[derive(Component)]
struct Poisoned;

#[derive(Resource)]
struct PoisonStrength(u32);

fn apply_poison(
    mut query: Query<&mut Life, With<Poisoned>>,
    strength: Res<PoisonStrength>,
) {
    for mut life in &mut query {
        life.current = life.current.saturating_sub(strength.0);
    }
}

#[test]
fn poison_only_hurts_poisoned_creatures() {
    let mut world = World::new();
    world.insert_resource(PoisonStrength(5));
    let poisoned = world.spawn((Life::new(100), Poisoned)).id();
    let healthy = world.spawn(Life::new(100)).id();

    world.run_system_once(apply_poison).unwrap();

    assert_eq!(world.get::<Life>(poisoned).unwrap().current, 95);
    assert_eq!(world.get::<Life>(healthy).unwrap().current, 100);
}
```

If your system returns a value, `run_system_once` passes it back through the `Result`.
This can be helpful to create complex test helpers that evaluate the state of the world, returning a single clean value that you can write assertions about across multiple tests.

If you use [`run_system_once_with`] instead, you can also pass in input values to configure exactly how the system should behave.
As always though, you should be deeply skeptical of overly clever tests.
They won't be helpful if they're more likely to be wrong than the code you're testing.

Deferred operations queued through [`Commands`] are applied automatically when the system finishes.

Be aware that the system is created fresh on every call.
[`Local`] variables will reset every time and change detection filters like `Added` or `Changed` won't work as they would in a real schedule.
If you need those, use a [`Schedule`] instead (see the next section).

This should be your default approach to testing real systems from your app and setting up complex scenarios.
You get full data access and Bevy's ECS ergonomics, but you still control exactly what runs and when.

[`World::run_system_once`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.run_system_once
[`run_system_once_with`]: https://docs.rs/bevy/latest/bevy/ecs/world/struct.World.html#method.run_system_once_with
[`Commands`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html
[`Local`]: https://docs.rs/bevy/latest/bevy/ecs/system/struct.Local.html
[`Schedule`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html
[`schedule.run`]: https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.Schedule.html#method.run

### Running Entire Schedules

When the behavior you care about comes from the interaction *between* systems, you can build up a [`Schedule`] and run it against a `World`.
This lets you verify [system ordering](../control-flow/system-ordering.md) and [run conditions](../control-flow/run-conditions.md), which you can't do with `run_system_once`.

```rust
fn regenerate(mut query: Query<&mut Life>) {
    for mut life in &mut query {
        life.heal(1);
    }
}

#[test]
fn ordering_affects_outcome() {
    let mut world = World::new();
    world.insert_resource(PoisonStrength(5));
    world.spawn((Life::new(100), Poisoned));

    let mut schedule = Schedule::default();
    schedule.add_systems((apply_poison, regenerate).chain());
    schedule.run(&mut world);

    let life = world.query::<&Life>().single(&world);
    // poison runs first: 100 - 5 + 1 = 96
    // without .chain(), the order would be ambiguous
    assert_eq!(life.current, 96);
}
```

Each call to [`schedule.run`] executes the systems once.
You can call it in a loop if you need to simulate multiple ticks.

Your test now depends on more moving parts, so failures can be harder to pin down.
Individually test systems first where you can, and save schedule-level tests for when you're specifically checking for system ordering or interaction.

### Stepping the `App` Loop

Taking this one step further, you can even construct an [`App`], add your plugins, and step it forward with [`app.update()`].
Each call to `update` runs a full frame: `Startup` (on the first call), then `PreUpdate`, `Update`, `PostUpdate`, and so on.

The reason to reach for this over a raw `Schedule` is [plugins](../../modular-architecture/plugins.md).
Suppose your game has a `CombatPlugin` that registers the `PoisonStrength` resource and the `apply_poison` system.
You can test the plugin as a whole, without manually recreating its internals:

```rust
#[test]
fn combat_plugin_applies_poison_each_frame() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, CombatPlugin));

    let entity = app.world_mut().spawn((Life::new(100), Poisoned)).id();

    app.update();

    // CombatPlugin registered PoisonStrength(5) and the apply_poison system
    let life = app.world().get::<Life>(entity).unwrap();
    assert_eq!(life.current, 95);
}
```

This is the closest you can get to "running the game" in a unit test.
Your systems will run inside the real plugin and schedule infrastructure without needing to run the entire rest of your game.

That fidelity comes at a cost though; these tests are significantly more brittle. 
Failures can be caused by completely unrelated code, and refactors commonly move critical logic in and out of the plugins you are testing.
Treat this approach like lightweight integration testing; it can ensure that everything fits together as expected, but it will also be a hassle to maintain.
Save them for critical game logic, and think carefully about your assertions to avoid breakage due to irrelevant changes.

#### Running To Completion

Sometimes you don't care *when* something happens, only *that* it happens.
A level loads, an animation finishes, a character dies, etc.
You can call [`app.update()`] in a loop and check after each tick:

```rust
#[test]
fn poisoned_creature_eventually_dies() {
    let mut app = App::new();
    app.add_plugins((MinimalPlugins, CombatPlugin));
    let entity = app.world_mut().spawn((Life::new(30), Poisoned)).id();

    // 30 hp at 5 damage/tick = 6 ticks to die; cap at 20 for margin
    for _ in 0..20 {
        app.update();

        if app.world().get::<Life>(entity).unwrap().current == 0 {
            return; // Test passed!
        }
    }

    panic!("creature did not die to poison within 20 ticks");
}
```

You'll need an iteration cap, as without one a bug can turn your test into an infinite loop.
Pick a number with some margin over the expected case, but not so large that a stuck test wastes minutes of your time.

This is the heaviest pattern discussed here, and is really only worth the trouble for simulation-style tests where the exact frame count doesn't matter.

[`app.update()`]: https://docs.rs/bevy/latest/bevy/app/struct.App.html#method.update

## Mocking User Inputs

On a bare technical level, mocking user inputs in Bevy is quite straightforward.
If you send the appropriate messages (e.g. `MouseButtonInput`) from `bevy_winit`, `bevy_input` or the like during `First`, they will automatically get picked up by all of Bevy's systems and processed into in-game actions.

The challenges come from:

1. Determining *when* to send the events.
2. Deciding on the abstraction layer you should be working at.
3. Ensuring that these tests are not too fragile.

These problems are tightly coupled: operating at the level of pixels, coordinates, and milliseconds is easy to implement and produces realistic results, but any test built that way will break whenever the user interface changes.

You should prefer:

1. Sending the highest level user input possible. Prefer actions over inputs, manipulate state rather than going through intermediates.
2. Locating objects to interact with by querying for them, rather than using hot keys or simulated positional mouse clicks.
3. Determining application state by polling the underlying data directly.
4. Deciding on whether or not to proceed by polling to see if the state has changed (with a timeout).

Only send low-level inputs when testing the low-to-high translation layer is the explicit goal, such as if you're writing your own input manager.

## Architecting for Testability

When thinking about _"how can we test this Bevy program"_, the difficulties of testing graphical output should be forefront in your mind.
This is because testing graphics:

1. Simply can't be done in most cheap or free CI setups, 
2. Is very hard to do in a robust fashion, and 
3. Is substantially slower and more expensive even when you do have a working setup.

As a result, your primary concern is keeping graphics decoupled from all of rest of your application so you can test *those* parts more easily.

When working with unit tests this is fairly easy.
Keep the scope of your tests low so they never have to instantiate any graphical code.
As long as your plugin architecture is reasonable (add `DefaultPlugins` and the like directly to your main app), this should be straightforward.

The challenge comes when trying to write headless integration tests.
If this is something you care about for your project, you should create a headless integration test immediately and religiously ensure that it continues working.
Decoupling an accidental rendering dependency a month after it was introduced is much harder than keeping it out of your `main` branch in the first place.

Bevy's own plugins are designed to work quite robustly without rendering. 
For example, `bevy_ui` and `bevy_ui_render` are different crates with their own plugins.
The best way to ensure that your code works without rendering (sometimes called "headlessly") is to create a `headless` feature flag for your application, and then disable plugins which are causing crashes in a feature-flagged cfg block:

```rust, hide-lines=1
# use bevy::prelude::*;
let mut app = App::new();

#[cfg(not(feature = "headless"))]
app.add_plugins(DefaultPlugins);

#[cfg(feature = "headless")]
app.add_plugins(
	DefaultPlugins.build()
	// Not a complete list!
	.disable::<AudioPlugin>()
	.disable::<UiRenderPlugin>()
);

app.run()
```

You could instead use conditional features to reduce the amount of compiled code for headless setups (see [Compiling Less Code]), but doing so is often more frustrating and harder to migrate than simply squashing panics as they arise.

Inside of your own code, be careful to never introduce a gameplay logic dependency on how something is rendered.
This is reasonable practice in any case to avoid surprising bugs.
Splitting your code into multiple crates can be useful to enforce this invariant as Cargo will help you catch cyclic dependencies, but doing so can seriously slow down iteration for small projects.

[Compiling Less Code]: ../releasing-projects/compiling-less-code.md

## Testing Graphical Output

As discussed above, testing graphical output is *hard*.
Sometimes however, you have no choice.

There are three problems to solve:

1. **Capturing data** from the screen programmatically.
2. **Running without a real GPU**, since most CI providers don't offer one.
3. **Evaluating the result**, when "correct" is subjective, resistant to computer analysis and changes constantly.

Getting all of this working is fiddly.
We'll cover the concepts and point you to the right tools, but expect to dig into external documentation and spend hours troubleshooting problems to get a setup you're happy with.

### Capturing Graphical Data

Bevy provides the [`Screenshot`] component for programmatic screen capture.
Spawn an entity with a `Screenshot` component and attach an observer to handle the result:

```rust, hide-lines=1
# use bevy::prelude::*;
fn take_screenshot(mut commands: Commands) {
commands
    .spawn(Screenshot::primary_window())
    .observe(save_to_disk("screenshot.png"));
}
```

[`save_to_disk`] writes the captured image to a file.
If you need the raw pixel data, observe the [`ScreenshotCaptured`] event directly to get the [`Image`].
The [`screenshot`] example has a complete demonstration.

For CI pipelines, Bevy provides a [`bevy_ci_testing`] plugin that reads a RON configuration file and dispatches events (including capturing screenshots and executing app exit) at specific frame numbers.
This is gated behind the `bevy_ci_testing` feature flag.
Here's the `.ron` file and `bash` command that Bevy uses to test its own rendering examples in CI:

```ron
(
    // Fix the timestep so animations are deterministic
    frame_time: Some(0.0166),
    events: [
        (100, Screenshot),
        (101, AppExit),
    ],
)
```

```bash
CI_TESTING_CONFIG=ci_config.ron cargo run --example my_example --features bevy_ci_testing
```

`bevy_ci_testing` can also take named screenshots, record video, move the camera, and handle custom events.
See its [API documentation][`bevy_ci_testing`] for details.

If you need to render without any window at all (say, on a build server with no display server) Bevy's [`headless_renderer`] example shows how to render to an offscreen texture using `RenderTarget::Image` with `WinitPlugin` disabled.

[`Screenshot`]: https://docs.rs/bevy/latest/bevy/render/view/window/screenshot/struct.Screenshot.html
[`save_to_disk`]: https://docs.rs/bevy/latest/bevy/render/view/window/screenshot/fn.save_to_disk.html
[`ScreenshotCaptured`]: https://docs.rs/bevy/latest/bevy/render/view/window/screenshot/struct.ScreenshotCaptured.html
[`Image`]: https://docs.rs/bevy/latest/bevy/image/struct.Image.html
[`screenshot`]: https://github.com/bevyengine/bevy/blob/main/examples/window/screenshot.rs
[`bevy_ci_testing`]: https://docs.rs/bevy/latest/bevy/dev_tools/ci_testing/index.html
[`headless_renderer`]: https://github.com/bevyengine/bevy/blob/main/examples/app/headless_renderer.rs

### Running Graphical Tests in CI

Most CI runners don't have GPUs.
To render anything at all, you need two things:
- A **software renderer** to emulate the GPU on the CPU,
- And on Linux, a **virtual display** to satisfy windowing requirements.

#### Software Renderers

Software renderers do all GPU work on the CPU.
They are much slower than real hardware, but they produce actual rendered output and will catch real rendering bugs.

Your options vary by platform:

| Platform | Graphics API | Software Renderer        | Notes                                        |
| -------- | ------------ | ------------------------ | -------------------------------------------- |
| Linux    | Vulkan       | [Lavapipe]               | Install via `mesa-vulkan-drivers`            |
| Windows  | DX12         | [WARP]                   | Built into Windows                           |
| macOS    | Metal        | None generally available | Some GitHub MacOS runners have physical GPUs |

In CI, you'll usually want to force the use of a software renderer using environment variables:

```bash
# Force a specific backend
WGPU_BACKEND=vulkan cargo run --example my_example

# Or select a specific adapter by name
WGPU_ADAPTER_NAME=llvmpipe cargo run --example my_example
```

You can test software rendering locally by setting [`WgpuSettings`]:

```rust, hide-lines=1-3
# use bevy::prelude::*;
# use bevy::render::settings::{RenderCreation, WgpuSettings};
# use bevy::render::RenderPlugin;
App::new()
    .add_plugins(DefaultPlugins.set(RenderPlugin {
        render_creation: RenderCreation::Automatic(WgpuSettings {
            force_fallback_adapter: true,
            ..default()
        }),
        ..default()
    }))
# ;
```

#### Headless Display on Linux

On Linux, creating a window requires a display server (X11 or Wayland), which CI runners typically don't have.
[`xvfb-run`] wraps your command with a virtual X11 framebuffer:

```bash
sudo apt-get install xvfb
xvfb-run cargo run --example my_example --features bevy_ci_testing
```

#### Self-hosted Runners

Software renderers work, but they're slow and don't behave identically to real GPUs.
Hosting your own CI runners on machines with actual graphics hardware is more common in gamedev than in other parts of software development.
A desktop with a mid-range GPU under a desk is enough to run graphical tests on real hardware, and the results will be far more representative of what your players actually see.

Both [GitHub Actions] and [GitLab CI] support self-hosted runners.
The setup is straightforward: install the runner agent on a machine with a GPU and a display (or `xvfb`), register it with your CI provider, and tag the tests that need real hardware to use that runner.

The tradeoff is maintenance.
You're responsible for keeping the machine running, updated, and secure.
Software renderers on a managed CI provider are low-maintenance by comparison, so most teams start there and only move to self-hosted hardware when software rendering becomes a bottleneck or isn't representative enough.

[GitHub Actions]: https://docs.github.com/en/actions/hosting-your-own-runners
[GitLab CI]: https://docs.gitlab.com/runner/install/

#### Putting It All Together

Setting up graphical CI is fiddly and very platform-specific.
Rather than documenting every combination here, we recommend studying Bevy's own CI workflows in [`.github/workflows/`] in the Bevy repository.
They test examples across Linux (lavapipe + `xvfb`), Windows (WARP), and macOS (real Metal GPU), capturing screenshots and uploading them for visual comparison.
Treat them as your reference implementation.

[Lavapipe]: https://docs.mesa3d.org/drivers/llvmpipe.html
[WARP]: https://learn.microsoft.com/en-us/windows/win32/direct3darticles/directx-warp
[`WgpuSettings`]: https://docs.rs/bevy/latest/bevy/render/settings/struct.WgpuSettings.html
[`xvfb-run`]: https://manpages.ubuntu.com/manpages/noble/man1/xvfb-run.1.html
[`.github/workflows/`]: https://github.com/bevyengine/bevy/tree/main/.github/workflows

### Evaluating Graphical Output

Once you can capture screenshots, you need to decide whether what you captured is *correct*.
This is where things get genuinely difficult, and your options range from trivial to elaborate.

#### Smoke Testing

The simplest graphical test is: **does it run without crashing?**
Many rendering bugs show up as panics, GPU validation errors, or hangs.
A test that boots your application, runs for a few hundred frames, and exits cleanly will catch more regressions than you'd expect (with zero evaluation logic as well!).

This practice is called **smoke testing**, and is what `bevy_ci_testing` with an `AppExit` event gives you.
This is where you should start.

#### Visual Regression Testing

The next step up is comparing screenshots against known-good baselines.
Capture a screenshot at a deterministic point (fixed scene, fixed frame count, fixed timestep), compare it pixel-by-pixel against a stored reference image, and flag differences that exceed a threshold.

This catches rendering regressions (broken shaders, missing textures, wrong transforms, etc.) without you writing any evaluation logic.
The catch is that it requires **deterministic rendering**, which is harder than it sounds.

Animations, particle systems, physics, and floating-point differences across GPU vendors all produce frame-to-frame variation that will break naive pixel-exact comparison.
To make visual regression work in practice:

- **Fix your timestep.** Use `bevy_ci_testing`'s `frame_time` option or [`TimeUpdateStrategy::ManualDuration`] so animations always land on the same frame.
- **Set random seeds:** If there is *any* randomness in your game or scene, set a fixed random seed every time.
- **Use static scenes.** Capture after everything has settled — no active animations, no asset loading weirdness, no randomness.
- **Set a difference threshold.** Anti-aliasing and floating-point rounding differ across platforms. Allow some pixel deviation rather than demanding exact matches.
- **Compare per-platform.** Each GPU, and each software renderer will produce slightly different output. Don't compare Linux screenshots against macOS ones.

For this, Bevy uses [Pixel Eagle], an external service that stores screenshots (per commit and per platform) and compares them against `main`.
If a PR changes rendering output, a bot comments on the PR with a link to the visual diff.

For your own project you could use Pixel Eagle, or you can build something simpler with an image comparison library like [`nv-flip-rs`].

In the end, **visual regression testing is for unexpected changes, not for correctness.**
A human still has to verify that the baseline looks right.
The automation just tells you when something *changed*.
You decide whether that change is a bug or an improvement.

#### The Siren's Call of Computer Vision

What we *really* want is to ask human-legible questions like "does this look okay?" or "are any UI elements occluded?" to an all-knowing machine who will automatically answer this for us.
In the modern era, you might think to turn to **computer vision**, the field of machine learning designed to answer exactly such questions.

However, computer vision is currently a poor fit for this task, at least for small teams.
Simply calling public APIs has three problems:

1. They have high marginal costs: CI needs a *ton* of screenshots analyzed for a production game.
2. They are not trained on these tasks, and may struggle to identify the objects you care about or cope with your artistic style.
3. They may change out from underneath you, as a new version of the model rolls out.

Training your own model to do this (probably via finetuning) may seem appealing, but is not for the faint of heart.
There are a huge number of potential failure modes: your style and features are a moving target, false positives are frustrating time-sinks, the investment in both expertise and hardware needed to do this well is quite large, etc.

Like always, keep your testing as simple as possible.

#### Know When To Test Logic Instead

Before investing in graphical testing infrastructure, ask yourself: **can this be tested without looking at pixels?**

_"Is the health bar at 50%?"_ is easier to verify by checking the `Life` component than by analyzing a screenshot.
_"Did the player's sword swing?"_ is better tested by asserting on animation state than by diffing frames.

Games have a natural split between simulation (what *is* happening) and presentation (what it *looks like*).
The techniques in [Testing with the ECS](#testing-with-the-ecs) cover the simulation side cheaply and reliably.
Save graphical testing for problems that are fundamentally graphical.

[Pixel Eagle]: https://pixel-eagle.com
[`nv-flip-rs`]: https://github.com/gfx-rs/nv-flip-rs
[`TimeUpdateStrategy::ManualDuration`]: https://docs.rs/bevy/latest/bevy/time/enum.TimeUpdateStrategy.html
