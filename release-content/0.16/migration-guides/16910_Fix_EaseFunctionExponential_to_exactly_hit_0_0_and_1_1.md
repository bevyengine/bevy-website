`EaseFunction::ExponentialIn`, `EaseFunction::ExponentialOut`, and `EaseFunction::ExponentialInOut` has slight discontinuities in 0.15, leading to [jumping behavior at the start and end of the function][jumping behavior]. In 0.16, these functions have been slightly adjusted so that they are continuous.

The new functions differ from the old by less than 0.001, so in most cases this change is not breaking. If, however, you depend on these easing functions for determinism, you will need to define custom curves using the previous functions.

[jumping behavior]: https://github.com/bevyengine/bevy/issues/16676
