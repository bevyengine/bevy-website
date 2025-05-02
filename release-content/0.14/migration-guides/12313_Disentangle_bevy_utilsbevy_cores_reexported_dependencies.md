`bevy::utils` no longer re-exports `petgraph`, `uuid`, `nonmax`, `smallvec`, or `thiserror`. Additionally, `bevy::core` no longer re-exports `bytemuck`'s `bytes_of`, `cast_slice`, `Pod`, and `Zeroable`.

If you need any of these as dependencies, you can add them to your own `Cargo.toml`.
