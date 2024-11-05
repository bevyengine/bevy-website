<!-- Add more tools for traversing hierarchies -->
<!-- https://github.com/bevyengine/bevy/pull/15627 -->

We've spruced up the [`HierarchyQueryExt`] [extension trait],
making it easier to traverse entity hierarchies defined by the [`Parent`] and [`Children`] components.

The full set of methods is now:

- `parent` (new)
- `children` (new)
- `root_ancestor` (new)
- `iter_leaves` (new)
- `iter_siblings` (new)
- `iter_descendants`
- `iter_descendants_depth_first` (new)
- `iter_ancestors`

All of these operations were previously possible, but we hope that this API makes working with hierarchies more pleasant, especially for UI and animation.

[`HierarchyQueryExt`]: https://docs.rs/bevy/0.15/bevy/hierarchy/trait.HierarchyQueryExt.html
[`Parent`]: https://docs.rs/bevy/0.15/bevy/hierarchy/struct.Parent.html
[`Children`]: https://docs.rs/bevy/0.15/bevy/hierarchy/struct.Children.html
[extension trait]: https://rust-lang.github.io/rfcs/0445-extension-trait-conventions.html
