+++
title = "Utilizing Input"
insert_anchor_links = "right"
[extra]
weight = 6
+++

By definition, games are interactive.
Players will perform some action or reaction, and the game will react and change in return.
When it comes to video games specifically, we need to receive input from the player through some kind of device.
Whether it's controllers, keyboards, mice, or even touch screens, Bevy allows us to interact with input from all of these devices.

You can interact with user input in one of two ways: react to input events, or access an input resource.
Both will let you see what input has been given by the player, but each has their own use case.

```rust
// This system reads and prints out all messages sent by `KeyboardInput`,
// which will be any time a key on a keyboard is pressed.
fn keyboard_events_reader(mut keyboard_inputs: MessageReader<KeyboardInput>) {
    for keyboard_input in keyboard_inputs.read() {
        info!("{:?}", keyboard_input);
    }
}
// This system provides access to the `Touches` resource.
fn touch_input_system(
    touch_input: Res<Touches>,
) {
    for touch in touch_input.iter() {
        info!("Active Touch: {:?}", touch);
    }
}

```

Reacting to input events is great for handling multiple different types of input at the same time.
This works well for ordering inputs within a single frame, or for activating one-shot systems when an input is received.

```rust
// This system will read input events from `MouseButtonInput`.
// If the Left Mouse Button is pressed, a one-shot system will be queued to run.
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

Alternatively, most input actions have an associated resource that we can directly access via a system parameter.
Directly accessing the resource can provide us with more information about the action and give us more control over how we respond to it.
For example, accessing a [`ButtonInput`] resource for a specific input action (`KeyCode` in the example below) provides us with a number of methods that lets us see if the button [has just been pressed], [is currently being pressed], or [if its just been released].

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

Additionally, we aren't limited to only accessing one input action at a time.
We can access both a mouse action and a keyboard action and use both.
In the same way, we can also read multiple input actions from the same device.

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

// This system prints when `Ctrl + Shift + A` is pressed.
fn multiple_keyboard_input(input: Res<ButtonInput<KeyCode>>) {
    let shift = input.any_pressed([KeyCode::ShiftLeft, KeyCode::ShiftRight]);
    let ctrl = input.any_pressed([KeyCode::ControlLeft, KeyCode::ControlRight]);

    if ctrl && shift && input.just_pressed(KeyCode::KeyA) {
        info!("Just pressed Ctrl + Shift + A!");
    }
}
```

[`ButtonInput`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html
[has just been pressed]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_pressed
[is currently being pressed]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.pressed
[if its just been released]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_released

## Input Devices

Bevy can read input from the aforementioned devices: gamepads (controllers), keyboards, mice, and touch inputs.
Each takes a similar approach when it comes to receiving and processing input, but each has their own unique quirks associated with them.

All input data comes from messages that are sent every frame.
Each input `Message` is read by a unique system in the `PreUpdate` schedule and placed into a `Resource` (with the exception of gamepads, see the [gamepads section] below for more information).
Processing the messages in `PreUpdate` allows for the input to be accounted for before the rest of the game data in the `Update` and `FixedUpdate` schedules.

Every input device is enabled through a [feature flag].
By default, larger groupings of features like profiles and collections will have all inputs devices enabled.
If you know that your game will not need touch inputs in your game (or keyboard and mouse inputs if you're building a mobile game), you can disable these input devices by turning its feature off in your project `Cargo.toml` file.
See the [Selective Feature Use section] on the Compiling Less Code page for more details.

[gamepads section]: .#gamepads
[feature flag]: https://docs.rs/bevy/latest/bevy/index.html#feature-list
[Selective Feature Use section]: /learn/book/releasing-projects/compiling-less-code/#more-selective-feature-use

### Keyboards

Using input from a keyboard is relatively straightforward.
Each key is read as input through a [`KeyboardInput`] message, which is then placed into the [`ButtonInput`] resource during the `PreUpdate` schedule.
This resource allows you access different conditions, thresholds, and key combinations that you can use to trigger some functionality in response.

When the `keyboard` feature is enabled in your game, Bevy will automatically:

- Add the [`KeyboardInput`] and [`KeyboardFocusLost`] messages to the `World`.
  - `KeyboardInput` records the actual keyboard input data, while `KeyboardFocusLost` indicates if a keyboard is sending input to the application.
- Initialize the `ButtonInput<Key>` and `ButtonInput<KeyCode>` resources.
  - This gives us access to the specific [`KeyCode`] buttons and generic [`Key`] value to read from.
  - This resource is updated with the data that is captured in the `KeyboardInput` message.
- Insert the [`keyboard_input_system`] into the [`PreUpdate`] schedule.
  - This system updates both `ButtonInput` resources with the input received in the `KeyboardInput` message.

```rust
fn keyboard_messages(
    keyboard_messages: MessageReader<KeyboardInput>,
) {
    // Read the KeyboardInput message events.
    for key_press in keyboard_messages.read() {
        if key_press.key == Key::F11 {
            // Take a screenshot.
        }
    }
}
fn keyboard_connection(
    keyboard_connection: MessageReader<KeyboardFocusLost>,
) {
    // See if the keyboard is still providing input to the game.
    if keyboard_connection.len() > 0 {
        println!("No longer receiving keyboard input!");
    }
}
fn keyboard_access(
    keyboard_resource: Res<ButtonInput<KeyCode>>,
) {
    // Check if Alt + F4 is being pressed.
    if keyboard_resource.all_pressed([KeyCode::AltLeft, KeyCode::F4]) {
        println!("The Game Will Shutdown!");
    }
}
```

[`KeyboardInput`]: https://docs.rs/bevy/latest/bevy/input/keyboard/struct.KeyboardInput.html
[`KeyboardFocusLost`]: https://docs.rs/bevy/latest/bevy/input/keyboard/struct.KeyboardFocusLost.html
[`KeyCode`]: https://docs.rs/bevy/latest/bevy/input/keyboard/enum.KeyCode.html
[`Key`]: https://docs.rs/bevy/latest/bevy/input/keyboard/enum.Key.html
[`keyboard_input_system`]: https://docs.rs/bevy/latest/bevy/input/keyboard/fn.keyboard_input_system.html
[`PreUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html

### Mice

Unlike keyboards, mice are multi-faceted: they can provide both button input _and_ motion.
When we take in input from a mouse, we're also taking in more data.

When the `mouse` feature is enabled in your game, Bevy will automatically:

- Add [`MouseButtonInput`], [`MouseMotion`], and [`MouseWheel`] messages to the `World`.
  - These messages record the actual input data from mouse button clicks, mouse motion, and scroll wheel actions.
- Initialize [`ButtonInput<MouseButton>`], [`AccumulatedMouseMotion`], and [`AccumulatedMouseScroll`] and resources.
  - These resources are updated with the input data that is captured in their respective message.
- Insert the [`mouse_button_input_system`], [`accumulate_mouse_motion_system`], and [`accumulate_mouse_scroll_system`] to the [`PreUpdate`] schedule.
  - These systems update their respective resources with all input actions and data that occur.

```rust
fn mouse_inputs(
    button_input: Res<ButtonInput<MouseButton>>,
    movement_input: Res<AccumulatedMouseMotion>,
    scroll_input: Res<AccumulatedMouseScroll>,
) {
    // Read a mouse button press.
    if button_input.pressed(MouseButton::LeftMouseButton) {
        println!("Left Mouse Button has been pressed.");
    }
    // See if the mouse is moving.
    if movement_input.delta == Vec2{x: 0.0, y: 0.0} {
        println!("Mouse isn't moving!");
    } else {
        println!("The mouse is moving!");
    }
    // Read the scroll wheel value.
    if scroll_input.delta.y > 10.0 {
        println!("We're scrolling too fast!");
    }
}
```

[`MouseButtonInput`]: https://docs.rs/bevy/latest/bevy/input/mouse/struct.MouseButtonInput.html
[`MouseMotion`]: https://docs.rs/bevy/latest/bevy/input/mouse/struct.MouseMotion.html
[`MouseWheel`]: https://docs.rs/bevy/latest/bevy/input/mouse/struct.MouseWheel.html
[`ButtonInput<MouseButton>`]: https://docs.rs/bevy/latest/bevy/input/mouse/enum.MouseButton.html
[`AccumulatedMouseMotion`]: https://docs.rs/bevy/latest/bevy/input/mouse/struct.AccumulatedMouseMotion.html
[`AccumulatedMouseScroll`]: https://docs.rs/bevy/latest/bevy/input/mouse/struct.AccumulatedMouseScroll.html
[`mouse_button_input_system`]: https://docs.rs/bevy/latest/bevy/input/mouse/fn.mouse_button_input_system.html
[`accumulate_mouse_motion_system`]: https://docs.rs/bevy/latest/bevy/input/mouse/fn.accumulate_mouse_motion_system.html
[`accumulate_mouse_scroll_system`]: https://docs.rs/bevy/latest/bevy/input/mouse/fn.accumulate_mouse_scroll_system.html

### Gamepads

Gamepads are also complex input devices.
While we are still receiving button input, a gamepad can also supply axis input data from any thumbsticks it might have.
It might seem like we could treat the axis input data like we would motion on a mouse, but axis values aren't recording actual motion.
Instead we're receiving a 2D directional value, which results in the axis value input providing additional data and allowing for different configurations.

When the `gamepad` feature is enabled in your project, Bevy will automatically setup a number of [message events] along with two systems: [`gamepad_connection_system`] and [`gamepad_event_processing_system`], both placed in `PreUpdate`.

You'll notice that we aren't setting up any resources to directly access.
This is because each Bevy uses various `Settings` structs within a larger [`GamepadSettings`] to determine similar conditions and thresholds to those in `ButtonInput`.
Since multiple gamepads can be connected at the same time, placed these settings within a `Resource` wouldn't make sense.
Instead, each gamepad can have it's own `GamepadSettings`, allowing you to adjust each gamepad based on any conditions or situations you might want.

```rust
fn gamepad_system(gamepads: Query<(Entity, &Gamepad)>) {
    for (entity, gamepad) in &gamepads {
        if gamepad.just_pressed(GamepadButton::South) {
            info!("{} just pressed South", entity);
        } else if gamepad.just_released(GamepadButton::South) {
            info!("{} just released South", entity);
        }

        let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        if right_trigger.abs() > 0.01 {
            info!("{} RightTrigger2 value is {}", entity, right_trigger);
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("{} LeftStickX value is {}", entity, left_stick_x);
        }
    }
}
```

[message events]: https://docs.rs/bevy/latest/bevy/input/gamepad/index.html?search=gamepad%20event
[`gamepad_connection_system`]: https://docs.rs/bevy/latest/bevy/input/gamepad/fn.gamepad_connection_system.html
[`gamepad_event_processing_system`]: https://docs.rs/bevy/latest/bevy/input/gamepad/fn.gamepad_event_processing_system.html
[`GamepadSettings`]: https://docs.rs/bevy/latest/bevy/prelude/struct.GamepadSettings.html

### Touch Devices

Last, but certainly not least, is touch input.
While keyboards, mice, and gamepads all have unique data to process, touch input is relatively simple in Bevy.
A [`Touch`] action is used to store the position and force of a touch input.
You can see all of the data a `Touch` action stores by visiting the docs page.

When the `touch` feature is enabled in your project, Bevy will automatically:

- Add a [`TouchInput`] message to the `World`.
  - This message records the input action.
- Initialize the [`Touches`] resource.
  - This resource stores the value provided by `TouchInput`.
- Insert the [`touch_screen_input_system`] into the `PreUpdate` schedule.
  - This system updates the `Touches` resource with the values of every `TouchInput` message received.

```rust
fn touch_system(touches: Res<Touches>) {
    for touch in touches.iter_just_pressed() {
        info!(
            "just pressed touch with id: {}, at: {}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_released() {
        info!(
            "just released touch with id: {}, at: {}",
            touch.id(),
            touch.position()
        );
    }

    for touch in touches.iter_just_canceled() {
        info!("canceled touch with id: {}", touch.id());
    }

    // You can also iterate all current touches and retrieve their state like this:
    for touch in touches.iter() {
        info!("active touch: {touch:?}");
        info!("  just_pressed: {}", touches.just_pressed(touch.id()));
    }
}
```

[`Touch`]: https://docs.rs/bevy/latest/bevy/input/touch/struct.Touch.html
[`TouchInput`]: https://docs.rs/bevy/latest/bevy/prelude/struct.TouchInput.html
[`Touches`]: https://docs.rs/bevy/latest/bevy/input/touch/struct.Touches.html
[`touch_screen_input_system`]: https://docs.rs/bevy/latest/bevy/input/touch/fn.touch_screen_input_system.html

## Window Input Events

## Bevy Picking

## Expanding Input Handling
