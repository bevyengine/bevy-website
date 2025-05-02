`Quad` was deprecated in 0.13, though its replacement `Rectangle` did not provide a clear replacement for `Quad::flip`. This has been amended: now you can call `flip()` on any `StandardMaterial`.

Please note that `Quad::flip` was specifically _horizontal_ flipping, though `StandardMaterial::flip()` supports both _vertical_ and _horizontal_ flipping.
