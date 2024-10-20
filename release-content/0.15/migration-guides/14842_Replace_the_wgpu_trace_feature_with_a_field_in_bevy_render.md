- The `bevy/wgpu_trace`, `bevy_render/wgpu_trace`, and `bevy_internal/wgpu_trace` features no longer exist. Remove them from your `Cargo.toml`, CI, tooling, and what-not.
- Follow the instructions in the updated `docs/debugging.md` file in the repository, under the WGPU Tracing section.

Because of the changes made, you can now generate traces to any path, rather than the hardcoded `%WorkspaceRoot%/wgpu_trace` (where `%WorkspaceRoot%` is… the root of your crate’s workspace) folder.

(If WGPU hasn’t restored tracing functionality…) Do note that WGPU has not yet restored tracing functionality. However, once it does, the above should be sufficient to generate new traces.
