The `trigger_observers` method now operates on `&[ComponentId]` rather than `impl Iterator<Item=ComponentId`>.

Try replacing `bundle_info.iter_components()` with `bundle_info.components()` or collect the iterator of component ids into a `Vec`.
