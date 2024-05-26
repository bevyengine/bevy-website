- 
`ReceivedCharacter` is now deprecated, use `KeyboardInput` instead.

- 
Before:

```rust
fn listen_characters(events: EventReader<ReceivedCharacter>) {
  for event in events.read() {
    info!("{}", event.char);
  }
}
```

After:

```rust
fn listen_characters(events: EventReader<KeyboardInput>) {
  for event in events.read() {
    // Only check for characters when the key is pressed.
    if event.state == ButtonState::Released {
      continue;
    }
    // Note that some keys such as `Space` and `Tab` won't be detected as before.
    // Instead, check for them with `Key::Space` and `Key::Tab`.
    if let Key::Character(character) = &event.logical_key {
      info!("{}", character);
    }
  }
}
```
