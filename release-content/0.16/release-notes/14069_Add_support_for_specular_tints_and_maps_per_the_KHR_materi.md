<!-- Add support for specular tints and maps per the `KHR_materials_specular` glTF extension. -->
<!-- https://github.com/bevyengine/bevy/pull/14069 -->

If you have an eye for light (or training in visual arts), you'll notice that shiny curved surfaces get extra-bright spots of light.
That's a specular highlight!
In Bevy 0.16, we've implemented a standard physically-based rendering (PBR) feature of specular highlights: the ability to tint their color.

![A shiny floating sphere with an iridescent multicolored sheen. It reminds you of a Christmas tree ornament. You can see a reflection of a city scene in it.](specular-tint-sphere.png)

This can be done uniformly across the material, by simply setting the `specular_tint` field on the [`StandardMaterial`] for your object.

Like many other material properties (color, normals, emissiveness, roughness etc), this can be varied over the material via the use of a texture map,
which describes how this property varies via a 2-dimensional UV-space image.

Maps are relatively expensive: you need an entire 2D image, rather than a single float or color, and GPUs have limits on the number of textures you can use per material, unless they support bindless textures.
As a result, specular maps are off by default, and gated behind the `pbr_specular_textures` Cargo feature.

To support this work, we now support the KHR_materials_specular glTF extension, allowing artists to set these properties in 3D modelling tools like Blender, then import them into Bevy.

[`StandardMaterial`]: https://dev-docs.bevyengine.org/bevy/pbr/struct.StandardMaterial.html
