Usage of `PhaseItem` has been split into `BinnedPhaseItem` and `SortedPhaseItem`. If you have custom `PhaseItem`s you will need to choose one of the new types. Notably some phases _must_ be Sorted (such as Transparent and Transmissive), while others can be Binned. Effectively Sorted is "what Bevy did before" and Binned is new, and the point of this change is to avoid sorting when possible for improved performance.

If you're looking for a quick migration, consider picking [`SortedPhaseItem`](https://docs.rs/bevy/0.14.0-rc.4/bevy/render/render_phase/trait.SortedPhaseItem.html) which requires the fewest code changes.

If you're looking for higher performance (and your phase doesn’t require sorting) you may want to pick [`BinnedPhaseItem`](https://docs.rs/bevy/0.14.0-rc.4/bevy/render/render_phase/trait.BinnedPhaseItem.html). Notably bins are populated based on `BinKey` and everything in the same bin is potentially batchable.

If you are only consuming these types, then a `Query` for a type like `&mut RenderPhase<Transparent2d>` will become a `Resource` as such:

```rust
mut transparent_render_phases: ResMut<ViewSortedRenderPhases<Transparent2d>>
```

`ViewSortedRenderPhases` and `ViewBinnedRenderPhases` are used in accordance with which phase items you're trying to access (sorted or binned).

Examples of [`SortedPhaseItems`s](https://docs.rs/bevy/0.14.0-rc.4/bevy/render/render_phase/trait.SortedPhaseItem.html#implementors):

- Transmissive3d
- Transparent2d
- Transparent3d
- TransparentUi

Examples of [`BinnedPhaseItem`s](https://docs.rs/bevy/0.14.0-rc.4/bevy/render/render_phase/trait.BinnedPhaseItem.html#implementors) include:

- Opaque3d
- Opaque3dPrepass
- Opaque3dDeferred
- AlphaMask3d
- AlphaMask3dPrepass
- AlphaMask3dDeferred
- [Shadow](https://docs.rs/bevy/0.14.0-rc.4/bevy/pbr/struct.Shadow.html)

If you do not have a mesh (such as for GPU-driven particles or procedural generation) and want to use the new binned behavior, the [`BinnedRenderPhase`](https://docs.rs/bevy/0.14.0-rc.4/bevy/render/render_phase/struct.BinnedRenderPhase.html) includes a new `non_mesh_items` collection which correlates with a new [`BinnedRenderPhaseType`](https://docs.rs/bevy/0.14.0-rc.4/bevy/render/render_phase/struct.BinnedRenderPhase.html). This type is used when [add](https://docs.rs/bevy/0.14.0-rc.4/bevy/render/render_phase/struct.BinnedRenderPhase.html#method.add)ing items to the `BinnedRenderPhase`.

It may be additionally useful to checkout the new [custom_phase_item example](https://github.com/bevyengine/bevy/blob/5876352206d1bcea792825bf013eb212383b73d6/examples/shader/custom_phase_item.rs) which details some of the new APIs.
