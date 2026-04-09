+++
title = "Input Data Patterns"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Interacting with input data can be done in many forms.
This page is meant to illustrate some of the ways you can combine input data with other tools that Bevy provides, along with highlighting some patterns to avoid if possible.

## Reacting To Input

The obvious application of input data is to run systems and update the game based on player input.
Given that input data is based on using [`Messages`], it only interacts with a small part of the various [control flow] tools that Bevy provides.
Lets compare handling input data with some other tools that are also used for running systems and updating values in a Bevy game.

[control flow]: /learn/book/control-flow

[`Messages`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Messages.html

### Observers Vs Input

[Observers (and the events that trigger them)] might be the first thing that comes to mind when reacting to changes in your game.
After all, observers will react immediately as opposed to waiting for the input data to update each frame.
Surely input would be better handled with events and observers rather than collecting `Messages` and moving them into a resource?

Not quite.

Observers can be triggered multiple times within a single frame.
If the data causing the observer to trigger isn't kept in check, the tidal wave of resulting logic could massively impact the performance of your game.
Observers are also sequentially evaluated, while `Messages` can be evaluated and placed into their respective resources concurrently.
This allows our systems that access `ButtonInput` to run much faster overall than if we were triggering observers individually for each input received.

This isn't to say the you can't use both in your systems.
If there's some logic that you want to call from both receiving input data and from triggering an event, then combining `ButtonInput` and observers can work for this purpose.
Take the example below, where we spawn a new unit by triggering an observer.
Receiving a `LeftMouseButton` input will trigger the observer, but we'll also trigger the observer when we're setting up our app.

```rust
// An event that will spawn a new unit.
#[derive(Event)]
struct PlaceUnitOnMap;

// A resource that tells us how many units to spawn on the map initially.
#[derive(Resource)]
struct SetupParameters {
    pub initial_units: i32,
    ...
}

// A system reading input data which will call a `PlaceUnitOnMap` event.
fn trigger_observer(mouse_input: Res<ButtonInput<MouseButton>>, mut commands: Commands) {
    if mouse_input.just_pressed(MouseButton::LeftMouseButton) {
        commands.trigger(PlaceUnitOnMap);
    }
}

// A system that will run on Startup and calls `PlaceUnitOnMap` a specific number of times.
fn setup_initial_units(mut commands: Commands, setup_parameters: Res<SetupParameters>) {
    for i in 0..setup_parameters.initial_units {
        commands.trigger(PlaceUnitOnMap);
    }
}

// The observer which will react to `PlaceUnitOnMap` and spawn the unit.
world.add_observer(|event: On<PlaceUnitOnMap>, mut commands: Commands) {
    commands.spawn(NewUnitBundle);
}
```

[Observers (and the events that trigger them)]: /learn/book/control-flow/events

### Activating One-shot Systems with Input

Continuing with the observer comparison, directly running one-shot systems with input can skip an entire processing step when compared to triggering an observer.
If we wanted to use an observer, we would have to:

1. Spawn our `Observer` and the `Event` that will trigger it.
2. Read a `ButtonInput`, and see if the key we want is being pressed.
3. If the button is pressed, trigger our `Observer` in response.
4. The `Observer` will then queue a one-shot system to be run.

If instead we run the one-shot system based on input alone, we'll simplify our systems by removing a redundant step.
Toggling UI boxes, initiating interactions with NPC characters, and adding a new player when a controller is connected are all situations where one-shot systems could be triggered by input directly.

We can see this in the example below, where we're running a `toggle_weapon_sights` system whenever the `RightMouseButton` is pressed.

```rust
// This system will read input events from `MouseButtonInput`.
fn activate_toggle_weapon_sights(
    mut commands: Commands,
    button_input: Res<ButtonInput<MouseButton>>,
) {
    if button_input.just_pressed(MouseButton::Right) {
        commands.run_system(toggle_weapon_sights);
    }
}
```

### Input System Conditions

Input data can also be used as a conditional check for deciding when a system should run.
If the input data is only meant to signal that a system should run (instead of being needed inside the system itself), we can use several built-in functions to check whether a system should run.
These functions will return a boolean based on the state of a specific input button, and require us to pass in a type that can be accessed from a `ButtonInput` type.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, jump.run_if(input_just_pressed(KeyCode::Space)))
        .run();
}
```

In the above example, [`input_just_pressed`] is a function that will evaluate whether `KeyCode::Space` has just been pressed.
If it has been pressed, then the `jump` system is ran once in the `Update` schedule.
Even though we aren't accessing `ButtonInput` directly, our conditional function (`input_just_pressed`) is accessing the `just_pressed` method on the `ButtonInput` that contains all of the `KeyCode` input data.
The same process occurs when we use the other input conditional functions: [`input_pressed`] and [`input_just_released`].

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // `jump` if the Space key has just been pressed.
        .add_systems(Update, jump.run_if(input_just_pressed(KeyCode::Space)))
        // Repeat a weapon attack if LeftMouseButton is currently being pressed.
        .add_systems(Update, weapon_repeated_attack.run_if(input_pressed(MouseButton::LeftMouseButton)))
        // Unscope the weapon if RightMouseButton has been released.
        .add_systems(Update, unscope_weapon.run_if(input_just_released(MouseButton::RightMouseButton)))
        .run();
}
```

We also have one more system condition function that relates to input: [`input_toggle_active`].
This condition takes in a button to check the state of (using the `just_pressed` method) and a `bool` value.
When you press the button you passed into `input_toggle_active` for the first time, the system will switch from the initial `bool` value you pass in.
If you initially passed in `true` to have the system run from its initialization, `input_toggle_active` will switch to `false` and the system will stop running.
Likewise, if you initially passed in `false`, the system will start running.
Subsequent presses of the specified button will continue this pattern, toggling the system from on to off and off to on with each alternating press.

In the example below, we only want to run the `pause_menu` system if the `Escape` key is pressed.
We'll pass in a `false` value alongside `KeyCode::Escape` to indicate that `pause_menu` starts disabled.
When we first press `KeyCode::Escape`, the `pause_menu` system will run.
If we press it a second time, `pause_menu` will stop running.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Update, pause_menu.run_if(input_toggle_active(false, KeyCode::Escape)))
        .run();
}

fn pause_menu() {
    println!("in pause menu");
}
```

{% callout(type="info") %}

#### Why Use Input System Conditions?

Given that the `run_if` method only requires a `SystemCondition`-satisfying system, you might wonder why we would want to use `input_just_pressed` or `input_just_released` when we could just check for the existence of an input `Message` with [`on_message`] in the first place.

`on_message` only checks for the existence of a new message.
It doesn't allow us to see the potential state of that message.
On the other hand, our input systems do allow us to explicitly check the state of the message, which will almost always be the preferred way of using input data for conditionally running systems.

[`on_message`]: https://docs.rs/bevy/latest/bevy/ecs/prelude/fn.on_message.html

{% end %}

[`input_just_pressed`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_just_pressed.html
[`input_pressed`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_pressed.html
[`input_just_released`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_just_released.html
[`input_toggle_active`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_toggle_active.html

## Input & Time

Input data can be accessed every frame, which means that any changes we make based on input data will be applied every frame.
This isn't an issue for one-shot logic, but if we have some functionality that repeats and needs to be consistent across the player's real-time experience, we begin to run into issues.

Most of what we'll cover here can also be found in more depth over in the [Time and Timers] page within The Game Loop chapter, so if there are concepts that aren't making sense or you just want to read more, be sure to check that page out as well.

[Time and Timers]: /learn/book/the-game-loop/time-and-timers

### Utilizing Delta Time with Input

Fixing the discrepancies between frame rate and consistent game systems is a fundamental aspect of game design.
Games usually don't run at a consistent frame rate, but players do experience a consistent flow of time.
If what the game displays and what the player is perceiving don't align, it can lead to an unexpected (and likely negative) experience.
To reconcile these, we use ["delta time"].

When incorporating delta time into input data, we have to think about what we're trying to adjust.
If we're trying to adjust player movement, we can just multiply the player's speed by the `delta_secs` value Bevy tracks internally.
This ensures that the player will be moving at a consistent rate between frames.

```rust
fn move_player(
    input: Res<ButtonInput<KeyCode>>,
    mut player_transform: Single<&mut Transform, With<Player>>,
    time: Res<Time>
){
    const PLAYER_SPEED: f32 = 60.;
    
    if input.pressed(KeyCode::W) {
        player_transform.translation.y += PLAYER_SPEED * time.delta_secs();
    }
}
```

However, there are also situations where delta time should not be applied.
For example, consider a player jumping.
How high a player can jump isn't affected by how long it took the last frame to be rendered; it's a set value.
The same can be said for any action that doesn't need to compensate for inconsistencies in the frame render time, like dashes, interactions, or attacks.

You should also be aware that if a player encounters large frame drops, accounting for delta time has the potential to overcorrect.
Further modifications to account for these types of situations are beyond the scope of this book, however its something to also be aware of.

["delta time"]: https://en.wikipedia.org/wiki/Delta_timing
