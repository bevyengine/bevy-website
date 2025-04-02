If you were queuing the structs of hierarchy-related commands directly, you will need to change them to methods implemented on `EntityCommands`:

| Struct                                                       | Method                                                                                         |
|--------------------------------------------------------------|------------------------------------------------------------------------------------------------|
| `commands.queue(AddChild { child, parent })`                 | `commands.entity(parent).add_child(child)` OR `commands.entity(child).insert(ChildOf(parent))` |
| `commands.queue(AddChildren { children, parent })`           | `commands.entity(parent).add_children(children)`                                               |
| `commands.queue(InsertChildren { children, parent, index })` | `commands.entity(parent).insert_children(index, children)`                                     |
| `commands.queue(RemoveChildren { children, parent })`        | `commands.entity(parent).remove_children(children)`                                            |
| `commands.queue(ReplaceChildren { children, parent })`       | `commands.entity(parent).replace_children(children)`                                           |
| `commands.queue(ClearChildren { parent })`                   | `commands.entity(parent).remove::<Children>()`                                                 |
| `commands.queue(RemoveParent { child })`                     | `commands.entity(child).remove::<ChildOf>()`                                                   |
| `commands.queue(DespawnRecursive { entity, warn: true })`    | `commands.entity(entity).despawn()`                                                            |
| `commands.queue(DespawnRecursive { entity, warn: false })`   | `commands.entity(entity).try_despawn()`                                                        |
| `commands.queue(DespawnChildrenRecursive { entity, warn })`  | `commands.entity(entity).despawn_related::<Children>()`                                        |

If you were queuing the structs of event-related commands directly, you will need to change them to methods implemented on `Commands`:

| Struct                                            | Method                                     |
|---------------------------------------------------|--------------------------------------------|
| `commands.queue(SendEvent { event })`             | `commands.send_event(event)`               |
| `commands.queue(TriggerEvent { event, targets })` | `commands.trigger_targets(event, targets)` |
