This release of bevy slightly tweaked the definitions of `EaseFunction::ExponentialIn`, `EaseFunction::ExponentialOut`, and `EaseFunction::ExponentialInOut`.  The previous definitions had small discontinuities, while the new ones are slightly rescaled to be continuous.  For the output values that changed, that change was less than 0.001, so visually you might not even notice the difference.

However, if you depended on them for determinism, you’ll need to define your own curves with the previous definitions.
