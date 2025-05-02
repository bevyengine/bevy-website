**Bevy 0.15** introduced [Required Components](/news/bevy-0-15/#required-components), which enabled components to "require" other components when they are inserted. A component's required components are automatically inserted alongside them (with some configured default value).

**Bevy 0.15** supported the following syntax for defining Required Components:

```rust
#[derive(Component)]
#[require(
    A, // Default: this will use A::default()
    B(some_b), // Function: this will use some_b to create B
    C(|| C { value: 1 }), // Inline Closure: will use the given closure
)]
struct Foo;

fn some_b() -> B {
    B(1)
}
```

This worked pretty well, but it had some downsides:

1. Defining custom values involved more boilerplate than necessary. You either had to define a whole standalone function to return the value, or use clunky closure syntax (where you repeat the component name).
2. Defining custom values wasn't aligned with Rust syntax or common derive macro conventions.

In **Bevy 0.16** we reworked and expanded Required Component syntax to support inline values and a more natural and idiomatic "expression" syntax.

```rust
#[derive(Component)]
#[require(
    A, // this will use A::default()
    B(1), // inline tuple-struct value
    C { value: 1 }, // inline named-struct value
    D::Variant, // inline enum variant
    E::SOME_CONST, // inline associated const
    F::new(1), // inline constructor
    G = returns_g(), // an expression that returns G
    H = SomethingElse::new(), // expression returns SomethingElse, where SomethingElse: Into<H> 
)]
struct Foo;
```

Much better!
