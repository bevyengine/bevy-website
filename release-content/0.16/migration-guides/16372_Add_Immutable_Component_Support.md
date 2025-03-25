- When implementing `Component` manually, you must now provide a type for `Mutability`. The type `Mutable` provides equivalent behaviour to earlier versions of `Component`:

```rust
impl Component for Foo {
    type Mutability = Mutable;
    // ...
}
```

- When working with generic components, you may need to specify that your generic parameter implements `Component<Mutability = Mutable>` rather than `Component` if you require mutable access to said component.
- The entity entry API has had to have some changes made to minimise friction when working with immutable components. Methods which previously returned a `Mut<T>` will now typically return an `OccupiedEntry<T>` instead, requiring you to add an `into_mut()` to get the `Mut<T>` item again.
