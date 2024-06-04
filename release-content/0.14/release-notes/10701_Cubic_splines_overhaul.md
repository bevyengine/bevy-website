The splines module in bevy has been lacking some features for a long time. Splines are extremely useful in game development, so improving them would improve everything that uses them.

The biggest addition is NURBS support! It is a variant of a B-Spline with much more parameters that can be tweaked to create specific curve shapes.

We also added a LinearSpline, which can be used to put straight line segments in a CubicCurve, which now acts as a sequence of cure segments, so you can mix various spline types together to form a single path.

And as a small improvement, the VectorSpace trait has been implemented and 4-dimensional Vectors. This trait is implemented for types that can be used in cubic curves, so now you have more types that can be used. For example, you can use the 4th element of a Vec4 to store tilt along the path on the curve.
