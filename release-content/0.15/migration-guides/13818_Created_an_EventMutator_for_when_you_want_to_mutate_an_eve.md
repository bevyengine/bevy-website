Users currently using `ManualEventReader` should use `EventCursor` instead. `ManualEventReader` will be removed in Bevy 0.16. Additionally, `Events::get_reader` has been replaced by `Events::get_cursor`.

Users currently directly accessing the `Events` resource for mutation should move to `EventMutator` if possible.
