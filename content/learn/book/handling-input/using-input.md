+++
title = "Using Input"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Bevy handles input data the same way for each supported device.
When a device creates input events (for example, when a button is pressed or the mouse is moved), Bevy will process those events and make them easily available for you to use in your games.

```rust
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' was just pressed");
    }
}
```

Bevy uses [Winit] (via [`bevy_winit`], Bevy's conversion layer for Winit) to initially turn the input event into a [`Message`].
These messages are then processed and placed into a resource that we can read and use for setting up movement, tracking aim, activating abilities, or any other input-based actions you'd like to set up.

[Winit]: https://crates.io/crates/winit
[`bevy_winit`]: https://docs.rs/bevy/latest/bevy/winit/index.html

[`Message`]: https://docs.rs/bevy/latest/bevy/ecs/message/trait.Message.html
[`ButtonInput`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html

## Accessing Button-like Inputs

It's likely that you'll be working with the [`ButtonInput`] resource more frequently than other input resources.
This is because Bevy stores all "button-like" input data in an input-specific [`ButtonInput`] resource.

{% callout(type="info") %}

### What Makes An Input "Button-like"?

When you start looking at each input device, you might wonder what makes an input "button-like"?

It's actually very straightforward.
To be considered "button-like" in Bevy, the input has to be "press-able".
This means that Bevy can register the state of the input as either `pressed` or `released`.
Both of these values are explicitly stored in a [`ButtonState`] enum, which is recorded as a part of every "button-like" input event.

Something like a joystick can be pressed if the joystick can be "clicked".
If it can be clicked, then the joytstick button data will be accessible in a `ButtonInput` resource.
However, the direction you move the joystick in is not "press-able", and therefore is not "button-like" and will not be accessible in a `ButtonInput` resource.

[`ButtonState`]: https://docs.rs/bevy/latest/bevy/input/enum.ButtonState.html
{% end %}

We can directly access an input-specific [`ButtonInput`] struct through a system parameter.
This will provide us with tools that can help us set up the exact situations that our gameplay requires.
For example, `ButtonInput` provides us with a number of methods that will return a `bool` based on if the button has just been pressed ([`just_pressed`]), is currently being pressed ([`pressed`]), or if its just been released ([`just_released`]).

```rust
// This system provides access to KeyCode input data from a `ButtonInput` resource.
fn keyboard_input_system(
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    if keyboard_input.pressed(KeyCode::KeyA) {
        info!("'A' is currently being pressed");
    }
    if keyboard_input.just_pressed(KeyCode::KeyA) {
        info!("'A' was just pressed");
    }
    if keyboard_input.just_released(KeyCode::KeyA) {
        info!("'A' was just released");
    }
}
```

[`pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.pressed
[`just_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_pressed
[`just_released`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_released

### Pressed Versus Just Pressed

Although it might appear like the difference between [`pressed`] and [`just_pressed`] is negligible, the two are quite distinct.
While both signal a `ButtonInput` button being activated, `pressed` is continuously `true` until the input is released.
`just_pressed` will only be `true` for _a single frame_ after the input is activated.
The same is true for [`just_released`], which will only be `true` for a single frame after the input is deactivated.

To see the differences, think of using a weapon in any action game.
Unless the game is based solely on precision, there will likely be moments where players will want to repeatedly use their weapon without having to also repeatedly press a button.
In this case, we can use `pressed` to check if the weapon attack button is pressed, and if it is, then the weapon can be repeatedly used.

```rust
// A component indicating a weapon.
#[derive(Component)]
struct Weapon;

// A component indicating how much time needs to pass in-between weapon attacks.
#[derive(Component)]
struct WeaponAttackInterval(pub Timer);

fn repeated_weapon_attack(
    button_input: Res<ButtonInput<MouseButton>>,
    player_weapon: Single<(&Weapon, &WeaponAttackInterval), With<PlayerWeapon>>
    time: Res<Time>,
) {
    let delta_time = time.delta();
    
    // Progress the WeaponAttackInterval.
    player_weapon.1.tick(delta_time);
    
    // If LeftMouseButton is pressed and our WeaponAttackInterval has completed, attack.
    if button_input.pressed(MouseButton::LeftMouseButton) && player_weapon.1.just_finished() {
        player_weapon.0.attack();
    }
}
```

However, we don't want the cooldown timer within the `WeaponAttackInterval` component to be running arbitrarily.
We only want it to run after the player has made an initial attack.
To start the timer in `WeaponAttackInterval`, we can now use `just_pressed` to start the cooldown timer after an initial attack is made.

```rust
fn weapon_attack(
    button_input: Res<ButtonInput<MouseButton>>,
    mut player_weapon: Single<(&Weapon, &mut WeaponAttackInterval), With<PlayerWeapon>>
) {
    if button_input.just_pressed(MouseButton::LeftMouseButton) {
        // Perform an initial weapon attack.
        player_weapon.0.attack();
        // Create a new the timer within `WeaponAttackInterval` that will run.
        player_weapon.1.0 = Timer::from_seconds(1.5, TimerMode::Repeating);
    }
}
```

Finally, we can extend this mechanic even further by using `just_released` to change the `WeaponAttackInterval` timer mode to `Once` when the player finally releases the attack button.
This will still let the timer tick down and finish.

```rust
fn cancel_weapon_attack_timer(
    button_input: Res<ButtonInput<MouseButton>>,
    mut player_weapon: Single<&mut WeaponAttackInterval, With<PlayerWeapon>>
) {
    if button_input.just_released(MouseButton::LeftMouseButton) {
        player_weapon.0.set_mode(TimerMode::Once);
    }
}
```

Since all three of these systems are interacting with the same data, we can combine them into one single `weapon_attack` system that can handle each scenario that might occur when mouse input is received.

```rust
fn weapon_attack(
    button_input: Res<ButtonInput<MouseButton>>,
    mut player_weapon: Single<(&Weapon, &mut WeaponAttackInterval), With<PlayerWeapon>>,
    time: Res<Time>,
) {
    let delta_time = time.delta();
    
    // Check the state of the WeaponAttackInterval timer if it's active.
    if !player_weapon.1.is_finished() {
        // Progress the WeaponAttackInterval.
        player_weapon.1.tick(delta_time);
    }
    
    // The initial LeftMouseButton press.
    if button_input.just_pressed(MouseButton::LeftMouseButton) {
        // Perform an initial weapon attack.
        player_weapon.0.attack();
        // Create a new the timer within `WeaponAttackInterval` that will run.
        player_weapon.1.0 = Timer::from_seconds(1.5, TimerMode::Repeating);
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

### Button Combinations

Additionally, we aren't limited to only accessing one button input at a time.
We're able to create button combinations by accessing multiple buttons within a given `ButtonInput`.
Using the [`any_pressed`], [`any_just_pressed`], or [`any_just_released`] methods allow us to establish AND logic for handling combinations.
Alternatively, we could use the [`all_pressed`], [`all_just_pressed`], and [`all_just_released`] methods and supply a list of button inputs instead.

```rust
// This system prints when `Ctrl + Shift + A` is pressed.
fn and_keyboard_combo(input: Res<ButtonInput<KeyCode>>) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if ctrl && shift && input.just_pressed(KeyCode::KeyA) {
        info!("Just pressed Ctrl + Shift + A!");
    }
}
// This system requires that `AltLeft` and `F4` are pressed.
fn strict_keyboard_combo(input: Res<ButtonInput<KeyCode>>) {
    if input.all_just_pressed([KeyCode::AltLeft, KeyCode::F4]) {
        info!("The game will shut down now!");
    }
}
```

We can also access a collection of every button that is being interacted with on a given frame.
Using [`get_pressed`], [`get_just_pressed`], or [`get_just_released`] will return an [`Iterator`] (specifically an [`ExactSizeIterator`]) of all currently pressed, just pressed, or just released buttons.
Any of these options can be preferable when processing a lot of input data at once since you can iterate over all relevant buttons.
As an example, if you want the `Escape` key to always be evaluated first (but not evaluated separately), we can use `get_pressed` to see if its being pressed.

```rust
fn get_all_pressed_buttons(input: Res<ButtonInput<KeyCode>>) {
    if input.get_pressed().iter().any(|key| key == KeyCode::Esc) {
        // Pause the game, open the menu.
    }
}
```

[`any_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.any_pressed
[`any_just_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.any_just_pressed
[`any_just_released`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.any_just_released
[`all_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.all_pressed
[`all_just_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.all_just_pressed
[`all_just_released`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.all_just_released
[`get_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.get_pressed
[`get_just_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.get_just_pressed
[`get_just_released`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.get_just_released
[`Iterator`]: https://doc.rust-lang.org/nightly/core/iter/trait.Iterator.html
[`ExactSizeIterator`]: https://doc.rust-lang.org/nightly/core/iter/trait.ExactSizeIterator.html

### Resetting Button Inputs

Since `ButtonInput` is accessed through a `Resource`, we also have the ability to alter this data through a `ResMut` system parameter.
This can be especially helpful if we want to clear all input from a specific key or button, or even reset all input from the entire device.
The [`clear`], [`clear_just_pressed`], and [`clear_just_released`] methods allow us to remove the current state of a button input.
For example, if we use `clear_just_pressed` on a button input, we won't receive a `true` value from calling `just_pressed` on that button input until a new button press occurs.

```rust
// Clear the `just_pressed` state of Left MouseButton.
fn clear_a_mouse_click(mut mouse_clicks: ResMut<ButtonInput<MouseButton>>) {
    if mouse_clicks.just_pressed(MouseButton::Left) {
        mouse_click.clear_just_pressed(MouseButton::Left);
    }
}
```

Additionally, if we want to go one step further and completely reset a button state, we have the [`reset`] and [`reset_all`] methods which will completely reset the state of either a single button or all buttons.

```rust
// Reset all MouseButton states.
fn clear_all_mouse_clicks(mut mouse_clicks: ResMut<ButtonInput<MouseButton>>) {
    if mouse_clicks.get_pressed().len() != 0 {
        mouse_clicks.reset_all();
    }
}
```

Finally, we also have the [`release`] and [`release_all`] methods which will register a release event for either a single button, or for all buttons.
`release` allows you to generate a release event for a specific button, while `release_all` will create release events for every button on the device.
These methods can be helpful in situations where the player continuously holding a button press would cause issues for your game.

```rust
// We only want this event to be triggered once.
#[deriver(Event)]
struct SingleTriggerEvent;

fn only_trigger_once(mut input: ResMut<ButtonInput<MouseButton>>, mut commands: Commands) {
    if input.just_pressed(MouseButton::LeftMouseButton) {
        // As soon as `LeftMouseButton` is pressed, release it.
        input.release(MouseButton::LeftMouseButton);
        commands.trigger(SingleTriggerEvent);
    }
}
```

[`clear`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.clear
[`clear_just_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.clear_just_pressed
[`clear_just_released`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.clear_just_released
[`reset`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.reset
[`reset_all`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.reset_all

[`release`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.release
[`release_all`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.release_all
