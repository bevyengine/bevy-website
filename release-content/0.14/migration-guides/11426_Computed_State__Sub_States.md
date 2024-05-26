If the user accessed the NextState resource’s value directly or created them from scratch they will need to adjust to use the new enum variants:

- if they created a `NextState(Some(S))` - they should now use `NextState::Pending(S)`
- if they created a `NextState(None)` -they should now use `NextState::Unchanged`
- if they matched on the `NextState` value, they would need to make the adjustments above

If the user manually utilized `apply_state_transition`, they should instead use systems that trigger the `StateTransition` schedule.
