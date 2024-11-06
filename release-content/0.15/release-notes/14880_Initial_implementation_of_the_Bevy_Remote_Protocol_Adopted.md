The Bevy Remote Protocol allows the ECS of a running
Bevy application to be interacted with remotely. This can be used, for example,
to inspect and edit entities and their components at runtime. We anticipate 
that this will be used primarily to create things like inspectors for editing
and debugging.

For now, you can use BRP to:
- getting the serialized values of a set of components from an entity;
- performing a query for all entities matching a set of components and retrieving
  their associated values;
- creating a new entity with a given set of component values;
- despawning an entity;
- inserting a set of components into an entity;
- removing a set of components from an entity;
- reparenting one or more entities;
- listing the components registered in the ECS or present on an entity.

Details on these methods are available in the `bevy_remote` module documentation.
Additionally, custom methods can also be written.

The functionality itself is split up between plugins; the `RemotePlugin` handles 
the processing of remote requests and is separate from the transport.  An HTTP 
transport is provided by default by the `RemoteHttpPlugin`.

```rust
// Minimal app setup required to use BRP over HTTP
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // process remote requests:
        .add_plugins(RemotePlugin::default())
        // accept remote requests over HTTP:
        .add_plugins(RemoteHttpPlugin::default())
        .run();
}
```

Sample request:
```json
{
    "method": "bevy/get",
    "id": 0,
    "params": {
        "entity": 4294967298,
        "components": [
            "bevy_transform::components::transform::Transform"
        ]
    }
}
```

Sample response:
```json
{
    "jsonrpc": "2.0",
    "id": 0,
    "result": {
        "bevy_transform::components::transform::Transform": {
            "rotation": { "x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0 },
            "scale": { "x": 1.0, "y": 1.0, "z": 1.0 },
            "translation": { "x": 0.0, "y": 0.5, "z": 0.0 }
        }
    }
}
```
