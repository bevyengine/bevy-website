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
