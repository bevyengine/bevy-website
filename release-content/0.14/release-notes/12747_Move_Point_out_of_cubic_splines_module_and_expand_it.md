Linear algebra is used everywhere in games, and we want to make sure it's easy to get right. That's why we've added a new `VectorSpace` trait, as part of our work to make `bevy_math` more general, expressive, and mathematically sound. Anything that implements `VectorSpace` behaves like a vector. More formally, the trait requires that implementations satisfy the vector space axioms for vector addition and scalar multiplication. We've also added a `NormedVectorSpace` trait, which includes an api for distance and magnitude.

These traits underpin the new curve and shape sampling apis. `VectorSpace` is implemented for `f32`, the `glam` vector types, and several of the new color-space types. It completely replaces `bevy_math::Point`.

The splines module in bevy has been lacking some features for a long time. Splines are extremely useful in game development, so improving them would improve everything that uses them.

The biggest addition is NURBS support! It is a variant of a B-Spline with much more parameters that can be tweaked to create specific curve shapes. We also added a `LinearSpline`, which can be used to put straight line segments in a curve. `CubicCurve` now acts as a sequence of curve segments to which you can add new pieces, so you can mix various spline types together to form a single path.
