Component hook registration is now split out into individual methods of `Component`. The original `Component::register_component_hooks()` has been deprecated, so please switch to the new `Component::on_add()`, `Component::on_remove()`, and related methods.

```rust
// 0.15
impl Component for Foo {
    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks.on_add(foo_on_add);
    }

    // ...
}

// 0.16
impl Component for Foo {
    fn on_add() -> Option<ComponentHook> {
        Some(foo_on_add)
    }

    // ...
}
```
