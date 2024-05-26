- `LoadContext::load_direct` has been renamed to `LoadContext::load_direct_untyped`. You may find the new `load_direct` is more appropriate for your use case (and the migration may only be moving one type parameter).
- `LoadContext::load_direct_with_reader` has been renamed to `LoadContext::load_direct_untyped_with_reader`.
---

This might not be an obvious win as a solution because it introduces quite a few new `load_direct` alternatives - but it does follow the existing pattern pretty well. I’m very open to alternatives. :sweat_smile:
