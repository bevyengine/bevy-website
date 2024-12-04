The new `Readback` component simplifies the tricky process of getting data back from the GPU to the CPU using an observer-based API.

```rust
commands.spawn(Readback::buffer(buffer.clone())).observe(
    |trigger: Trigger<ReadbackComplete>| {
        let data = trigger.event().to_shader_type();
        // ...
    },
);
```

Normally, manually retrieving data from the GPU involves a lot of boilerplate and careful management of GPU resources. You have to deal with synchronization, ensure the GPU has finished processing, and handle copying data between memory spaces—which isn’t straightforward!

The new `Readback` component streamlines this process. When spawned into the main world, `Readback` will queue a `Handle<Image>` or `Handle<ShaderStorageBuffer>` to be asynchronously read and copied back from the GPU to CPU in a future frame where it will trigger a `ReadbackComplete` event containing the raw bytes of the resource.

This is especially useful for debugging, saving GPU-generated data, or performing CPU-side computations with results from the GPU. It’s perfect for scenarios where you need to analyze simulation data, capture rendered frames, or process large datasets on the GPU and retrieve the results for further use on the CPU.
