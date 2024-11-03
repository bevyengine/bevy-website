The `DynamicScene` format is changed to use custom serialize impls so old scene files will need updating:

Old: 

```ron
(
  resources: {},
  entities: {
    4294967299: (
      components: {
        "bevy_render::camera::projection::OrthographicProjection": (
          near: 0.0,
          far: 1000.0,
          viewport_origin: (
            x: 0.5,
            y: 0.5,
          ),
          scaling_mode: WindowSize(1.0),
          scale: 1.0,
          area: (
            min: (
              x: -1.0,
              y: -1.0,
            ),
            max: (
              x: 1.0,
              y: 1.0,
            ),
          ),
        ),
      },
    ),
  },
)
```

New:

```ron
(
  resources: {},
  entities: {
    4294967299: (
      components: {
        "bevy_render::camera::projection::OrthographicProjection": (
          near: 0.0,
          far: 1000.0,
          viewport_origin: (0.5, 0.5),
          scaling_mode: WindowSize(1.0),
          scale: 1.0,
          area: (
            min: (-1.0, -1.0),
            max: (1.0, 1.0),
          ),
        ),
      },
    ),
  },
)
```
