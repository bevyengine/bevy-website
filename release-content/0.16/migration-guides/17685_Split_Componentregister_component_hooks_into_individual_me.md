`Component::register_component_hooks` is now deprecated and will be removed in a future release. When implementing `Component` manually, also implement the respective hook methods on `Component`.

```rust
// Before
impl Component for Foo {
    // snip
    fn register_component_hooks(hooks: &mut ComponentHooks) {
            hooks.on_add(foo_on_add);
    }
}

// After
impl Component for Foo {
    // snip
    fn on_add() -> Option<ComponentHook> {
            Some(foo_on_add)
    }
}
```
