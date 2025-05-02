Users have sometimes asked for us to trigger exit and entry steps when moving from a state to itself.
While this has its uses (refreshing is the core idea), it can be surprising and unwanted in other cases.
We've found a compromise that lets users hook into this type of transition if it's something they need.

`StateEventTransition` events will now include transitions from a state to itself,
which will also propagate to all dependent `ComputedStates` and `SubStates`.

Because it is a niche feature, `OnExit` and `OnEnter` schedules will ignore the new identity transitions by default,
but you can visit the new [`custom_transitions`](https://github.com/bevyengine/bevy/tree/v0.14.0/examples/state/custom_transitions.rs) example to see how you can bypass or change that behavior!
