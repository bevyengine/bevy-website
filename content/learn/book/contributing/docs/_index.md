+++
title = "Docs"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
[extra]
long_title = "Contributing Docs"
+++

## The Bevy Book

The Bevy Book is open source, along with the rest of this website. Check out the [Bevy Website repository on GitHub](https://github.com/bevyengine/bevy-website). The Bevy Book content is written in Markdown.

### Building the Website

The website is built using the [Zola static site generator](https://www.getzola.org/). Download Zola, then do the following:

1. Clone the Bevy Website git repo and move to that directory:
    ```sh
    git clone https://github.com/bevyengine/bevy-website.git
    cd bevy-website
    ```
2. Start the Zola server
    ```sh
    zola serve
    ```

A local server should start and you should be able to access a local version of the website from there.

### Rust API Doc Syntax

We made an extension to the markdown syntax that makes linking to Rust API docs easier and prettier.
Here are some example links and the associated shortcodes:

- {{rust_type(type="struct" crate="std" mod="collections" name="HashMap")}}: ???. This is the standard invocation.
- {{rust_type(type="struct" crate="bevy" mod="ecs::system" name="Commands" method = "spawn")}}: ???. We can specify which method we're referring to using the `method` argument.
- {{rust_type(type="trait" crate="bevy_ecs" mod="system" name="Command" method = "write")}}: ???. Change the `type` argument to link to enums, traits or keywords.
- {{rust_type(type="struct" crate="std" mod="result" name="Result" show_mod=true)}}: ???. By using `show_mod`, we can see the full path.
- {{rust_type(crate="std" mod="collections")}}: ???. By omitting `type` and `name`, we can link to the crate or module itself.

There are several options available, toggled by adding the following arguments (separating each argument with a space) within the curly braces:

- `show_crate`: shows the originating crate in the path
- `show_mod`: shows the originating module in the path
- `plural`: adds an "s" at the end of the linked type

Modules from {{rust_type(crate="std")}} will link to [doc.rust-lang.org](https://doc.rust-lang.org/std/index.html). Other modules (like {{rust_type(crate="bevy_render" mod="render_graph")}} ) will link to [docs.rs](https://docs.rs).

## Rust API Docs

Bevy's Rust API Docs are automatically generated from the latest Bevy source code. If you add [Rust documentation comments](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments) to the Bevy codebase, the API docs will be automatically updated.

## Bevy Markdown Docs

Bevy's CI will check markdown files like Readmes using [markdownlint](https://github.com/DavidAnson/markdownlint). If you contribute to markdown files consider installing [markdownlint-cli](https://github.com/igorshubovych/markdownlint-cli) to locally lint your changes. Running `markdownlint -f -c .github/linters/.markdown-lint.yml .` in the root directory of the Bevy project will apply the same linting rules to your changes as the CI workflow.
