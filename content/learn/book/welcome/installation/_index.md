+++
title = "Installing Rust and Bevy"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
+++

## Installing Rust

TODO: steal from old book

## Installing Bevy

TODO: steal from old book

## Troubleshooting

Having trouble getting Bevy running?
Take a look at these known problems and how to solve them.

### Unable to find a GPU

```
thread 'main' panicked at 'Unable to find a GPU! Make sure you have installed required drivers!'
```

This error message means that bevy is unable to draw to your screen.
Causes include:

1. Vulkan-compatible drivers not installed. To fix this, install/update the drivers. On Linux this may be `vulkan-intel` or `vulkan-radeon`.
2. Trying to run an example on a headless machine. To fix this, install a GPU!
