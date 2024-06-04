If you were depending on the `PartialOrd` behaviour of `FloatOrd`, it has changed from matching `f32` to matching `FloatOrd`’s `Ord` ordering, never returning `None`.
