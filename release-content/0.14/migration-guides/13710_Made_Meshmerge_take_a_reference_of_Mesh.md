`Mesh::merge()` now takes `&Mesh` instead of `Mesh`. Because of this, you can now share the same `Mesh` across multiple `merge()` calls without cloning it.
