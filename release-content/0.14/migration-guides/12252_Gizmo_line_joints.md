Line joins have been added for gizmos, allowing for smooth or sharp corners between lines. If you manually created your own `GizmoConfig`, you will have to specify the type of line joins with the `line_joins` field.

The `Default` implementation of `GizmoLineJoint` is `None`, but you may be interested in `Miter` for sharp joints or `Round` for smooth joints.
