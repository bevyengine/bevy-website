While [Tracy](https://github.com/bevyengine/bevy/blob/main/docs/profiling.md) already lets us measure CPU time per system, our GPU diagnostics are much weaker.
In Bevy 0.14 we've added support for two classes of rendering-focused statistics via the [`RenderDiagnosticsPlugin`](https://docs.rs/bevy/0.14/bevy/render/diagnostic/struct.RenderDiagnosticsPlugin.html):

1. **Timestamp queries:** how long did specific bits of work take on the GPU?
2. **Pipeline statistics:** information about the quantity of work sent to the GPU.

While it may sound like timestamp queries are the ultimate diagnostic tool, they come with several caveats.
Firstly, they vary quite heavily from frame-to-frame as GPUs dynamically ramp up and down clock speed due to workload (idle gaps in GPU work, e.g., a bunch of consecutive barriers, or the tail end of a large dispatch) or the physical temperature of the GPU.
To get an accurate measurement, you need to look at summary statistics: mean, median, 75th percentile and so on.

Secondly, while timestamp queries will tell you how long something takes, but it will not tell you why things are slow.
For finding bottlenecks, you want to use a GPU profiler from your GPU vendor (Nvidia's NSight, AMD's RGP, Intel's GPA or Apple's XCode).
These tools will give you much more detailed stats about cache hit rate, warp occupancy, and so on.
On the other hand they lock your GPU's clock to base speeds for stable results, so they won't give you a good indicator of real world performance.

[`RenderDiagnosticsPlugin`] tracks the following pipeline statistics, recorded in Bevy's [`DiagnosticsStore`](https://docs.rs/bevy/0.14/bevy/diagnostic/struct.DiagnosticsStore.html): Elapsed CPU time, Elapsed GPU time, [Vertex shader](https://www.khronos.org/opengl/wiki/Vertex_Shader) invocations, [Fragment shader](https://www.khronos.org/opengl/wiki/Fragment_Shader) invocations, [Compute shader](https://www.khronos.org/opengl/wiki/Compute_Shader) invocations, [Clipper invocations](http://gpa.helpmax.net/en/intel-graphics-performance-analyzers-help/metrics-descriptions/extended-metrics-description/rasterizer-metrics/clipper-invocations/), and [Clipper primitives](http://gpa.helpmax.net/en/intel-graphics-performance-analyzers-help/metrics-descriptions/extended-metrics-description/rasterizer-metrics/post-clip-primitives/).

You can also track individual render/compute passes, groups of passes (e.g. all shadow passes), and individual commands inside passes (like draw calls).
 To do so, instrument them using methods from the [`RecordDiagnostics`](https://docs.rs/bevy/0.14/bevy/render/diagnostic/trait.RecordDiagnostics.html) trait.

[`RenderDiagnosticsPlugin`]: http://dev-docs.bevyengine.org/bevy/render/diagnostic/struct.RenderDiagnosticsPlugin.html