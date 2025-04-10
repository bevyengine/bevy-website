Transforms in Bevy (and other 3D software) come in two flavors:

1. Global transforms ([`GlobalTransform`]), which represent the absolute world-space position, scale and rotation of an object.
2. Local transforms ([`Transform`]), which represent the position, scale and rotation of an object relative to its parent.

In order to compute the global transform of each object (which is what rendering and physics care about!),
we need to recursively combine the local transforms of all of our objects down the parent-child hierarchy.
This process, known as transform propagation, has always been one of the most computationally intensive operations in most Bevy applications.
As it turns out, most entities in your game are going to have a transform!

This made transform propagation a prime candidate for some serious optimization work, and Bevy 0.16 comes with *two* impressive performance optimizations, stolen shamelessly from the [`big_space`] crate by the same author.

The first optimization improves our parallelization strategies. While we were already splitting the work across threads,
better work sharing, parallelization across trees and a leaf vs non-leaf split to optimize cache coherency made a huge difference.

The second optimization focuses on saving work for trees where none of the objects have moved.
In many cases, this is the overwhelming majority of objects: level geometry and props are not typically moving around each frame!
We're now propagating a "dirty bit" up the hierarchy towards ancestors; allowing transform propagation to ignore entire subtrees of the hierarchy if they encounter an entity without the dirty bit.

The results speak for themselves: taken together, our testing on the huge (127,515 objects) [Caldera Hotel] scene  from Call of Duty: Warzone shows that transform propagation took 1.1 ms in 0.15 on an M4 Max Macbook, and 0.1 ms after these changes in 0.16.
Even fully dynamic scenes (like our [`many_foxes`] stress test) are substantially faster due to the improved parallelism. Nice!
This work matters even more for more typical hardware: on large scenes on mid or low-end hardware transform propagation could eat an entire 25% of the frame budget. Ouch!

While that's an impressive 11x performance improvement, the absolute magnitude of the time saved is the key metric.
With about 16 ms per frame at 60 FPS, that's 6% of your *entire* game's CPU budget saved. making huge open worlds or incredibly complex CAD assemblies more viable than ever before.

![A screenshot of a `tracy` histogram showing the effects of these changes on Caldera. 0.15 peaks at 1.1 ms, while 0.16 peaks at 0.1 ms.][caldera-transform-propagation-bench]

If you're interested in the gory technical details of these optimizations, take a look at [the code itself].
It's incredibly well-commented and great to learn from.

[Caldera Hotel]: https://github.com/Activision/caldera
[the code itself]: https://github.com/bevyengine/bevy/blob/b0c446739888705d3e95b640e9d13e0f1f53f06d/crates/bevy_transform/src/systems.rs#L12
[caldera-transform-propagation-bench]: caldera-transform-propagation-bench.png
[`many_foxes`]: https://github.com/bevyengine/bevy/blob/main/examples/stress_tests/many_foxes.rs
[`big_space`]: https://github.com/aevyrie/big_space
