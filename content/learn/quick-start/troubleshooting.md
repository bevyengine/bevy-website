+++
title = "Troubleshooting"
insert_anchor_links = "right"
aliases = ["learn/book/troubleshooting"]
[extra]
weight = 5
+++

Is something in Bevy not working as expected? Maybe one of these will resolve your problem:

## Unable to find a GPU

```txt
thread 'main' panicked at 'Unable to find a GPU! Make sure you have installed required drivers!'
```

This error message means that bevy is unable to draw to your screen.
Causes include:

1. Vulkan-compatible drivers not installed. To fix this, install/update the drivers. On Linux this may be `vulkan-intel` or `vulkan-radeon`.
2. Trying to run an example on a headless machine. To fix this, install a GPU!

## Unable to debug dynamically linked Bevy application in VSCode on Windows

```txt
The program '[10184] my-game.exe' has exited with code -1073741515 (0xc0000135).
```

Whilst `cargo run` may load the application successfully, running via the debugging UI in VSCode may yield the above error. This error means
that the required libraries were not loaded correctly (likely due to a pathing quirk with VSCode debug extensions on Windows).

Edit your launch configurations in `.vscode/launch.json` so that the rust libraries are found correctly.

For `cppvsdbg`:

```json
"environment": [
    {"name":"PATH", "value":"%USERPROFILE%/.rustup/toolchains/nightly-x86_64-pc-windows-msvc/bin;${workspaceFolder}/target/debug/deps;%PATH%"}
    // Switch `nightly` to `stable` if you're using Rust stable
],
```

Or for `codelldb`:

```json
"env": {
    "PATH": "${env:USERPROFILE}/.rustup/toolchains/nightly-x86_64-pc-windows-msvc/bin;${workspaceFolder}/target/debug/deps;${env:PATH}",
    // Switch `nightly` to `stable` if you're using Rust stable
},
```
