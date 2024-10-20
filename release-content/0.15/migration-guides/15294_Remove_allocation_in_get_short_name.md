__For `format!`, `dbg!`, `panic!`, etc.__

```rust
// Before
panic!("{} is too short!", get_short_name(name));

// After
panic!("{} is too short!", ShortName(name));
```

__Need a `String` Value__

```rust
// Before
let short: String = get_short_name(name);

// After
let short: String = ShortName(name).to_string();
```
