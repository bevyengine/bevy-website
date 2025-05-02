The Bevy Remote Protocol allows the ECS of a running
Bevy application to be interacted with remotely. This can be used, for example,
to inspect and edit entities and their components at runtime. We anticipate 
that this will be used to create things like inspectors which monitor the
content of the ECS from a separate process. We're planning on using BRP in the
upcoming Bevy Editor to communicate with remote Bevy apps.

Currently, you can use BRP to:

- Get the serialized values of a set of components from an entity
- Perform a query for all entities matching a set of components and retrieve the matching values
- Create a new entity with a given set of component values
- For a given entity, insert or remove a set of components
- Despawn an entity
- Reparent one or more entities
- List the components registered in the ECS or present on an entity

Here is the minimal app setup required to use BRP over HTTP:

```rust
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            // The "core" plugin, which handles remote requests provided by transports
            RemotePlugin::default(),
            // Provides remote request transport via HTTP
            RemoteHttpPlugin::default(),
        ))
        .run();
}
```

Here is a sample request:
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

And here is a sample response:
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
