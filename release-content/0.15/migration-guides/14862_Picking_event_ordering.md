For users switching from `bevy_mod_picking` to `bevy_picking`:

- Instead of adding an `On<T>` component, use `.observe(|trigger: Trigger<T>|)`. You may now apply multiple handlers to the same entity using this command.
- Note that you need to add the non default `MeshPickingPlugin` if you're using picking on meshes.
- Pointer interaction events now have semi-deterministic ordering which (more or less) aligns with the order of the raw input stream. Consult the docs on `bevy_picking::event::pointer_events` for current information. You may need to adjust your event handling logic accordingly.
- `PointerCancel` has been replaced with `Pointer<Canceled>`, which now has the semantics of an OS touch pointer cancel event.
- `InputMove` and `InputPress` have been merged into `PointerInput`. The use remains exactly the same.
- Picking interaction events are now only accessible through observers, and no `EventReader`. This functionality may be re-implemented later.

For users of `bevy_winit`:

- The event `bevy_winit::WinitEvent` has moved to `bevy_window::WindowEvent`. If this was the only thing you depended on `bevy_winit` for, you should switch your dependency to `bevy_window`.
- `bevy_window` now depends on `bevy_input`. The dependencies of `bevy_input` are a subset of the existing dependencies for `bevy_window` so this should be non-breaking.
