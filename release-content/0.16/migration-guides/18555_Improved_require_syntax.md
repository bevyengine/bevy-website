Required component syntax has been reworked to be more intuitive with Rust's syntax. Custom-constructor requires should use the new expression-style syntax:

```rust
// 0.15
#[derive(Component)]
#[require(A(returns_a))]
struct Foo;

// 0.16
#[derive(Component)]
#[require(A = returns_a())]
struct Foo;
```

Inline-closure-constructor requires should use the inline value syntax where possible:

```rust
// 0.15
#[derive(Component)]
#[require(A(|| A(10)))]
struct Foo;

// 0.16
#[derive(Component)]
#[require(A(10))]
struct Foo;
```

In cases where that is not possible, use the expression-style syntax:

```rust
// 0.15
#[derive(Component)]
#[require(A(|| A(10)))]
struct Foo;

// 0.16
#[derive(Component)]
#[require(A = A(10))]
struct Foo;
```
