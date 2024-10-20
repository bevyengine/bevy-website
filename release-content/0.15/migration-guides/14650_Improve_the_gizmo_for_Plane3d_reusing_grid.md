The optional builder methods on 

```rust

gizmos.primitive_3d(&Plane3d { }, ...);

```

changed from

- `segment_length`
- `segment_count`
- `axis_count`

to 

- `cell_count`
- `spacing`
