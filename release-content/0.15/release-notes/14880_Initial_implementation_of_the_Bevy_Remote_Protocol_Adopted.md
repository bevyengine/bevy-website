The Bevy Remote Protocol allows the ECS of a running
Bevy application to be interacted with remotely. This can be used, for example,
to inspect and edit entities and their components at runtime. We anticipate 
that this will be used to create things like inspectors which monitor the
content of the ECS from a separate process; the Bevy editor *may* also use
such an architecture eventually, although this is still highly experimental.

For now, you can use BRP to:
- get the serialized values of a set of components from an entity;
- perform a query for all entities matching a set of components and retrieving
  their associated values;
- create a new entity with a given set of component values;
- despawn an entity;
- insert a set of components into an entity;
- remove a set of components from an entity;
- reparent one or more entities;
- list the components registered in the ECS or present on an entity.

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
