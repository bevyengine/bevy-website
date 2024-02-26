# Code Validation

This tool checks that all code examples on the website compile and execute correctly through the use of Rust's [documentation tests].

[documentation tests]: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html

To run code validation, you can simply run `cargo test`:

```shell
cargo test -p code-validation
```

## Validating New Pages

Anytime a new page is added to the website with code examples, it should be added here as well. Each page is represented by a module (`mod`) whose documentation comments are the page itself.

```rust
mod learn {
    // Include the entire contents of the getting started section as this module's documentation.
    #[doc = include_str!("../../content/learn/quick-start/_index.md")]
    mod quick_start {
        // This is a page.
        #[doc = include_str!("../../content/learn/quick-start/introduction.md")]
        mod introduction {}

        // This is a section with subpages.
        #[doc = include_str!("../../content/learn/quick-start/getting-started/_index.md")]
        mod getting_started {
            // ...
        }
    }

    // ...
}
```

When `cargo test` is run, Cargo automatically loads the specified Markdown files and runs all Rust code blocks. Be warned Cargo will assume that a code block without a language attribute is Rust by default.

`````markdown
This is run:

```rust
assert!(true);
```

So is this:

```
assert_eq!(3, 1 + 2);
```

This is not:

```text
Unoxidized text...
```
`````

## Ignoring Examples

If there is a code example that should not be validated, you can add the `ignore` attribute.

`````markdown
```ignore
const COMPILE_ERROR: u32 = false;
```
`````

If there is a code example that should not be run, but might benefit from being compiled, you can add the `no_run` attribute.

`````markdown
```no_run
loop {
    // An infinite loop! Oh no...
}
```
`````

For a full list of attributes, see the [rustdoc book].

[rustdoc book]: https://doc.rust-lang.org/rustdoc/write-documentation/documentation-tests.html#attributes
