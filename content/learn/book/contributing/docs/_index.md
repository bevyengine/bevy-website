+++
title = "Docs"
weight = 2
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
insert_anchor_links = "right"
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

We made an extension to the markdown syntax that makes linking to Rust API docs nicer. It also gives the links special formatting. Here are some examples:

* Full Type Path: {{rust_type(type="struct" crate="std", mod="collections", name="HashMap")}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap")}{{curly_close()}}```
* Short Type: {{rust_type(type="struct", crate="std" mod="collections", name="HashMap", no_mod=true)}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true)}{{curly_close()}}```
* Plural Type: {{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true plural=true)}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true, plural=true)}{{curly_close()}}```
* Function: {{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true method="insert")}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true method="insert")}{{curly_close()}}```
* Module: {{rust_mod(crate="std" mod="collections")}}

    ```{{curly_open()}}{rust_mod(crate="std" mod="collections")}{{curly_close()}}```

Modules from {{rust_mod(crate="std")}} will link to [https://doc.rust-lang.org](https://doc.rust-lang.org/std/index.html). Other modules (like {{rust_mod(crate="bevy_render" mod="render_graph")}} ) will link to [https://docs.rs](https://docs.rs).

## Rust API Docs

Bevy's Rust API Docs are automatically generated from the latest Bevy source code. If you add [Rust documentation comments](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments) to the Bevy codebase, the API docs will be automatically updated.

## Bevy Markdown Docs

Bevy's CI will check markdown files like Readmes using [markdownlint](https://github.com/DavidAnson/markdownlint). If you contribute to markdown files consider installing [markdownlint-cli](https://github.com/igorshubovych/markdownlint-cli) to locally lint your changes. Running `markdownlint -f -c .github/linters/.markdown-lint.yml .` in the root directory of the Bevy project will apply the same linting rules to your changes as the CI workflow.
