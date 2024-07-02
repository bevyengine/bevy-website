Previously, StandardMaterial always defaulted to using ATTRIBUTE_UV_0 for each texture except lightmap, which isn't flexible enough for a lot of gltf files. In **Bevy 0.14**, a new UvChannel enum was added allowing you to select the channel to use for each texture in StandardMaterial.

Here's a before and after showing the support of ATTRIBUTE_UV_1 across textures:

![UV Channel Selection](uv_channel_selection.png)
