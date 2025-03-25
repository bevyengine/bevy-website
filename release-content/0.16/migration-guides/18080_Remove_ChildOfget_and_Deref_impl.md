```rust
// Before
**child_of
// After
child_of.parent

// Before
child_of.get()
// After
child_of.parent

// Before
entity.get::<ChildOf>().map(ChildOf::get)
// After
entity.get::<ChildOf>().map(|c| c.parent)
```
