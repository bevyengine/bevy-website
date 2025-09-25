<!-- Support systems that take references as input -->
<!-- https://github.com/bevyengine/bevy/pull/15184 -->

System piping is a powerful (if relatively niche) tool to pass data directly from one system to another.
While this is useful for [error handling],
it's a general purpose tool for composing fragments of logic by gluing together matching inputs and outputs.

This machinery has since been repurposed for use with [one-shot systems],
allowing you to call [`World::run_system_with_input`] to evaluate systems with whatever input you supply,
and get the return value back out.
Great for writing tests!

However, this set of tools has always had a frustrating and confusing limitation: any data passed into a system must have a static lifetime.
This seems absurd; the data is passed directly from one owner to the next and the systems are run as if they were a single unit.

With the liberal application of some type magic pixie dust, this limitation has been lifted!

```rust
let mut world = World::new();

let mut value = 2;

// This always worked:
fn square(In(input): In<usize>) -> usize {
    input * input
}
value = world.run_system_with_input(value, square);

// Now possible:
fn square_ref(InRef(input): InRef<usize>) -> usize {
    *input * *input
}
value = world.run_system_with_input(&value, square_ref);

// Mutably:
fn square_mut(InMut(input): InMut<usize>) {
    *input *= *input;
}
world.run_system_with_input(&mut value, square_mut);
```

We're excited to see what you do with this newfound power.

[error handling]: https://github.com/bevyengine/bevy/blob/main/examples/ecs/system_piping.rs
[one-shot systems]: https://bevy.org/news/bevy-0-12/#one-shot-systems
[`World::run_system_with_input`]: https://docs.rs/bevy/0.15.0/bevy/ecs/prelude/struct.World.html#method.run_system_with_input
