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

### Observers Vs Input

[Observers (and the events that trigger them)] might be the first thing that comes to mind when reacting to changes in your game.
After all, observers will react immediately as opposed to waiting for the input `Messages` to update each frame.
Surely input would be better handled with events and observers rather than `Messages`?

Not quite.

Observers can be triggered multiple times within a single frame.
If the data causing the observer to trigger isn't kept in check, the tidal wave of resulting logic could massively impact the performance of your game.
Observers are also sequentially evaluated, while `Messages` can be evaluated concurrently.
This allows our systems that access input `Messages` to run much faster overall than if we were triggering observers individually for each input received.

This isn't to say the you can't use both in your systems.
If there's some logic that you want to call from both input data and from events, then combining input messages and observers can work for this purpose.
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

### Activating One-shot Systems with Input

In a similar way, input messages can also make activating one-shot systems incredibly straightforward.
We can see this in the example below, where every click of the left mouse button will queue a `mouse_click_system` to be run.
Toggling UI boxes, initiating interactions with NPC characters, and adding a new player when a controller is connected are all situations where one-shot systems could be triggered by input messages.

```rust
// This system will read input events from `MouseButtonInput`.
fn run_system_on_click(
    mut commands: Commands,
    input_event_reader: MessageReader<MouseButtonInput>
) {
    for event in input_event_reader {
        if event.button == MouseButton::Left && 
            event.state == ButtonState::Pressed {
            commands.run_system(registered_system);
        }
    }
}
```

[Observers (and the events that trigger them)]: /learn/book/control-flow/events

### Input System Conditions

Input data can also be used as a conditional check for deciding when a system should run.
Instead of needing to manually evaluate input data within a system, we can use several built-in functions to check whether a system should run.
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
If it has, then the `jump` system is ran.
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
This condition uses XOR bitwise logic, taking in a button to check the state of (using `just_pressed`) and a `bool` value to compare against.
If either the `bool` you pass in or the button you want to check evaluate to true, then the system will run.
However, if both the `bool` and the evaluated button state are equivalent, then the system will not run.

In the example below, we only want to run the `pause_menu` system if the `Escape` key is pressed.
We'll pass in a `false` value alongside `KeyCode::Escape` to ensure that `pause_menu` will only run when `KeyCode::Escape` is pressed.

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

[`input_just_pressed`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_just_pressed.html
[`input_pressed`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_pressed.html
[`input_just_released`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_just_released.html
[`input_toggle_active`]: https://docs.rs/bevy/latest/bevy/input/common_conditions/fn.input_toggle_active.html

## Input & Time
