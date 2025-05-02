The `close_on_esc` system was removed because it was too opiniated and lacked customization. If you used this system, you may copy its contents below:

```rust
pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}
```

You may be interested in using the built-in keybinds provided by the operating system instead, such as <kbd>Alt+F4</kbd> and <kbd>Command+Q</kbd>.
