Before:

```rust
commands.spawn(UiImage::new(image));
```

After:

```rust
commands.spawn(ImageNode::new(image));
```
