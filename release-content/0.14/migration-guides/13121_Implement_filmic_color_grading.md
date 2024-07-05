The `ColorGrading` component has been expanded to support individually configuring the shadow, midtone, and highlight sections. If you configured the `gamma` or `pre_saturation` fields previously, you will now have to set them for all sections:

```rust
// 0.13
let color_grading = ColorGrading {
    gamma: 2.0,
    pre_saturation: 0.8,
    ..default()
};

// 0.14
let mut color_grading = ColorGrading::default();

for section in color_grading.all_sections_mut() {
    section.gamma = 2.0;
    // `pre_saturation` has been renamed to `saturation`.
    section.saturation = 0.8;
}
```

Additionally, the `post_saturation` and `exposure` fields have been moved specifically to the new `global` field, which is a `ColorGradingGlobal` that supports more operations for the image as a whole.

```rust
// 0.13
let color_grading = ColorGrading {
    post_saturation: 1.2,
    exposure: 0.4,
};

// 0.14
let color_grading = ColorGrading {
    global: ColorGradingGlobal {
        post_saturation: 1.2,
        exposure: 0.4,
        ..default()
    },
    ..default()
};
```
