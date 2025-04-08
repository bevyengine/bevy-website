<!-- Procedural atmospheric scattering -->
<!-- https://github.com/bevyengine/bevy/pull/16314 -->
Bevy 0.16 introduces blazing-fast procedural atmospheric scattering, a customizable system for simulating sunsets, sunrises, and dynamic day/night cycles in real time. It also adds a form of distance fog that takes the atmosphere into account! For this release, we focused on optimizing for cameras relatively close to the ground and earth-like atmospheres, but we're hoping to expand to arbitrary atmospheres and space views soon.

```rs
commands.spawn((
    Camera3d::default(),
    // Just add the new Atmosphere component. As with most PBR techniques, it's *correct*, but can take some tweaking to look its best.
    // The included example at `examples/3d/atmosphere.rs` includes some recommendations for lighting and camera settings.
    Atmosphere::EARTH,
));

// the atmosphere will consider all directional lights in the scene as "suns"
commands.spawn(DirectionalLight::default());
```

The mathematics behind the effect are based on a complicated integral, but it boils down to solving for two quantities at every point along each camera ray: the probability that light from the sun reflects off an air molecule towards the camera, and the number of air molecules between that point and the sun. The former is easy to approximate based on some known angles, but the latter needs a secondary raycast from *every point*, so a naive implementation could be really slow!

However, because our implementation assumes that the atmosphere is mostly symmetric, we can precalculate the amount of atmosphere (the "optical depth") along any given ray based only on its altitude and angle above the horizon. This already lets us speed things up a lot, but it's possible to go faster: since the sky generally has fairly low-frequency detail, we can render the sky at a very small resolution, and then upscale to the skybox with almost no loss in detail.

See [Sébastien Hillaire's 2020 paper] for more info. There's a lot of depth to this technique, like approximated multi-scattering, that would take much more than this section to cover :slightly_smiling_face:

<video controls loop aria-label="A showcase of simulated sunrises and sunsets made in Bevy"><source src="atmosphere-showcase.mp4" type="video/mp4"/></video>
Credit to `@aevyrie` for their amazing [atmosphere showcase]! It uses a fancy custom exposure curve to accentuate the near-dusk colors.

[atmosphere showcase]: https://github.com/aevyrie/bevy/tree/atmosphere_showcase
[Sébastien Hillaire's 2020 paper]: https://sebh.github.io/publications/egsr2020.pdf

<!-- TODO -->
