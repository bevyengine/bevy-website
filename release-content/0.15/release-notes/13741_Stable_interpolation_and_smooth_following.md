When animating cameras or programming unit AI (not that kind of AI!), moving something continuously towards a target is an essential basic operation.
Simply [lerping] to the target seems easy enough, but as [Freya Holmer explains],
making sure that this interpolation is timestep independent is both vital and surprisingly tricky.

We've done the math for you; you just need to use the [`StableInterpolate`] trait's `interpolate_stable` and `smooth_nudge` methods
and tune the `decay_rate` parameter to really optimize your _game feel_.
Fear not: it even works on quaternions!
Stable, smooth camera controllers have never been easier.

[lerping]: https://en.wikipedia.org/wiki/Linear_interpolation
[Freya Holmer explains]: https://www.youtube.com/watch?v=LSNQuFEDOyQ
[`StableInterpolate`]: https://docs.rs/bevy/0.15.0-rc.3/bevy/math/trait.StableInterpolate.html