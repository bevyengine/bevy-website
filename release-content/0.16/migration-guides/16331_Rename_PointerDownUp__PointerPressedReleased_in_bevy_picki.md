#### `bevy_picking/src/pointer.rs`

__`enum PressDirection`:__
- `PressDirection::Down` changes to `PressDirection::Pressed`.
- `PressDirection::Up` changes to `PressDirection::Released`.

These changes are also relevant when working with `enum PointerAction`

#### `bevy_picking/src/events.rs`

Clicking and pressing Events in events.rs categories change from [Down], [Up], [Click] to [Pressed], [Released], [Click].

- `struct Down` changes to `struct Pressed` - fires when a pointer button is pressed over the ‘target’ entity.
- `struct Up` changes to `struct Released` - fires when a pointer button is released over the ‘target’ entity.
- `struct Click` now fires when a pointer sends a Pressed event followed by a Released event on the same ‘target’.
- `struct DragStart` now fires when the ‘target’ entity receives a pointer Pressed event followed by a pointer Move event.
- `struct DragEnd` now fires when the ‘target’ entity is being dragged and receives a pointer Released event.
- `PickingEventWriters<'w>::down_events: EventWriter<'w, Pointer<Down>>` changes to `PickingEventWriters<'w>::pressed_events: EventWriter<'w, Pointer<Pressed>>`.
- `PickingEventWriters<'w>::up_events changes to PickingEventWriters<'w>::released_events`.
