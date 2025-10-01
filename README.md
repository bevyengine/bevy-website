# Bevy Website

The source files for <https://bevy.org>. This includes official Bevy news, docs, and interactive examples.

If you would like to contribute, check out [CONTRIBUTING.md](/CONTRIBUTING.md) and then submit a pull request!

## Zola

The Bevy website is built using the Zola static site engine. In our experience, it is fast, flexible, and straightforward to use.

To check out any local changes you've made:

1. [Install Zola](https://www.getzola.org/documentation/getting-started/installation/) version `0.18.0` or later.
2. Clone the Bevy Website git repo and enter that directory:
   1. `git clone https://github.com/bevyengine/bevy-website.git`
   2. `cd bevy-website`
3. Start the Zola server with `zola serve`.

A local server should start and you should be able to access a local version of the website from there.

### Assets, Errors, and Examples pages

These pages need to be generated in a separate step by running the shell scripts in the `generate-assets`, `generate-errors`, and `generate-wasm-examples` directories. On Windows, you can use [WSL](https://learn.microsoft.com/en-us/windows/wsl/install) or [git bash](https://gitforwindows.org/).

The Examples page uses the same Wasm binaries as https://bevy.org/examples by default. To build examples locally, have a look at `generate-wasm-examples/build_wasm_examples_debug.sh`. After running it, start zola with `zola --config config.local.toml serve` to use the locally built examples.
