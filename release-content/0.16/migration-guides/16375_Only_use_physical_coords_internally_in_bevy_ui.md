`ComputedNode`’s fields and methods now use physical coordinates.
`ComputedNode` has a new field `inverse_scale_factor`. Multiplying the physical coordinates by the `inverse_scale_factor` will give the logical values.
