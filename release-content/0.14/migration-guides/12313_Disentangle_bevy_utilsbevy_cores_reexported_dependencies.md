`bevy_utils` no longer re-exports `petgraph`, `uuid`, `nonmax`, `smallvec`, or `thiserror`.

`bevy_core` no longer re-exports `bytemuck`’s `bytes_of`, `cast_slice`, `Pod`, and `Zeroable`.

You can add these as dependencies in your own `Cargo.toml` instead.
