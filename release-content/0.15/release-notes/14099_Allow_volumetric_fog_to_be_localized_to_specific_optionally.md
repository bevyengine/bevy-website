<!-- Allow volumetric fog to be localized to specific, optionally voxelized, regions. -->
<!-- https://github.com/bevyengine/bevy/pull/14099 -->

**Bevy 0.15** adds the concept of "fog volumes". These are entities with the [`FogVolume`] component, which defines a bounding box for fog, which can be scaled and positioned to define where the fog will be rendered.

A camera with the [`VolumetricFog`] component will render any [`FogVolume`] entities in its view. Fog volumes can also define a density texture, which is a 3D texture of voxels that specify the density of the fog at each point:

![fog volume](fog_volume.png)
