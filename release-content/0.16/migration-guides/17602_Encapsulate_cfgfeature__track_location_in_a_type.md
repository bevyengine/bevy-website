Methods like `Ref::changed_by()` that used to return a `&'static Location<'static>` will now be available even when the `track_location` feature is disabled, but they will now return the new `MaybeLocation` type. `MaybeLocation` wraps a `&'static Location<'static>` when the feature is enabled, and is a ZST when the feature is disabled.

Existing code that needs a `&Location` can call `MaybeLocation::into_option()` to recover it. Many trait impls are forwarded, so if you only need `Display` then no changes will be necessary.

If that code was conditionally compiled, you may instead want to use the methods on `MaybeLocation` to remove the need for conditional compilation.

Code that constructs a `Ref`, `Mut`, `Res`, or `ResMut` will now need to provide location information unconditionally.  If you are creating them from existing Bevy types, you can obtain a `MaybeLocation` from methods like `Table::get_changed_by_slice_for()` or `ComponentSparseSet::get_with_ticks`. Otherwise, you will need to store a `MaybeLocation` next to your data and use methods like `as_ref()` or `as_mut()` to obtain wrapped references.
