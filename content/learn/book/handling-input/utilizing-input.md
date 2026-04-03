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
When the button on a gamepad or a key on a keyboard gets pressed, Bevy is able to convert the value using [Winit] and [`bevy_winit`] (Bevy's translation tool for Winit).

In order to start using input in your game, all you need to know is that Bevy takes the input data and converts it to a [`Message`] that can be [read like any other message you'd use normally]. 
These messages are sent every frame, and are processed in the [`PreUpdate`] schedule.
Each input device type will read from a unique message type: [`KeyboardInput`] for keyboards, [`MouseButtonInput`] for mouse button presses, and so on for each input device type.

All input in Bevy is handled in a similar way, however each device type has their own unique circumstances to be aware of.
To see how each input device type can be used in Bevy, see their respective pages:

- [Keyboard Input](/learn/book/handling-input/keyboard-input)
- [Mouse Input](/learn/book/handling-input/mouse-input)
- [Gamepad Input](/learn/book/handling-input/gamepad-input)
- [Touch Input](/learn/book/handling-input/touch-input)

Every input device is enabled through a [feature flag].
By default, larger groupings of features like profiles and collections will have all input devices enabled.
If you know that your game will not need touch input (or keyboard and mouse inputs if you're building a mobile game), you can disable these input devices by turning their feature flag off in your project `Cargo.toml` file.
See the [Selective Feature Use section] on the Compiling Less Code page for more details.

[read like any other message you'd use normally]: /learn/book/control-flow/messages
[Selective Feature Use section]: /learn/book/releasing-projects/compiling-less-code/#more-selective-feature-use

[Winit]: https://crates.io/crates/winit
[`bevy_winit`]: https://docs.rs/bevy/latest/bevy/winit/index.html
[`Message`]: https://docs.rs/bevy/latest/bevy/ecs/message/trait.Message.html
[`PreUpdate`]: https://docs.rs/bevy/latest/bevy/app/struct.PreUpdate.html
[`KeyboardInput`]: https://docs.rs/bevy/latest/bevy/input/keyboard/struct.KeyboardInput.html
[`MouseButtonInput`]: https://docs.rs/bevy/latest/bevy/input/mouse/struct.MouseButtonInput.html
[feature flag]: https://docs.rs/bevy/latest/bevy/index.html#feature-list

## Input Messages

The first way we can interact with input is by reading the [`Messages`] that are sent from each input source.

```rust
// This system reads and prints out all `KeyboardInput` messages.
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
// A system that will run when Left Mouse Button is clicked.
fn mouse_click_system() {
    println!("The Left Mouse Button was clicked!");
}
// Register a system that will be queued to run.
fn register_system(mut commands: Commands) {
    let registered_system = commands.register_system(mouse_click_system);
}
```


[`Messages`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Messages.html

## ButtonInput Resources

Alternatively, once most input messages are processed they'll be stored in an associated [`ButtonInput`] resource that we can directly access via a system parameter.
Accessing input data this way can provide us with more information about the input and give us more control over how we respond to it.

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

{% callout(type="info") %}

### _Pressed_ Versus _Just Pressed_

Although it might appear like the difference between [`pressed`] and [`just_pressed`] is negligible, the two are quite distinct.
While both signal a `ButtonInput` input being activated, `pressed` is continuously `true` until the input is released.
Meanwhile `just_pressed` will only be `true` for _a single frame_ after the input is activated.

The same is true for [`just_released`], which will only be `true` for a single frame after the input is deactivated, and for any other `ButtonInput` method that contains `just` in it's name.

[`pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.pressed
[`just_pressed`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_pressed
[`just_released`]: https://docs.rs/bevy/latest/bevy/input/struct.ButtonInput.html#method.just_released
{% end %}

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

## Window Input Events
