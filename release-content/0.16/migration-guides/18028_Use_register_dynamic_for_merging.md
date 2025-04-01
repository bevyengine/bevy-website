`RequiredComponents::register_dynamic` has been changed to `RequiredComponents::register_dynamic_with`.

Old:

```rust
required_components.register_dynamic(
      component_id,
      component_constructor.clone(),
      requirement_inheritance_depth,
);
```

New:

```rust
required_components.register_dynamic_with(
      component_id,
      requirement_inheritance_depth,
      || component_constructor.clone(),
);
```

This can prevent unnecessary cloning.
