The `bevy/wgpu_trace` and `bevy_render/wgpu_trace` features have been removed, as WGPU tracing is now enabled during the creation of `bevy_render::RenderPlugin`.

{% callout(type="info") %}
At the time of writing, WGPU has not reimplemented tracing support, so WGPU tracing will not currently work. However, once WGPU has reimplemented tracing support, the steps below should be sufficient to continue generating WGPU traces.

You can track the progress of WGPU tracing being reimplemented at [gfx-rs/wgpu#5974](https://github.com/gfx-rs/wgpu/issues/5974).
{% end %}

To continue generating WGPU traces:

1. Remove any instance of the `bevy/wgpu_trace` or `bevy_render/wgpu_trace` features you may have in any of your `Cargo.toml` files.
2. Follow the instructions in `docs/debugging.md` file in the repository, under the WGPU Tracing section.
