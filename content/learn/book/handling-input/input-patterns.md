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
After all, observers will react immediately as opposed to waiting for the input `Messages` to update each frame.
Surely input would be better handled with events and observers rather than `Messages`?

Not quite.

Observers can be triggered multiple times within a single frame.
If the data causing the observer to trigger isn't kept in check, the tidal wave of resulting logic could massively impact the performance of your game.
Observers are also sequentially evaluated, while `Messages` can be evaluated concurrently.
This allows our systems that access input `Messages` to run much faster overall than if we were triggering observers individually for each input received.

This isn't to say the you can't use both in your systems.
If there's some logic that you want to call from both receiving input data and from triggering an event, then combining input messages and observers can work for this purpose.
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
2. Read an input `Message` and trigger our `Observer` in response.
3. The `Observer` will call a one-shot system to be run.

Input messages simplifies activating one-shot systems by removing a redundant step.
Toggling UI boxes, initiating interactions with NPC characters, and adding a new player when a controller is connected are all situations where one-shot systems could be triggered by input messages.

We can see this in the example below, where we're running a `toggle_weapon_sights` system whenever the `RightMouseButton` is pressed.

```rust
// This system will read input events from `MouseButtonInput`.
fn activate_toggle_weapon_sights(
    mut commands: Commands,
    input_event_reader: MessageReader<MouseButtonInput>
) {
    for event in input_event_reader {
        if event.button == MouseButton::Right && 
            event.state == ButtonState::Pressed {
            commands.run_system(toggle_weapon_sights);
        }
    }
}
```

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

Input [`Messages`] are sent every frame, which means that its possible for frame drops and lag to introduce gaps where there is no player input being received by the game.
This also means that if we're using timers and cooldowns inside of systems that will only run based on input, we have to be aware of the fact that those timers and cooldowns won't function properly unless we manually correct them.

Most of what we'll cover here can also be found in more depth over in the [Time and Timers] page within The Game Loop chapter, so if there are concepts that aren't making sense or you just want to read more, be sure to check that page out as well.

### Utilizing Delta Time with Input

Fixing the discrepancies between frame rate and consistent game systems is a fundamental aspect of game design.
Games usually don't run at a consistent frame rate, but players do experience a consistent flow of time.
If the what the game displays and what the player is perceiving don't align, it can lead to an unexpected (and likely negative) experience.
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
    
    if input.just_pressed(KeyCode::W) {
        player_transform.translation.x += PLAYER_SPEED * time.delta_secs();
    }
}
```

However, you should be aware that if a player encounters large frame drops, this system has the potential to overcorrect.
Further modifications to account for these types of situations are beyond the scope of this book, however it is something to be aware about.

["delta time"]: https://en.wikipedia.org/wiki/Delta_timing

### Timers and Input

[`Timers`] and [`Stopwatches`] allow us to run code at set intervals, and can pair nicely with input data.
However, you have to be careful when using these together, as a `Timer`'s (and a `Stopwatch`'s) functionality is tied to how often they are updated.
As we discussed in the previous section, input data is sent every frame, which means any logic tied to input data isn't guaranteed to run at a consistent rate.

In the [Pressed vs Just Pressed] section of the Using Input page, we established a scenario where we wanted to create a system that would allow the player to repeatedly use a weapon attack after a cooldown had occurred.
Utilizing a system that read mouse input, we initiated, ticked (progressed), and then adjusted a timer based on whether `LeftMouseButton` was being pressed.
However, a key flaw was that the timer's  [`tick`] was tied to the input data.
If we wanted a truly consistent timer, we need to fix this.

At the end of that example, the code looked like this:

```rust
// A component indicating a weapon.
#[derive(Component)]
struct Weapon;

// A component indicating how much time needs to pass in-between weapon attacks.
#[derive(Component)]
struct WeaponAttackInterval(pub Timer);

fn weapon_attack(
    button_input: Res<ButtonInput<MouseButton>>,
    mut player_weapon: Single<(&Weapon, &mut WeaponAttackInterval), With<PlayerWeapon>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_secs();
    
    // Check the state of the WeaponAttackInterval timer if it's active.
    if player_weapon.1.is_finished() != true {
        // Progress the WeaponAttackInterval.
        player_weapon.1.tick(delta_time);
    }
    
    // The initial LeftMouseButton press.
    if button_input.just_pressed(MouseButton::LeftMouseButton) {
        // Perform an initial weapon attack.
        player_weapon.0.attack();
        // Create a new the timer within `WeaponAttackInterval` that will run.
        player_weapon.1.from_seconds(1.5, TimerMode::Repeating);
    }
    
    // If LeftMouseButton is pressed and our WeaponAttackInterval has completed, attack.
    if button_input.pressed(MouseButton::LeftMouseButton) && player_weapon.1.just_finished() {
        player_weapon.0.attack();
    }
    
    // If LeftMouseButton was released, switch the timer mode to expire.
    if button_input.just_released(MouseButton::LeftMouseButton) {
        player_weapon.1.set_mode(TimerMode::Once);
    }
}
```

The solution to fixing this is simple: our `WeaponAttackInterval` timer `tick` needs to occur independently of the `weapon_attack` system.
We can set up a `weapon_attack_tick` system to run in the [`FixedUpdate`] schedule, meaning that it will update consistently.
Within `weapon_attack_tick` we'll place our `WeaponAttackInterval` timer `tick` update, removing it from the `weapon_attack` system.
Now, the whole weapon attack setup looks like this:

```rust
// The plugin where we add our systems to the game.
fn weapon_plugin(mut app: &mut App) {
    app.add_systems(FixedUpdate, weapon_attack_tick);
    app.add_systems(Update, weapon_attack);
    ...
}
// A system which consistently updates the `tick` of the player weapon.
fn weapon_attack_tick(
    mut player_weapon: Single<&mut WeaponAttackInterval, With<PlayerWeapon>>,
    time: Res<Time>
) {
    if player_weapon.0.remaining() > 0 {
        player_weapon.0.tick(time.delta_secs());
    }
}
// A system which will perform the weapon attacks and adjust the timer.
fn weapon_attack(
    button_input: Res<ButtonInput<MouseButton>>,
    mut player_weapon: Single<(&Weapon, &mut WeaponAttackInterval), With<PlayerWeapon>>,
) {
    if button_input.just_pressed(MouseButton::LeftMouseButton) {
        player_weapon.0.attack();
        player_weapon.1.from_seconds(1.5, TimerMode::Repeating);
    }
    
    if button_input.pressed(MouseButton::LeftMouseButton) && player_weapon.1.just_finished() {
        player_weapon.0.attack();
    }
    
    if button_input.just_released(MouseButton::LeftMouseButton) {
        player_weapon.1.set_mode(TimerMode::Once);
    }
}
```

[Pressed vs Just Pressed]: /learn/book/handling-input/using-input#pressed-versus-just-pressed

[`Timers`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Timer.html#method.tick
[`Stopwatches`]: https://docs.rs/bevy/latest/bevy/time/struct.Stopwatch.html
[`tick`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Timer.html#method.tick
[`FixedUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.FixedUpdate.html
