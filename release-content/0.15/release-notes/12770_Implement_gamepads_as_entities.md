<!-- Implement gamepads as entities -->
<!-- https://github.com/bevyengine/bevy/pull/12770 -->

Gamepads are surprisingly complex objects: each with their own array of settings and unique identifier.
To make it easier to work with gamepads, each gamepad is now a distinct entity.
Metadata and the state of each button and axis is stored on the [`Gamepad`] component, and gamepad-specific settings like deadzones and sensitivity can be configured with the [`GamepadSettings`] component.

In Bevy 0.14, you might write:

```rust
fn gamepad_system(
   gamepads: Res<Gamepads>,
   button_inputs: Res<ButtonInput<GamepadButton>>,
   button_axes: Res<Axis<GamepadButton>>,
   axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in gamepads.iter() {
        if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
            println!("just pressed South");
        } 
         
        let right_trigger = button_axes
           .get(GamepadButton::new(
               gamepad,
               GamepadButtonType::RightTrigger2,
           ))
           .unwrap();
        if right_trigger.abs() > 0.01 {
            info!("RightTrigger2 value is {}", right_trigger);      
        }

        let left_stick_x = axes
           .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
           .unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("LeftStickX value is {}", left_stick_x);        
        }
    }
}
```

In 0.15, we can write this much more simply as:

```rust
fn gamepad_system(
   gamepads: Query<&Gamepad>
) {
    for gamepad in gamepads.iter() {
      if gamepad.just_pressed(GamepadButton::South) {
            println!("just pressed South");
        } 
         
        let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        if right_trigger.abs() > 0.01 {
            info!("RightTrigger2 value is {}", right_trigger);      
        }

        let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("LeftStickX value is {}", left_stick_x);        
        }
    }
}
```

This design makes adding custom behavior to each gamepad easy, and  opens the door to supporting more advanced features such as rumble and gyroscopic control directly in Bevy in the future!

[`Gamepad`]: https://docs.rs/bevy/0.15.0-rc.2/bevy/input/gamepad/struct.Gamepad.html
[`GamepadSettings`]: https://docs.rs/bevy/0.15.0-rc.2/bevy/input/gamepad/struct.GamepadSettings.html
