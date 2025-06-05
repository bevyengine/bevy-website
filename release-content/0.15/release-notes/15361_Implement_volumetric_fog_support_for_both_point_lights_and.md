Volumetric fog was [introduced in Bevy 0.14](https://bevy.org/news/bevy-0-14/#volumetric-fog-and-volumetric-lighting-light-shafts-god-rays). Initially, only directional lights could interact with it. In Bevy 0.15, point lights and spot lights work with it too:

![volumetric fog](volumetric_fog.jpg)

To add volumetric fog to your scene, add [VolumetricFog](https://docs.rs/bevy/0.15/bevy/pbr/struct.VolumetricFog.html) to the camera, and add [VolumetricLight](https://docs.rs/bevy/0.15/bevy/pbr/struct.VolumetricLight.html) to directional light, point light, or spot light that you wish to be volumetric.

```rust
// Add VolumetricFog to the camera.
commands
    .spawn((
        Camera3d::default(),
        Camera {
            hdr: true,
            ..default()
        },
    ))
    .insert(VolumetricFog {
        // This value is explicitly set to 0 since we have no environment map light.
        ambient_intensity: 0.0,
        ..default()
    });

// Add VolumetricLight to point light.
commands.spawn((
    PointLight {
        shadows_enabled: true,
        range: 150.0,
        color: RED.into(),
        intensity: 1000.0,
        ..default()
    },
    VolumetricLight,
    Transform::from_xyz(-0.4, 1.9, 1.0),
));

// Add VolumetricLight to spot light.
commands.spawn((
    SpotLight {
        intensity: 5000.0, // lumens
        color: Color::WHITE,
        shadows_enabled: true,
        inner_angle: 0.76,
        outer_angle: 0.94,
        ..default()
    },
    VolumetricLight,
    Transform::from_xyz(-1.8, 3.9, -2.7).looking_at(Vec3::ZERO, Vec3::Y),
));
```