**Decals** are textures which can be dynamically layered on top of existing meshes, conforming to their geometry.
This has two benefits over simply changing a mesh's texture:

1. You can add them dynamically in response to player actions. Most famously, bullet holes in FPS games use decals for this.
2. You don't need to create an entirely new texture for every combination, which makes them more efficient and flexible when creating levels with details like graffiti or cracks in building facades.

Like many things in rendering, there are a huge number of ways to implement this feature, each with their own tradeoffs.
In **Bevy 0.16**, we've selected two complementary approaches: **forward decals** and **clustered decals**.

### Forward Decals

![decals](decals.jpg)

Our implementation of forward decals (or to be more precise, contact projective decals) was inspired by [Alexander Sannikovs talk on the rendering techniques of Path of Exile 2], and was upstreamed from the [`bevy_contact_projective_decals`] ecosystem crate.
Due to nature of this technique, looking at the decal from very steep angles will cause distortion.
This can be mitigated by creating textures that are bigger than the effect, giving the decal more space to stretch.
To create a forward decal, spawn a [`ForwardDecal`] entity, which uses a [`ForwardDecalMaterial`] using the [`ForwardDecalMaterialExt`] material extension.

### Clustered Decals

![clustered decals](clustered-decals.jpg)

Clustered decals (or decal projectors) work by projecting images from a 1x1x1 cube onto surfaces found in the +Z direction.
They are clusterable objects, just like point lights and light probes, which means that decals are only evaluated for objects within the bounds of the projector.
To create a clustered decal, spawn a [`ClusteredDecal`] entity.

Ultimately, forward decals offer broader hardware and driver support, while clustered decals are higher quality and don't require the creation of bounding geometry, improving performance.
Currently clustered decals require bindless textures and thus don't support WebGL2, WebGPU, iOS and Mac targets. Forward decals _are_ available on these targets.

[Alexander Sannikovs talk on the rendering techniques of Path of Exile 2]: https://www.youtube.com/watch?v=TrHHTQqmAaM
[`bevy_contact_projective_decals`]: https://github.com/naasblod/bevy_contact_projective_decals
[`ForwardDecal`]: https://docs.rs/bevy/0.16/bevy/pbr/decal/struct.ForwardDecal.html
[`ForwardDecalMaterial`]: https://docs.rs/bevy/0.16/bevy/pbr/decal/type.ForwardDecalMaterial.html
[`ForwardDecalMaterialExt`]: https://docs.rs/bevy/0.16/bevy/pbr/decal/struct.ForwardDecalMaterialExt.html
[`ClusteredDecal`]: https://docs.rs/bevy/0.16/bevy/pbr/decal/clustered/struct.ClusteredDecal.html
