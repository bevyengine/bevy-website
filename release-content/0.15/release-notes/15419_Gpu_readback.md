The new `Readback` component simplifies the tricky process of getting data back from the GPU to the CPU using an observer based API.

```rust
commands.spawn(Readback::buffer(buffer.clone())).observe(
    |trigger: Trigger<ReadbackComplete>| {
        let data = trigger.event().to_shader_type();
        // ...
    },
);
```

When spawned into the main world the `Readback` component will queue a `Handle<Image>` or `Handle<ShaderStorageBuffer>` to be asynchronously read and copied back from the GPU to CPU in a future frame. 

This is especially useful for debugging, saving GPU-generated data, or performing CPU-side computations with results from the GPU. It’s perfect for scenarios where you need to analyze simulation data, capture rendered frames, or process large datasets on the GPU and retrieve the results for further use on the CPU.
