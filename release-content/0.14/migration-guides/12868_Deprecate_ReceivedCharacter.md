`ReceivedCharacter` is now deprecated, use `KeyboardInput` instead.

```rust
// 0.13
fn listen_characters(events: EventReader<ReceivedCharacter>) {
  for event in events.read() {
    info!("{}", event.char);
  }
}

// 0.14
fn listen_characters(events: EventReader<KeyboardInput>) {
  for event in events.read() {
    // Only check for characters when the key is pressed.
    if !event.state.is_pressed() {
      continue;
    }
    // Note that some keys such as `Space` and `Tab` won't be detected as before.
    // Instead, check for them with `Key::Space` and `Key::Tab`.
    match &event.logical_key {
        Key::Character(character) => {
            info!("{} pressed.", character);
        }
        Key::Space => {
            info!("Space pressed.");
        }
        _ => {}
    }
  }
}
```
