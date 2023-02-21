# Windows local build instructions

Building bevy-website locally is tedious, especially on windows. Here is one (brute force) method:

1. [Install WSL](https://learn.microsoft.com/en-us/windows/wsl/install), with Ubuntu.

note: You could instead use [git bash](https://gitforwindows.org/), but some of the details might be different.

2. Open a WSL bash shell
3. [install rustup](https://rustup.rs/) `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
4. Add the wasm target `rustup target add wasm32-unknown-unknown`
5. Install build dependencies `sudo apt install build-essential pkg-config`
6. Install wasm-bindgen tool `cargo install wasm-bindgen-cli`

Now we have everything we need to start generating the site. We can roughly follow the
outline of the CI build script `.github/workflows/ci.ympl`

7. cd into the website repo. There are a bunch of directories starting with "generate-".
   cd into these directories and run the associated scripts.
   ```
   cd generate-assets
   ./generate_assets.sh
   cd ..

   cd generate-errors
   ./generate_errors.sh
   cd ..

   cd generate-wasm-examples
   ./generate_wasm_examples.sh
   cd ..

   cd generate-community
   ./generate_community.sh
   ```

Now you can can use `zola serve` from the root of the repository. This should work both in
cmd/powershell, or in the WSL bash shell - just make sure you've
[installed zola](https://www.getzola.org/documentation/getting-started/installation/) for whichever
shell you are using.

note: You will need to regenerate these assets manually each time.
