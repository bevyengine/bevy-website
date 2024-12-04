Image formats that previously weren’t feature-gated are now feature-gated, meaning they will have to be enabled if you use them:

- `avif`
- `ff` (Farbfeld)
- `gif`
- `ico`
- `tiff`

Additionally, the `qoi` feature has been added to support loading QOI format images.

Previously, these formats appeared in the enum by default, but weren’t actually enabled via the `image` crate, potentially resulting in weird bugs. Now, you should be able to add these features to your projects to support them properly.

---

If you were individually configuring the `bevy_render` crate, the feature flags for the general image formats were moved to `bevy_image` instead. For example, `bevy_render/png` no longer exists, and `bevy_image/png` is the new location for this. The texture formats are still available on `bevy_render`, e.g. `bevy_render/ktx2` is needed to fully enable `ktx2` support, and this will automatically enable `bevy_image/ktx2` for loading the textures.
