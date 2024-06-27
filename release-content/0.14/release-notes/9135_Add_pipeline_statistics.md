When working on rendering features (in the engine and in projects), it's useful to know how expensive they are. 
[Tracy](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md) already lets us measure CPU time per system, but we didn't have a way of measuring how long the GPU spends on each render node.

The [`RenderDiagnosticsPlugin`](https://dev-docs.bevyengine.org/bevy/render/diagnostic/struct.RenderDiagnosticsPlugin.html) provides this information,
collecting both information about the CPU/GPU spent per node and [pipeline](https://sotrh.github.io/learn-wgpu/beginner/tutorial3-pipeline/) statistics such as the number of triangles drawn.
This data is stored in Bevy's central [`DiagnosticsStore`](https://dev-docs.bevyengine.org/bevy/diagnostic/struct.DiagnosticsStore.html),
and can be accessed by calling `DiagnosticsStore::iter()` to get a list of all registered diagnostics.
The [`DiagnosticPath`](https://dev-docs.bevyengine.org/bevy/diagnostic/struct.DiagnosticPath.html) contained in the [`Diagnostic`](https://dev-docs.bevyengine.org/bevy/diagnostic/struct.Diagnostic.html)s returned can also be used to fetch specific diagnostics for more focused investigation.

By default, this feature will track:

- Elapsed CPU time
- Elapsed GPU time
- [Vertex shader](https://www.khronos.org/opengl/wiki/Vertex_Shader) invocations
- [Fragment shader](https://www.khronos.org/opengl/wiki/Fragment_Shader) invocations
- [Compute shader](https://www.khronos.org/opengl/wiki/Compute_Shader) invocations
- [Clipper invocations](http://gpa.helpmax.net/en/intel-graphics-performance-analyzers-help/metrics-descriptions/extended-metrics-description/rasterizer-metrics/clipper-invocations/)
- [Clipper primitives](http://gpa.helpmax.net/en/intel-graphics-performance-analyzers-help/metrics-descriptions/extended-metrics-description/rasterizer-metrics/post-clip-primitives/)

but as outlined in the documentation, you can use this feature to track additional information that you might care about.
