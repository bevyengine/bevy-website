<!-- Implement motion vectors and TAA for skinned meshes and meshes with morph targets. -->
<!-- https://github.com/bevyengine/bevy/pull/13572 -->

Back in **Bevy 0.11** we added [Temporal Anti Aliasing (TAA)](/news/bevy-0-11/#temporal-anti-aliasing), which uses Motion Vectors to determine how fast an object is moving. However, in **Bevy 0.11** we only added Motion Vector support for "static" meshes, meaning TAA did not work for animated meshes using skeletal animation or morph targets.

In **Bevy 0.14**, we implemented [Per-Object Motion Blur](#per-object-motion-blur), which _also_ uses Motion Vectors and therefore would have that same limitation.

Fortunately in **Bevy 0.14** we implemented Motion Vectors for skinned meshes and meshes with morph targets, closing this gap and enabling TAA, Per-Object Motion Blur, and future Motion Vector features to work with animated meshes.
