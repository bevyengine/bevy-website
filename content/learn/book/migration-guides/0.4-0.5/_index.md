+++
title = "0.4 to 0.5"
weight = 1
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Migration Guide: 0.4 to 0.5"
+++

<!-- TODO: link to release blog post here -->

## `commands: &mut Commands` SystemParam is now `mut commands: Commands`

```rust
// 0.4
fn foo(commands: &mut Commands) {
}

// 0.5
fn foo(mut commands: Commands) {
}
```

Systems using the old `commands: &mut Commands` syntax in 0.5 will fail to compile when calling `foo.system()`.

This change was made because `Commands` now holds an internal `World` reference to enable safe Entity allocations.
