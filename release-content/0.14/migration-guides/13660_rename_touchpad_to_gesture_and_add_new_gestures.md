In a recent `winit` update, touchpad events can now be triggered on mobile. To account for this, touchpad-related items have been renamed to gestures:

- `bevy::input::touchpad` has been renamed to `bevy::input::gestures`.
- `TouchpadMagnify` has been renamed to `PinchGesture`.
- `TouchpadRotate` has been renamed to `RotationGesture`.
