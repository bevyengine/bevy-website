<!-- Allow volumetric fog to be localized to specific, optionally voxelized, regions. -->
<!-- https://github.com/bevyengine/bevy/pull/14099 -->

**Bevy 0.15** adds the concept of "fog volumes". These are entities with the [`FogVolume`] component, which defines a bounding box for fog, which can be scaled and positioned to define where the fog will be rendered.

A camera with the [`VolumetricFog`] component will render any [`FogVolume`] entities in its view. Fog volumes can also define a density texture, which is a 3D texture of voxels that specify the density of the fog at each point:

![fog volume](fog_volume.png)

[`FogVolume`] has a `density_texture_offset`, which allows the 3D texture to be "scrolled". This allows effects such as clouds "passing through" the volume:

<video controls><source src="scrolling_fog.mp4" type="video/mp4"/></video>
