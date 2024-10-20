A new asset `ShaderStorageBuffer` has been added to simplify working with storage buffers in custom materials and compute shaders as part of our ongoing work to make working with compute simpler.

```rust
#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
struct CustomMaterial {
    #[storage(0, read_only)]
    colors: Handle<ShaderStorageBuffer>,
}

fn setup(
    mut buffers: ResMut<Assets<ShaderStorageBuffer>>,
    mut materials: ResMut<Assets<CustomMaterial>>,
) {
    // Example data for the storage buffer
    let color_data: Vec<[f32; 4]> = vec![
        [1.0, 0.0, 0.0, 1.0],
        [0.0, 1.0, 0.0, 1.0],
        [0.0, 0.0, 1.0, 1.0],
        [1.0, 1.0, 0.0, 1.0],
        [0.0, 1.0, 1.0, 1.0],
    ];

    let colors = buffers.add(ShaderStorageBuffer::from(color_data));

    // Create the custom material with the storage buffer
    let custom_material = CustomMaterial { colors };

    materials.add(custom_material);
}
```

By declaring `Handle<ShaderStorageBuffer>` on the material using `AsBindGroup`, this buffer can now be accessed in the shader:

```wgsl
@group(2) @binding(0) var<storage, read> colors: array<vec4<f32>, 5>;
```

Storage buffers are especially useful for sharing data between different passes, like generating data in a compute shader that is later used in a material. This allows for flexible workflows, such as updating procedural data on the GPU or using compute-generated values in rendering passes, all without needing to manually manage complex buffer bindings or data upload.
