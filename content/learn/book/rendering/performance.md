+++
title = "TODO"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

* Profile first! Really!! (Link to that part of the book)
  * CPU vs GPU bottlenecks, render world and render commands vs GPU timeline
* Mesh/material auto-batching and pipeline/draw calls, MeshTag
* RenderDiagnostics
* Use a single-digits amount of shadow-casting lights
* Don't use more than ~10(?) lights affecting the same pixel
* Deferred vs forward vs forward with prepass
* Overdraw, culling, and wasted work
* RenderAssetUsages to save RAM
* VRAM usage in general (texture size, mesh size, etc)
* Optimize textures:
  * Texture size (4k vs 1080p)
  * Mipmaps (perf, not just quality)
  * GPU-compressed texture formats (BC or ASTC)
  * CPU-compressed files (zstd) (different than GPU-compressed _formats_)
* Give users the option to turn off more expensive features (e.g. lower SSAO quality, or adjust shadow map resolution)
* LOD/VisibilityRange/VirtualGeometry
