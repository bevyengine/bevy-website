Virtual geometry (the `meshlet` feature) got a ton of improvements in Bevy 0.15. It's still not production ready, and will remain as an experimental module, but performance has been greatly improved upon since the last release.

For all the interesting details, read the [author's blog post](https://jms55.github.io/posts/2024-11-14-virtual-geometry-bevy-0-15).

{% callout(type="warning") %}
For existing users of this feature:
* Your GPU must now support `WgpuFeatures::SHADER_INT64_ATOMIC_MIN_MAX` to use this feature. As forewarned in the previous release, older GPUs may no longer be compatible.
* You must regenerate your MeshletMesh assets. MeshletMesh assets generated in Bevy 0.14 are not compatible with Bevy 0.15.
* Make sure you read both the migration guide and the updated rustdoc for full details on how to upgrade your project.
{% end %}
