<!-- Implement PBR anisotropy per `KHR_materials_anisotropy`. -->
<!-- https://github.com/bevyengine/bevy/pull/13450 -->

> This commit implements support for physically-based anisotropy in Bevy's StandardMaterial, following the specification for the KHR_materials_anisotropy glTF extension.

> Anisotropy (not to be confused with anisotropic filtering) is a PBR feature that allows roughness to vary along the tangent and bitangent directions of a mesh. In effect, this causes the specular light to stretch out into lines instead of a round lobe. This is useful for modeling brushed metal, hair, and similar surfaces. Support for anisotropy is a common feature in major game and graphics engines; Unity, Unreal, Godot, three.js, and Blender all support it to varying degrees.

> Two new parameters have been added to StandardMaterial: anisotropy_strength and anisotropy_rotation. Anisotropy strength, which ranges from 0 to 1, represents how much the roughness differs between the tangent and the bitangent of the mesh. In effect, it controls how stretched the specular highlight is. Anisotropy rotation allows the roughness direction to differ from the tangent of the model.

> In addition to these two fixed parameters, an anisotropy texture can be supplied. Such a texture should be a 3-channel RGB texture, where the red and green values specify a direction vector using the same conventions as a normal map ([0, 1] color values map to [-1, 1] vector values), and the the blue value represents the strength. This matches the format that the KHR_materials_anisotropy specification requires. Such textures should be loaded as linear and not sRGB. Note that this texture does consume one additional texture binding in the standard material shader.
