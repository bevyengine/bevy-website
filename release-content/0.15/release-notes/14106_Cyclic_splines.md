<!-- Cyclic splines -->
<!-- https://github.com/bevyengine/bevy/pull/14106 -->

Most cubic spline constructions now support creating a closed loop instead of just
a path, if desired. This can be convenient for constructing things like periodic
paths for NPCs or other game entities.

The only difference is that `to_curve_cyclic` must be called in place of `to_curve`.
The supported spline constructions are:
- Hermite splines (`CubicHermite`),
- Cardinal splines (`CubicCardinalSpline`),
- B-splines (`CubicBSpline`),
- Linear splines (`LinearSpline`).

