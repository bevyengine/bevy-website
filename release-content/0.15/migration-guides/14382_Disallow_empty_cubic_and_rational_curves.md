The `to_curve` method on Bevy’s cubic splines is now fallible (returning a `Result`), meaning that any existing calls will need to be updated by handling the possibility of an error variant. 

Similarly, any custom implementation of `CubicGenerator` or `RationalGenerator` will need to be amended to include an `Error` type and be made fallible itself. 

Finally, the fields of `CubicCurve` and `RationalCurve` are now private, so any direct constructions of these structs from segments will need to be replaced with the new `CubicCurve::from_segments` and `RationalCurve::from_segments` methods.
