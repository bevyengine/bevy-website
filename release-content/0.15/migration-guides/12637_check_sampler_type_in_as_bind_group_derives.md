Instead of panicking, the `AsBindGroup` derive can now fail.
To accommodate this, `PrepareAssetError` now has another arm: `PrepareAssetError::AsBindGroupError`.
If you were exhaustively matching, you now need to handle this failure mode.
