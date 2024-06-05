Identity state transitions are a niche feature that's been missing in previous implementations.

Starting now, `StateEventTransition` events will include transitions from a state to itself!
(Identity transitions will also propagate to all dependent `ComputedStates` and `SubStates`)

Because it is a niche feature, `OnExit` and `OnEnter` schedules will ignore the new identity transitions by default,
but you can visit the new `custom_transitions` example to see how you can bypass or change that behavior!
