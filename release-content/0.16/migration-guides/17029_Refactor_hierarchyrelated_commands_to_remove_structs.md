If you were queuing the structs of hierarchy-related commands or `SendEvent` directly, you will need to change them to the methods implemented on `EntityCommands` (or `Commands` for `SendEvent`):

Struct|Method
------|------
`commands.queue(AddChild { child, parent });`|`commands.entity(parent).add_child(child);` OR `commands.entity(child).set_parent(parent);`
`commands.queue(AddChildren { children, parent });`|`commands.entity(parent).add_children(children);`
`commands.queue(InsertChildren { children, parent });`|`commands.entity(parent).insert_children(children);`
`commands.queue(RemoveChildren { children, parent });`|`commands.entity(parent).remove_children(children);`
`commands.queue(ReplaceChildren { children, parent });`|`commands.entity(parent).replace_children(children);`
`commands.queue(ClearChildren { parent });`|`commands.entity(parent).clear_children();`
`commands.queue(RemoveParent { child });`|`commands.entity(child).remove_parent()`
`commands.queue(DespawnRecursive { entity, warn: true });`|`commands.entity(entity).despawn_recursive();`
`commands.queue(DespawnRecursive { entity, warn: false });`|`commands.entity(entity).try_despawn_recursive();`
`commands.queue(DespawnChildrenRecursive { entity, warn: true });`|`commands.entity(entity).despawn_descendants();`
`commands.queue(DespawnChildrenRecursive { entity, warn: false});`|`commands.entity(entity).try_despawn_descendants();`
`commands.queue(SendEvent { event });`|`commands.send_event(event);`
