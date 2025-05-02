Gamepad input is no longer accessed using resources, instead they are entities and are accessible using the Gamepad component as long as the gamepad is connected.

Gamepads resource has been deleted, instead of using an internal id to identify gamepads you can use its Entity. Disconnected gamepads will **NOT** be despawned. Gamepad components that don’t need to preserve their state will be removed i.e. Gamepad component is removed, but GamepadSettings is kept.
Reconnected gamepads will try to preserve their Entity id and necessary components will be re-inserted.

GamepadSettings is no longer a resource, instead it is a component attached to the Gamepad entity.

Axis<GamepadButton>, Axis<GamepadAxis> and ButtonInput<GamepadButton> methods are accessible via Gamepad component.

```diff
fn gamepad_system(
-   gamepads: Res<Gamepads>,
-   button_inputs: Res<ButtonInput<GamepadButton>>,
-   button_axes: Res<Axis<GamepadButton>>,
-   axes: Res<Axis<GamepadAxis>>,
+   gamepads: Query<&Gamepad>
) {
    for gamepad in gamepads.iter() {
-      if button_inputs.just_pressed(GamepadButton::new(gamepad, GamepadButtonType::South)) {
+      if gamepad.just_pressed(GamepadButton::South) {
            println!("just pressed South");
        } 
         
-      let right_trigger = button_axes
-           .get(GamepadButton::new(
-               gamepad,
-               GamepadButtonType::RightTrigger2,
-           ))
-           .unwrap();
+      let right_trigger = gamepad.get(GamepadButton::RightTrigger2).unwrap();
        if right_trigger.abs() > 0.01 {
            info!("RightTrigger2 value is {}", right_trigger);      
        }

-        let left_stick_x = axes
-           .get(GamepadAxis::new(gamepad, GamepadAxisType::LeftStickX))
-           .unwrap();
+       let left_stick_x = gamepad.get(GamepadAxis::LeftStickX).unwrap();
        if left_stick_x.abs() > 0.01 {
            info!("LeftStickX value is {}", left_stick_x);        
        }
    }
}
```
