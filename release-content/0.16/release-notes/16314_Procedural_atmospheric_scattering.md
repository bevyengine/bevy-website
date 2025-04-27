<!-- Procedural atmospheric scattering -->
<!-- https://github.com/bevyengine/bevy/pull/16314 -->
**Bevy 0.16** introduces procedural atmospheric scattering, a customizable system for simulating sunsets, sunrises, and dynamic day/night cycles in real time:

<video controls loop aria-label="A showcase of simulated sunrises and sunsets made in Bevy"><source src="atmosphere-showcase.mp4" type="video/mp4"/></video>
Credit to `@aevyrie` for their amazing [atmosphere showcase]! It uses a fancy custom exposure curve to accentuate the near-dusk colors.

Enabling atmosphere rendering is simple, just add the new [`Atmosphere`] component to your camera!

```rs
commands.spawn((
    Camera3d::default(),
    Atmosphere::EARTH,
));

// the atmosphere will consider all directional lights in the scene as "suns"
commands.spawn(DirectionalLight::default());
```

When it is enabled, the primary Bevy skybox is overlaid with one that updates in real-time based on the directional lights in the scene, and the default distance fog is replaced with one that takes into account directional lights and the other atmosphere parameters. Distant objects will fade to blue on a clear day, and will be tinged orange and pink at sunset! Also, because the atmosphere is composited on *top* of the skybox, creating a nighttime starscape is easy ... just spawn the skybox and it'll naturally fade away as the sky grows brighter at dawn.

As with most PBR techniques, it's *correct*, but it can take some tweaking to look its best. All of the atmosphere parameters can also be customized: for example, a high desert sky might exhibit less Mie scattering due to the lack of humidity. The included example at `examples/3d/atmosphere.rs` includes some recommendations for lighting and camera settings.

There's some current limitations to be aware of: the atmosphere currently doesn't affect the [`EnvironmentMapLight`] or the direct lighting from directional lights on surfaces, so reflections might not be fully accurate. We're also working on integrating atmospheric scattering with volumetric fog (see the [What's Next?](#what-s-next) section for our ambitious plans!).

Because of a number of optimizations described in Sébastien Hillaire's [EGSR 2020 paper], our implementation is super fast, and should work great even on mobile devices and WebGPU. The secret sauce is that because the atmosphere is mostly symmetric, we can precalculate most of the ray marching inner loop ahead of time.

[atmosphere showcase]: https://github.com/aevyrie/bevy/tree/atmosphere_showcase
[EGSR 2020 paper]: https://sebh.github.io/publications/egsr2020.pdf
[`EnvironmentMapLight`]: https://docs.rs/bevy/0.16/bevy/pbr/environment_map/struct.EnvironmentMapLight.html
[`Atmosphere`]: https://docs.rs/bevy/0.16/bevy/pbr/struct.Atmosphere.html
