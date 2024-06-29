The UV-mapping of `Triangle2d` has changed with this PR: the main difference is that the UVs are no longer dependent on the triangle’s absolute coordinates but instead follow translations of the triangle itself in its definition. If you depended on the old UV-coordinates for `Triangle2d`, then you will have to update affected areas to use the new ones which can be briefly described as follows:

- The first coordinate is parallel to the line between the first two vertices of the triangle.
- The second coordinate is orthogonal to this, pointing in the direction of the third point.

Generally speaking, this means that the first two points will have coordinates `[_, 0.]`, while the third coordinate will be `[_, 1.]`, with the exact values depending on the position of the third point relative to the first two. For acute triangles, the first two vertices always have UV-coordinates `[0., 0.]` and `[1., 0.]` respectively. For obtuse triangles, the third point will have coordinate `[0., 1.]` or `[1., 1.]`, with the coordinate of one of the two other points shifting to maintain proportionality.

For example: 

- The default `Triangle2d` has UV-coordinates `[0., 0.]`, `[0., 1.]`, [`0.5, 1.]`.
- The triangle with vertices `vec2(0., 0.)`, `vec2(1., 0.)`, `vec2(2., 1.)` has UV-coordinates `[0., 0.]`, `[0.5, 0.]`, `[1., 1.]`.
- The triangle with vertices `vec2(0., 0.)`, `vec2(1., 0.)`, `vec2(-2., 1.)` has UV-coordinates `[2./3., 0.]`, `[1., 0.]`, `[0., 1.]`.
