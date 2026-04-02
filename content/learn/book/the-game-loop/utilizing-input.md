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
```

Alternatively, each input type has an associated resource that we can directly access via a system parameter.
Directly accessing the resource can provide us with more information about the input and give us more control over how we respond to it.
For example, reading the [`ButtonInput`] resource provides us with a number of methods that lets us see if the button [has just been pressed], [is currently being pressed], or [if its just been released].

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

Additionally, we aren't limited to only accessing one input at a time.
We can access both a mouse input resource and a keyboard input resource and use input from both.
In the same way, we can also read multiple inputs from the same device. 

```rust
// This system prints when `Ctrl + LeftMouseButton` is pressed.
fn keyboard_and_mouse_input(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>
) {
    let mouse_click = mouse_input.pressed(MouseButtom::Left);
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

However, if we're accessing an input resource, then we have to remember the rules of the borrow checker: access can be mutable or multiple, but never both at the same time.

[`ButtonInput`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html
[has just been pressed]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_pressed
[is currently being pressed]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.pressed
[if its just been released]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_released

## Input Devices

Bevy can read input from the aforementioned devices: gamepads (controllers), keyboards, mice, and touch inputs.
Each uses a similar approach when it comes to reading input, but each has their own unique quirks associated with them.

### Keyboards

### Mice

### Gamepads

### Touch Devices

## Window Input Events

## Bevy Picking

## Expanding Input Handling
