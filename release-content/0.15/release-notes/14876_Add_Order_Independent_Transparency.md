<!-- Add Order Independent Transparency -->
<!-- https://github.com/bevyengine/bevy/pull/14876 -->

<video controls autoplay loop muted><source src="oit-demo.mp4" type="video/mp4"/></video>

Before this feature, bevy only used alpha blending to render transparent meshes. We now have the option to use Order Independent Transparency when rendering transparent meshes. Instead of only sorting the mesh, this will sort every pixel that contributes to a transparent triangle. This is useful if you have a lot of transparent layers in your scene.

The implementation is currently pretty simple and uses a lot of gpu memory but it should always render perfectly accurate transparency as long as you have configured enough layers. This feature is still a work in progress and we will keep working on improving it.

This feature was contributed to bevy by Foresight Spatial Labs. It is based on an internal implementation they use in their applications.

