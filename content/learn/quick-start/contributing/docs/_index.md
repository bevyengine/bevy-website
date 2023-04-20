+++
title = "Docs"
weight = 2
sort_by = "weight"
template = "docs-section.html"
page_template = "docs-section.html"
insert_anchor_links = "right"
aliases = ["learn/book/contributing/docs"]
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

### Bevy Book Contribution Guidelines

As outlined in a [discussion on GitHub](https://github.com/bevyengine/bevy-website/issues/623), contributions to the book should adhere to following guidelines:

- The version-specific Bevy Book is locked to versions of Bevy. No retroactive changes can be made to the Bevy Book for previous versions.
  - Meaning that once Bevy 0.11.0 and Bevy Book 0.11.0 has been released to the public, it can no longer change. Next version that will contain any updates is either 0.12.0 or 0.11.1.
- Shortcodes used in the version-specific Bevy Book should not change after a version has been finalized, except for adding paramaters while retaining the same behaviour for existing usages
  - Example: A `youtube` shortcode defaults to not being full-screen and is being used in version `0.11.0`. If this shortcode for `0.12.0` has to change the default behaviour, it should now start to accept a new parameter to set it to be fullscreen or not, which defaults to false, in order to have it work the same way in `0.11.0` forever.
- Assets used for a specific version lives in a version-specific directory in `static/assets`,
  - Example: `static/assets/v0.11.0` for assets created/edited for `0.11.0` of Bevy.
- Once version been finalized, the versioned asset directory can no longer change.
  - Assets that don't change between versions, should remain from the old version's asset directory.
- In order to create a new version of the book, copy previous version into its own directory
  - Previous version was v0.11.0 and we want to create version v0.12.0
    - Copy `/content/learn/book/v0.11.0` to `/content/learn/book/v0.12.0`
    - Change all references from `/content/learn/book/v0.11.0/*` to `/content/learn/book/v0.12.0/*`
    - Create empty asset directory for images in `/static/assets/v0.12.0`
  - Any new assets added should be added to this asset directory, otherwise reference assets in `/static/assets/v0.11.0`
    - If a asset has to be modified in any way, copy it from `/static/assets/v0.11.0` to `/static/assets/v0.12.0` and then modify it, changing the reference only in the current version and not in previous versions
  - Once the version is ready to be available to the public and published visible on the website:
    - Prepend the version to `/content/learn/book/versions.toml`
    - Change reference in `/content/learn/links.toml` to link to current new version

### Rust API Doc Syntax

We made an extension to the markdown syntax that makes linking to Rust API docs nicer. It also gives the links special formatting. Here are some examples:

- Full Type Path: {{rust_type(type="struct" crate="std", mod="collections", name="HashMap")}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap")}{{curly_close()}}```
- Short Type: {{rust_type(type="struct", crate="std" mod="collections", name="HashMap", no_mod=true)}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true)}{{curly_close()}}```
- Plural Type: {{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true plural=true)}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true, plural=true)}{{curly_close()}}```
- Function: {{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true method="insert")}}

    ```{{curly_open()}}{rust_type(type="struct" crate="std" mod="collections" name="HashMap" no_mod=true method="insert")}{{curly_close()}}```
- Module: {{rust_mod(crate="std" mod="collections")}}

    ```{{curly_open()}}{rust_mod(crate="std" mod="collections")}{{curly_close()}}```

Modules from {{rust_mod(crate="std")}} will link to [https://doc.rust-lang.org](https://doc.rust-lang.org/std/index.html). Other modules (like {{rust_mod(crate="bevy_render" mod="render_graph")}} ) will link to [https://docs.rs](https://docs.rs).

## Rust API Docs

Bevy's Rust API Docs are automatically generated from the latest Bevy source code. If you add [Rust documentation comments](https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments) to the Bevy codebase, the API docs will be automatically updated.

## Bevy Markdown Docs

Bevy's CI will check markdown files like Readmes using [markdownlint](https://github.com/DavidAnson/markdownlint). If you contribute to markdown files consider installing [markdownlint-cli](https://github.com/igorshubovych/markdownlint-cli) to locally lint your changes. Running `markdownlint -f -c .github/linters/.markdown-lint.yml .` in the root directory of the Bevy project will apply the same linting rules to your changes as the CI workflow.
