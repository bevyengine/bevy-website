The following methods (some removed in previous PRs) are now replaced by `Access::try_iter_component_access`:

- `Access::component_reads_and_writes`
- `Access::component_reads`
- `Access::component_writes`

As `try_iter_component_access` returns a `Result`, you’ll now need to handle the failing case (e.g., `unwrap()`). There is currently a single failure mode, `UnboundedAccess`, which occurs when the `Access` is for all `Components` _except_ certain exclusions. Since this list is infinite, there is no meaningful way for `Access` to provide an iterator. Instead, get a list of components (e.g., from the `Components` structure) and iterate over that instead, filtering using `Access::has_component_read`, `Access::has_component_write`, etc.

Additionally, you’ll need to `filter_map` the accesses based on which method you’re attempting to replace:

- `Access::component_reads_and_writes` -> `Exclusive(_) | Shared(_)`
- `Access::component_reads` -> `Shared(_)`
- `Access::component_writes` -> `Exclusive(_)`

To ease migration, please consider the below extension trait which you can include in your project:

```rust
pub trait AccessCompatibilityExt {
    /// Returns the indices of the components this has access to.
    fn component_reads_and_writes(&self) -> impl Iterator<Item = T> + '_;

    /// Returns the indices of the components this has non-exclusive access to.
    fn component_reads(&self) -> impl Iterator<Item = T> + '_;

    /// Returns the indices of the components this has exclusive access to.
    fn component_writes(&self) -> impl Iterator<Item = T> + '_;
}

impl<T: SparseSetIndex> AccessCompatibilityExt for Access<T> {
    fn component_reads_and_writes(&self) -> impl Iterator<Item = T> + '_ {
        self
            .try_iter_component_access()
            .expect("Access is unbounded. Please refactor the usage of this method to directly use try_iter_component_access")
            .filter_map(|component_access| {
                let index = component_access.index().sparse_set_index();

                match component_access {
                    ComponentAccessKind::Archetypal(_) => None,
                    ComponentAccessKind::Shared(_) => Some(index),
                    ComponentAccessKind::Exclusive(_) => Some(index),
                }
            })
    }

    fn component_reads(&self) -> impl Iterator<Item = T> + '_ {
        self
            .try_iter_component_access()
            .expect("Access is unbounded. Please refactor the usage of this method to directly use try_iter_component_access")
            .filter_map(|component_access| {
                let index = component_access.index().sparse_set_index();

                match component_access {
                    ComponentAccessKind::Archetypal(_) => None,
                    ComponentAccessKind::Shared(_) => Some(index),
                    ComponentAccessKind::Exclusive(_) => None,
                }
            })
    }

    fn component_writes(&self) -> impl Iterator<Item = T> + '_ {
        self
            .try_iter_component_access()
            .expect("Access is unbounded. Please refactor the usage of this method to directly use try_iter_component_access")
            .filter_map(|component_access| {
                let index = component_access.index().sparse_set_index();

                match component_access {
                    ComponentAccessKind::Archetypal(_) => None,
                    ComponentAccessKind::Shared(_) => None,
                    ComponentAccessKind::Exclusive(_) => Some(index),
                }
            })
    }
}
```

Please take note of the use of `expect(...)` in these methods. You should consider using these as a starting point for a more appropriate migration based on your specific needs.
