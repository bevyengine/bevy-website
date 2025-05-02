The `Point` trait has been replaced by `VectorSpace`. These traits are very similar, with a few minor changes:

- `VectorSpace` implementations must now provide the `ZERO` constant.
- `VectorSpace` now requires the `Div<f32, Output = Self>` and `Neg` trait bounds.
- `VectorSpace` no longer requires the `Add<f32, Output = Self>`, `Sum`, and `PartialEq` trait bounds.

For most cases you can replace all `Point` usage with `VectorSpace`, but you may have to make further changes if you depend on anything in the list above.
