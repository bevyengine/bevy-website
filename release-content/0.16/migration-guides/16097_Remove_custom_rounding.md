`UiSurface::get_layout` now also returns the final sizes before rounding. Call `.0` on the `Ok` result to get the previously returned `taffy::Layout` value.
