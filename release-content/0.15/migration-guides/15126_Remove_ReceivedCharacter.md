`ReceivedCharacter` was deprecated in 0.14 due to `winit` reworking their keyboard system. It has now been fully removed. Switch to using `KeyboardInput` instead.

```rust
// 0.14
fn listen_characters(events: EventReader<ReceivedCharacter>) {
    for event in events.read() {
        info!("{}", event.char);
    }
}

// 0.15
fn listen_characters(events: EventReader<KeyboardInput>) {
    for event in events.read() {
        // Only check for characters when the key is pressed.
        if !event.state.is_pressed() {
            continue;
        }

        // Note that some keys such as `Space` and `Tab` won't be detected as a character.
        // Instead, check for them as separate enum variants.
        match &event.logical_key {
            Key::Character(character) => {
                info!("{} pressed.", character);
            },
            Key::Space => {
                info!("Space pressed.");
            },
            _ => {},
        }
    }
}
```
