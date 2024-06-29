`glam` has been updated from 0.25 to 0.27. Please view [the changelong] for both 0.26 and 0.27 to update your code.

[the changelog]: https://github.com/bitshifter/glam-rs/blob/e1b521a4c8146f27b97e510d38fab489c39650d1/CHANGELOG.md#0270---2024-03-23

The largest breaking change is that the `fract()` method for vector types now evaluates as `self - self.trunc()` instead of `self - self.floor()`. If you require the old behavior, use the `fract_gl()` method instead.
