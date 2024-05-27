Working with `Commands` when you have exclusive world access has always been a pain.
Create a `CommandQueue`, generate a `Commands` out of that, send your commands and then apply it?
Not exactly the most intuitive solution.

Now, you can access the `World`'s own command queue:

```rust
let mut world = World::new();
let mut commands = world.commands();
commands.spawn(TestComponent);
world.flush_commands();
```

While this isn't the most performant approach (just apply the mutations directly to the world and skip the indirection),
this API can be great for quickly prototyping with or easily testing your custom commands (and is used internally to power lifecycle hooks).

As a bonus, one-shot systems now apply their commands (and other deferred system params) immediately when run!
We already have exclusive world access: why introduce delays and subtle bugs?
