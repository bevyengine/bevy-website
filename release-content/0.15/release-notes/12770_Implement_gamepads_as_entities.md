<!-- Implement gamepads as entities -->
<!-- https://github.com/bevyengine/bevy/pull/12770 -->

Gamepads are now represented as entities, which makes them easier to work with!
The [`Gamepad`] component provides button and axis state, as well as metadata such as the vendor and product ID.
The [`GamepadSettings`] component provides configurable settings for a given [`Gamepad`], such as deadzones and sensitivity. The name of the gamepad is now stored in Bevy's standard [`Name`] component.

In Bevy 0.14, you might write:

```rust
fn gamepad_system(
   gamepads: Res<Gamepads>,
   button_inputs: Res<ButtonInput<GamepadButton>>,
   button_axes: Res<Axis<GamepadButton>>,
   axes: Res<Axis<GamepadAxis>>,
) {
    for gamepad in &gamepads {
        if button_inputs.just_pressed(
            GamepadButton::new(gamepad, GamepadButtonType::South)
        ) {
            info!("just pressed South");
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
fn gamepad_system(gamepads: Query<&Gamepad>) {
    for gamepad in &gamepads {
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

Much better!

[`Gamepad`]: https://docs.rs/bevy/0.15/bevy/input/gamepad/struct.Gamepad.html
[`GamepadSettings`]: https://docs.rs/bevy/0.15/bevy/input/gamepad/struct.GamepadSettings.html
[`Name`]: https://docs.rs/bevy/0.15/bevy/core/struct.Name.html
