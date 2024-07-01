<!-- Implement PBR anisotropy per `KHR_materials_anisotropy`. -->
<!-- https://github.com/bevyengine/bevy/pull/13450 -->

[Anisotropic materials](https://en.wikipedia.org/wiki/Anisotropy) change based on the axis of motion, such as how wood behaves very differently when working with versus against the grain.
But in the context of phsically-based rendering, **anisotropy** refers specifically to a feature that allows roughness to vary along the tangent and bitangent directions of a mesh.
In effect, this causes the specular light to stretch out into lines instead of a round lobe. This is useful for modeling brushed metal, hair, and similar surfaces.
Support for anisotropy is a common feature in major game and graphics engines; Unity, Unreal, Godot, three.js, and Blender all support it to varying degrees.

{{ compare_slider(
    path="POST_PATH",
    left_title="Without Anisotropy",
    left_image="without_anisotropy.png",
    right_title="With Anisotropy",
    right_image="with_anisotropy.png"
) }}

Two new parameters have been added to [`StandardMaterial`](https://docs.rs/bevy/0.14/bevy/pbr/struct.StandardMaterial.html): `anisotropy_strength` and `anisotropy_rotation`.
Anisotropy strength, which ranges from 0 to 1, represents how much the roughness differs between the tangent and the bitangent of the mesh.
In effect, it controls how stretched the specular highlight is. Anisotropy rotation allows the roughness direction to differ from the tangent of the model.

In addition to these two fixed parameters, an anisotropy texture can be supplied.
Such a texture should be a 3-channel RGB texture, where the red and green values specify a direction vector using the same conventions as a normal map ([0, 1] color values map to [-1, 1] vector values), and the blue value represents the strength.
This matches the format that the `KHR_materials_anisotropy` specification requires.
Such textures should be loaded as linear and not sRGB.
Note that this texture does consume one additional texture binding in the standard material shader.

Like always, give it a spin at the corresponding [`anisotropy` example](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/3d/anisotropy.rs).
