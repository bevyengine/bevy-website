If you wish to optimize something, you must first measure and understand it.
When looking at the performance of applications, [tracy] is our tool of choice.
It gives us a clear understanding of how long work takes, when it happens relative to other work each frame,
and how various threads are used.
Read our [profiling docs] to get started!

But until now, it's had a critical limitation: work done on the GPU wasn't shown,
forcing devs to pull up dedicated GPU-focused tools (like [NSight] or [RenderDoc]) and struggle to piece together an intuition for how it all fits together.

In 0.16, we've connected the [rendering diagnostics added in Bevy 0.14] to [tracy], creating a cohesive picture of
all of the work that's being done in a Bevy application in a single convenient place.

That said, we've only instrumented a few of our passes so far.
While we will improve this in the future, you will need to add spans to your own custom rendering code,
and specialized GPU diagnostic tools will always be more powerful: capturing all GPU-related work done,
and providing more detailed information.

Special thanks to [@wumpf] for trailblazing this work in the excellent [wgpu-profiler] tool, and demonstrating how to wire [wgpu] and [tracy] together.

[tracy]: https://github.com/wolfpld/tracy
[profiling docs]: https://github.com/bevyengine/bevy/blob/main/docs/profiling.md
[NSight]: https://developer.nvidia.com/nsight-systems
[RenderDoc]: https://renderdoc.org/
[@wumpf]: https://github.com/Wumpf
[wgpu-profiler]: https://github.com/Wumpf/wgpu-profiler
[rendering diagnostics added in Bevy 0.14]: https://bevyengine.org/news/bevy-0-14/#tools-for-profiling-gpu-performance
