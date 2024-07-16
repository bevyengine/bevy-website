The asset loader's `Reader` type alias now requires the new `AsyncSeek` trait. Please implement `AsyncSeek` for any structures that must be a `Reader`, or use an alternative if seeking is not supported.

If this is a problem for you, please chime in at [bevy#12880](https://github.com/bevyengine/bevy/issues/12880) and help us improve the design for 0.15!
