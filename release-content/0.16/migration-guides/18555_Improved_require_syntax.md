Custom-constructor requires should use the new expression-style syntax:

```rust
// before
#[derive(Component)]
#[require(A(returns_a))]
struct Foo;

// after
#[derive(Component)]
#[require(A = returns_a())]
struct Foo;
```

Inline-closure-constructor requires should use the inline value syntax where possible:

```rust
// before
#[derive(Component)]
#[require(A(|| A(10))]
struct Foo;

// after
#[derive(Component)]
#[require(A(10)]
struct Foo;
```

In cases where that is not possible, use the expression-style syntax:

```rust
// before
#[derive(Component)]
#[require(A(|| A(10))]
struct Foo;

// after
#[derive(Component)]
#[require(A = A(10)]
struct Foo;
```
