+++
title = "Troubleshooting"
weight = 5
sort_by = "weight"
template = "book-section.html"
page_template = "book-section.html"
+++

Is something in Bevy not working as expected? Maybe one of these will resolve your problem:

## Unable to find a GPU
```
thread 'main' panicked at 'Unable to find a GPU! Make sure you have installed required drivers!'
```
This error message means that bevy is unable to draw to your screen.
Causes include:
1. Vulkan-compatible drivers not installed. To fix this, install/update the drivers. On Linux this may be `vulkan-intel` or `vulkan-radeon`.
3. Trying to run an example on a headless machine. To fix this, install a GPU!

## `pkg-config` unable to find `libudev`

```
    Compiling libudev-sys v0.1.4
error: failed to run custom build command for `libudev-sys v0.1.4`

Caused by:
  process didn't exit successfully: `/home/tim/Bevy/sniff/target/debug/build/libudev-sys-cbff3ca3e15d7fee/build-script-build` (exit status: 101)
  --- stdout
  cargo:rerun-if-env-changed=LIBUDEV_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=LIBUDEV_STATIC
  cargo:rerun-if-env-changed=LIBUDEV_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR

  --- stderr
  thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: "`\"pkg-config\" \"--libs\" \"--cflags\" \"libudev\"` did not exit successfully: exit status: 1\n--- stderr\nPackage libudev was not found in the pkg-config search path.\nPerhaps you should add the directory containing `libudev.pc'\nto the PKG_CONFIG_PATH environment variable\nNo package 'libudev' found\n"', /home/tim/.cargo/registry/src/github.com-1ecc6299db9ec823/libudev-sys-0.1.4/build.rs:38:41
  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
error: build failed
```

This error indicates that Rust wasn't able to compile one of Bevy's dependencies, `libudev-sys` because the underlying library wasn't found. To fix this, ensure that the the development version of the `libudev` package is installed. On systems that use the `apt` package manager, this command should resolve the problem.

```sh
sudo apt install libudev-dev
```
