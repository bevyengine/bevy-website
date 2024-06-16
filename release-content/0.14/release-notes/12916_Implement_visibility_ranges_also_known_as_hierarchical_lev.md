<!-- Implement visibility ranges, also known as hierarchical levels of detail (HLODs). -->
<!-- https://github.com/bevyengine/bevy/pull/12916 -->

<!-- TODO -->

> Implement visibility ranges, also known as hierarchical levels of detail (HLODs).
>
> This commit introduces a new component, VisibilityRange, which allows developers to specify camera distances in which meshes are to be shown and hidden. Hiding meshes happens early in the rendering pipeline, so this feature can be used for level of detail optimization. Additionally, this feature is properly evaluated per-view, so different views can show different levels of detail.
> 
> This feature differs from proper mesh LODs, which can be implemented later. Engines generally implement true mesh LODs later in the pipeline; they're typically more efficient than HLODs with GPU-driven rendering. However, mesh LODs are more limited than HLODs, because they require the lower levels of detail to be meshes with the same vertex layout and shader (and perhaps the same material) as the original mesh. Games often want to use objects other than meshes to replace distant models, such as octahedral imposters or billboard imposters.
> 
> The reason why the feature is called hierarchical level of detail is that HLODs can replace multiple meshes with a single mesh when the camera is far away. This can be useful for reducing drawcall count. Note that VisibilityRange doesn't automatically propagate down to children; it must be placed on every mesh.
> 
> Crossfading between different levels of detail is supported, using the standard 4x4 ordered dithering pattern from 1. The shader code to compute the dithering patterns should be well-optimized. The dithering code is only active when visibility ranges are in use for the mesh in question, so that we don't lose early Z.
> 
> Cascaded shadow maps show the HLOD level of the view they're associated with. Point light and spot light shadow maps, which have no CSMs, display all HLOD levels that are visible in any view. To support this efficiently and avoid doing visibility checks multiple times, we precalculate all visible HLOD levels for each entity with a VisibilityRange during the check_visibility_range system.
> 
> A new example, visibility_range, has been added to the tree, as well as a new low-poly version of the flight helmet model to go with it. It demonstrates use of the visibility range feature to provide levels of detail.
