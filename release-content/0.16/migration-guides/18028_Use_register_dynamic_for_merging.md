`RequiredComponents::register_dynamic()` has been replaced by `RequiredComponents::register_dynamic_with()`, which avoids unnecessary cloning.

```rust
// 0.15
required_components.register_dynamic(
    component_id,
    component_constructor.clone(),
    requirement_inheritance_depth,
);

// 0.16
required_components.register_dynamic_with(
    component_id,
    requirement_inheritance_depth,
    || component_constructor.clone(),
);
```
