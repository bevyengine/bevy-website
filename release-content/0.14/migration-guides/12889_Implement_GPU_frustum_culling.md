For phase items, the `dynamic_offset: Option<NonMaxU32>` field is now `extra_index: PhaseItemExtraIndex`, which wraps a `u32`. Instead of `None`, use `PhaseItemExtraIndex::NONE`.

This change affects `AlphaMask3d`, `AlphaMask3dDeferred`, `AlphaMask3dPrepass`, `Opaque2d`, `Opaque3dDeferred`, `Opaque3dPrepass`, `Shadow`, `Transmissive3d`, `Transparent2d`, `Transparent3d`, and `TransparentUi`.
