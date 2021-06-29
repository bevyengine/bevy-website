+++
title = "Generic systems"
weight = 7
template = "book-section.html"
page_template = "book-section.html"
+++

You are not limited to just *one* of a single system: you can insert more copies in different places through your schedule when you want to perform the work repeatedly.
This fact becomes particularly useful when we combine it with [generic types](https://doc.rust-lang.org/book/ch10-01-syntax.html): creating **generic systems** whose behavior is specialized on individual types.

Generic systems are useful for repeating the same logic across many related types, and are incredibly value for library authors who wish to provide configurable APIs that mesh nicely with their users code.
In the latter case, note that entire *plugins* can be made generic in the same way.

All of the standard tricks for Rust's generics work when used in systems, allowing you to create systems with [trait bounds](https://doc.rust-lang.org/book/ch10-02-traits.html#trait-bound-syntax), multiple generic type parameters and even [const generics](https://blog.rust-lang.org/2021/02/26/const-generics-mvp-beta.html) as type arguments.

Let's take a look at a simple example of how this might be useful: repeating logic across many different types of buttons.

TODO: write example code
```rust

```
