+++
title = "Utilizing Input"
insert_anchor_links = "right"
[extra]
weight = 1
+++

By definition, games are interactive.
Players will perform some action or reaction, and the game will react and change in return.
When it comes to video games specifically, we need to receive input from the player through some kind of device.
Whether it's controllers, keyboards, mice, or even touch screens, Bevy allows us to interact with input from all of these devices.

## Interacting with Input Devices

Bevy can read input from the aforementioned devices: gamepads (controllers), keyboards, mice, and touch inputs.
All of these devices are enabled through a [feature flag].
By default, larger groupings of features like profiles and collections will enable all input devices.
If you know that your game will not need touch input (or keyboard and mouse inputs if you're building a mobile game), you can disable these input devices by turning their feature flag off in your project `Cargo.toml` file.
See the [Selective Feature Use section] in the Compiling Less Code page for more details.

Once you start working with the actual input data, you'll find that Bevy handles all input in a similar way.
When the button on a gamepad or a key on a keyboard gets pressed, Bevy uses [Winit] (via [`bevy_winit`], Bevy's conversion tool for Winit) to initially turn the input into [`Messages`] that we can use inside our systems.

Input data from every device will follow the same process that is covered on this page, however you should know that each device type also has their own unique circumstances to be aware of.
Once you've read how input data is generally handled on this page, make sure to read a specific devices' page to see how their data is uniquely handled:

- [Keyboard Input](/learn/book/handling-input/keyboard-input)
- [Mouse Input](/learn/book/handling-input/mouse-input)
- [Gamepad Input](/learn/book/handling-input/gamepad-input)
- [Touch Input](/learn/book/handling-input/touch-input)

[Selective Feature Use section]: /learn/book/releasing-projects/compiling-less-code/#more-selective-feature-use

[Winit]: https://crates.io/crates/winit
[`bevy_winit`]: https://docs.rs/bevy/latest/bevy/winit/index.html
[feature flag]: https://docs.rs/bevy/latest/bevy/index.html#feature-list

## Input Messages

Bevy takes input data from a device and converts it to a [`Message`].
These messages can be [read like any other message] and are sent every frame.
Each input device type will send a unique message type: [`KeyboardInput`] for keyboards, [`MouseButtonInput`] for mouse button presses, and so on for each input type.
Each `Message` type will automatically be set up for each input device that has its feature flag enabled.
In addition, Bevy will also insert systems in the [`PreUpdate`] schedule to process and eventually clear each `Message` type.

Accessing input data in this manner gives us access to all of the regular functionality that messages provide, including [`MessageReader`], [`MessageWriter`], and [`MessageMutator`].

```rust
// This system reads and prints out all `KeyboardInput` messages.
fn keyboard_message_reader(keyboard_inputs: MessageReader<KeyboardInput>) {
    for keyboard_input in keyboard_inputs.read() {
        info!("{:?}", keyboard_input);
    }
}
// This system writes a new `KeyboardInput` message.
fn keyboard_message_writer(
    mut keyboard_inputs: MessageWriter<KeyboardInput>,
    window_query: Single<(Entity, &Window)>,
) {
    keyboard_inputs.write(
        KeyboardInput {
            key_code: KeyCode::W,
            logical_key: Key::Character("W"),
            state: ButtonState::Pressed,
            text: Some("W"),
            repeat: false,
            window: window_query.0,
        }
    )
}
// This system changes the right Ctrl, Alt, and Shift keys to their left versions.
fn keyboard_message_mutator(mut keyboard_inputs: MessageMutator<KeyboardInput>) {
    for message in keyboard_inputs.par_read() {
        if message.key_code == KeyCode::CtrlRight {
            message.key_code = KeyCode::CtrlLeft
        }
        if message.key_code == KeyCode::AltRight {
            message.key_code == KeyCode::AltLeft
        }
        if message.key_code == KeyCode::ShiftRight {
            message.key_code == KeyCode::ShiftLeft
        }
    }
}
```

Reading input messages can be beneficial in several situations.
If we're dealing with multiple types of input data, input messages can allow us to easily order the input data relative to each other within a single frame.

As an example, let's look at how we can handle multiple keys being pressed down at the same time.
This can be the case when using the W, A, S, and D keys for general movement alongside using the Shift or Ctrl keys for crouching.
Since crouching might have a slight delay before being fully active, we'll want to make sure that we're constantly moving first rather than crouching.
This ensures that the player won't be interrupted by crouching or sprinting while moving.

```rust
#[derive(Component)]
enum CrouchState {
    Uncrouched,
    Crouched
}

fn movement_plugin(mut app: &mut App) {
    // Using `.chain()` allows us to ensure that WASD movements get applied before Crouch movements.
    app.add_systems(FixedUpdate, (wasd_movement, crouching_system).chain());
}

fn wasd_movement(
    movement_input: MessageReader<KeyboardInput>,
    mut player_location: Single<&mut Transform, With<Player>>,
) {
    for input in movement_input.read() {
        // Match the input message based on what keyboard key is being pressed.
        match input.key_code {
            KeyCode::W => player_location.0.translation.y += 0.1,
            KeyCode::A => player_location.0.translation.x -= 0.1,
            KeyCode::S => player_location.0.translation.y -= 0.1,
            KeyCode::D => player_location.0.translation.x += 0.1,
            _ => continue;
        }
    }
}

fn crouching_system(
    movement_input: Messages<KeyboardInput>,
    mut player_crouch_state: Single<&mut CrouchState, With<Player>>,
) {
    for input in movement_input.read() {
        // Make the input data more easily accessible. 
        let (button_key, button_state) = (input.key_code, input.state);
        // We only care about crouch input data.
        if button_key != KeyCode::CtrlLeft {
            continue
        }
        // Crouching logic, with `Uncrouched` as a default response.
        match (button_state, player_crouch_state) {
            (ButtonState::Pressed, CrouchState::Uncrouched) => 
                *player_crouch_state = CrouchState::Crouched,
            (ButtonState::Unpressed, CrouchState::Crouched) => 
                *player_crouch_state = CrouchState::Uncrouched,
            _ => *player_crouch_state = CrouchState::Uncrouched,
        } 
    }
}
```

Because of the immediate nature of input messages, they also make activating one-shot systems incredibly straightforward.
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
// A system that will run when Left Mouse Button is clicked.
fn mouse_click_system() {
    println!("The Left Mouse Button was clicked!");
}
// Register a system that will be queued to run.
fn register_system(mut commands: Commands) {
    let registered_system = commands.register_system(mouse_click_system);
}
```

[read like any other message]: /learn/book/control-flow/messages

[`Message`]: https://docs.rs/bevy/latest/bevy/ecs/message/trait.Message.html
[`PreUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html
[`KeyboardInput`]: https://docs.rs/bevy/latest/bevy/input/keyboard/struct.KeyboardInput.html
[`MouseButtonInput`]: https://docs.rs/bevy/latest/bevy/input/mouse/struct.MouseButtonInput.html
[`MessageReader`]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageReader.html
[`MessageWriter`]: https://docs.rs/bevy/latest/bevy/prelude/struct.MessageWriter.html
[`MessageMutator`]: https://docs.rs/bevy/latest/bevy/ecs/message/struct.MessageMutator.html

## ButtonInput Resources

Once most input messages are processed, they're placed into an associated [`ButtonInput`] resource that we can directly access via a system parameter.
Input data is added to a `ButtonInput` via the same systems that process and clear the input `Message` data, meaning we'll only have access to the input data as long as the original input `Message` is still available.

Interacting with input data this way gives us access to a variety of methods which provide more information about the state of the input and give us more control over how we respond to it. For example, accessing a [`ButtonInput`] resource provides us with a number of methods that will return a `bool` based on if the button [has just been pressed], [is currently being pressed], or [if its just been released].

```rust
// This system provides access to the KeyCodes in the `ButtonInput` resource.
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

Additionally, we aren't limited to only accessing one button input at a time.
We can access mouse input data and keyboard input data at the same time, and use both in our systems if needed.

```rust
// This system prints when `Ctrl + LeftMouseButton` is pressed.
fn keyboard_and_mouse_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>
) {
    let mouse_click = mouse_input.pressed(MouseButton::Left);
    let keyboard_press = keyboard_input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);
    
    if mouse_click && keyboard_press {
        info!("Just clicked LeftMouseButton and pressed Ctrl!");
    }
}
```

[`ButtonInput`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html
[has just been pressed]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_pressed
[is currently being pressed]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.pressed
[if its just been released]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_released

### ButtonInput Combinations

We're able to create button combinations by accessing multiple buttons within a given `ButtonInput<>`.
Using the [`any_pressed`], [`any_just_pressed`], or [`any_just_released`] methods allows us to establish AND logic for handling our combinations.
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

We can also access a collection of every button that is being interacted with at a given point in time.
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

### Resetting ButtonInput

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

### Pressed Versus Just Pressed

Although it might appear like the difference between [`pressed`] and [`just_pressed`] is negligible, the two are quite distinct.
While both signal a `ButtonInput` input being activated, `pressed` is continuously `true` until the input is released.
Meanwhile `just_pressed` will only be `true` for _a single frame_ after the input is activated.
The same is true for [`just_released`], which will only be `true` for a single frame after the input is deactivated.

To see the differences, think of using a weapon in any action game.
Unless the game is based solely on precision, there will likely be moments where players will want to repeatedly use their weapon without having to also repeatedly press a button.
In this case, we can use `pressed` to check if the button which activates the weapon is pressed, and if so then the weapon can be repeatedly used.

```rust
// A component indicating a weapon.
#[derive(Component)]
struct Weapon;

// A component indicating how much time needs to pass in-between weapon attacks.
#[derive(Component)]
struct WeaponAttackInterval(pub Timer);

fn repeated_weapon_use(
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
        player_weapon.1.from_seconds(1.5, TimerMode::Repeating);
    }
}
```

Finally, we can extend this mechanic even further by using `just_released` to change the `WeaponAttackInterval` timer mode to `Once` when the player finally releases the attack button.
This will still let the timer tick down and finish.

```rust
fn cancel_weapon_interval(
    button_input: Res<ButtonInput<MouseButton>>,
    mut player_weapon: Single<&mut WeaponAttackInterval, With<PlayerWeapon>>
) {
    if button_input.just_released(MouseButton::LeftMouseButton) {
        player_weapon.0.set_mode(TimerMode::Once);
    }
}
```

This example doesn't quite cover the full extent of a mechanic like this, however it does provide you with an idea of how `pressed`, `just_pressed`, and `just_released` can be effectively used.

[`pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.pressed
[`just_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_pressed
[`just_released`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_released

## Input System Conditions

## Window Input Events
