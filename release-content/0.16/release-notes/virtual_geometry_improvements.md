Virtual geometry (the `meshlet` cargo feature) is Bevy's Nanite-like rendering system, allowing much greater levels of geometry density than is otherwise possible, and freeing artists from manually creating LODs.

In Bevy 0.16, virtual geometry got some performance improvements thanks to new algorithms and GPU APIs.

Read more details on the [author's blog post](https://jms55.github.io/posts/2025-03-27-virtual-geometry-bevy-0-16).

Users are not required to regenerate their [`MeshletMesh`](https://dev-docs.bevyengine.org/bevy/pbr/experimental/meshlet/struct.MeshletMesh.html) assets, but doing so is recommended in order to take advantage of the improved clustering algorithm in Bevy 0.16.

![Screenshot of the meshlet example](meshlet_bunnies.png)
