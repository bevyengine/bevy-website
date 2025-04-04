Bloom is that soft glow bleeding out from bright areas of light: in the real world, this is caused by your camera (or eyes) getting overwhelmed or an imperfect focus.
In games, it's a powerful artistic tool for creating everything from cyberpunk neon signs to tastefully glowing windows to juicy geometrically-inspired arcade games.

Bevy has had bloom since version 0.9, but we're giving artists another simple lever to tweak: the ability to stretch, squash and otherwise distort the effect by setting the 2-dimensional `scale` parameter on the [`Bloom`] component on your camera.

TODO: add image.

When heavily skewed (usually horizontally), this effect is known as **anamorphic bloom**.
This effect is associated with a cinematic, futuristic vibe, and emulates the unusual geometry of certain film cameras as they compress a wider image onto narrower film.
But, regardless of why it occurs, it simply looks neat!

[`Bloom`]: https://dev-docs.bevyengine.org/bevy/core_pipeline/bloom/struct.Bloom.html