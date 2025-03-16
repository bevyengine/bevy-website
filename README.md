# Bevy Website

The source files for <https://bevyengine.org>. This includes official Bevy news, docs, and interactive examples.

If you would like to contribute, check out [CONTRIBUTING.md](/CONTRIBUTING.md) and then submit a pull request!

## Zola

The Bevy website is built using the Zola static site engine. In our experience, it is fast, flexible, and straightforward to use.

To check out any local changes you've made:

1. [Install Zola](https://www.getzola.org/documentation/getting-started/installation/) version `0.18.0`.
2. Clone the Bevy Website git repo and enter that directory:
   1. `git clone https://github.com/bevyengine/bevy-website.git`
   2. `cd bevy-website`
3. Start the Zola server with `zola serve`.

A local server should start and you should be able to access a local version of the website from there.

### Assets, Errors, and Examples pages

These pages need to be generated in a separate step by running the shell scripts in the `generate-assets`, `generate-errors`, and `generate-wasm-examples` directories. On Windows, you can use [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) or [git bash](https://gitforwindows.org/).

## Search Index

We use [Pagefind](https://pagefind.app) for the search functionality.

### Configuration

Which pages to index, category assignment and the weight is configured in `/templates/macros/pagefind.html`.
Note that the category names must be reflected when instancing `SearchCategories` in `/static/search.mjs`.

Pagefind can be tweaked (ignore content, change weights…) at page and element level by using `data-pagefind-*` HTML attributes.

The default configuration for the Pagefind CLI is defined in `/pagefind.yml`.

### Local Development

To generate the index for local development, download the [Pagefind binary](https://github.com/CloudCannon/pagefind/releases) in the project root and the run:

```sh
rm -rf public
zola build
./pagefind --with-playground
```

This will create the `/static/pagefind` folder which contains the index and JS library.
You can access `http://127.0.0.1:1111/pagefind/playground/` to debug search issues.
