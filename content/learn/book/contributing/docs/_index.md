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

The Bevy Book (and the rest of this website) is open source. Check out the <a href="https://github.com/bevyengine/bevy-website" target="_blank">Bevy Website repository on GitHub</a>. The Bevy Book content is written in Markdown. 

### Building the Website

The website is built using the <a href="https://www.getzola.org/" target="_blank">Zola static site generator</a>. Download Zola, then do the following:

1. Clone the Bevy Website git repo and move to that directory: 
    ```
    git clone https://github.com/bevyengine/bevy-website.git
    cd bevy-website
    ```
2. Start the Zola server
    ```
    zola serve
    ```

A local server should start and you should be able to access a local version of the website from there.

### Rust API Doc Syntax

We made an extension to the markdown syntax that makes linking to Rust API docs nicer. It also gives the links special formatting. Here are some examples:

* Full Type Path: {{rust_type(type="struct", mod="std::collections", name="HashMap")}}
    
    ```{{curly_open()}}{rust_type(type="struct", mod="std::collections", name="HashMap")}{{curly_close()}}```
* Short Type: {{rust_type(type="struct", mod="std::collections", name="HashMap", short=true)}}
    
    ```{{curly_open()}}{rust_type(type="struct", mod="std::collections", name="HashMap", short=true)}{{curly_close()}}```
* Plural Type: {{rust_type(type="struct", mod="std::collections", name="HashMap", short=true, plural=true)}}
    
    ```{{curly_open()}}{rust_type(type="struct", mod="std::collections", name="HashMap", short=true, plural=true)}{{curly_close()}}```
* Function: {{rust_type(type="struct", mod="std::collections", name="HashMap", short=true, method="insert")}}
    
    ```{{curly_open()}}{rust_type(type="struct", mod="std::collections", name="HashMap", short=true, method="insert")}{{curly_close()}}```
* Module: {{rust_mod(mod="std::collections")}}
    
    ```{{curly_open()}}{rust_mod(mod="std::collections")}{{curly_close()}}```

{{rust_mod(mod="std")}} modules will link to <a href="https://doc.rust-lang.org" target="_blank">doc.rust-lang.org</a>. Other modules (like {{rust_mod(mod="bevy")}} ) will link to <a href="https://docs.rs" target="_blank">docs.rs</a> 

## Rust API Docs

Bevy's Rust API Docs are automatically generated from the latest Bevy source code. If you add <a href="https://doc.rust-lang.org/book/ch14-02-publishing-to-crates-io.html#making-useful-documentation-comments" target="_blank">Rust documentation comments</a> to the Bevy codebase, the API docs will be automatically updated.