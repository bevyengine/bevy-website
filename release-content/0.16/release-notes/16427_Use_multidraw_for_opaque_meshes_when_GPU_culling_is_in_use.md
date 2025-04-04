<!-- Use multidraw for opaque meshes when GPU culling is in use. -->
<!-- https://github.com/bevyengine/bevy/pull/16427 -->

We've now added support for *multidraw*. This is a feature that allows multiple meshes to be drawn
in a single draw call, which can be a significant performance improvement by reducing the number of
draw culls as well as the number of rebindings. As a side-effect, GPU culling has now been enabled
by default!

One caveat with multidraw is that it currently only works on Vulkan. We believe we can close this
gap in future Bevy versions. Another caveat is that multidraw currently does not apply to non-opaque
meshes or 2D meshes.
