#### Serialization with registry context

[`bevy_reflect`] provides a way to easily serialize and deserialize nearly any type that
implement [`Reflect`]. It does so by relying purely on the reflection APIs and the [`TypeRegistry`],
without having to know the type at compile-time.

However, sometimes serialization/deserialization for a type requires more explicit control.
In such cases, a custom `Serialize`/`Deserialize` implementation can be provided by registering
the [`ReflectSerialize`]/[`ReflectDeserialize`] type data for the type in the `TypeRegistry`.

This approach generally works well enough for most cases.
However, sometimes you want to handle the case for your type alone and continue using reflection
for the rest of the fields.
For example, you might want to serialize your type as a map that includes a few extra entries,
but you still want to use the reflection serializer for each value.

Unfortunately, not only does this not nest well within serializers,
but it also means you need to manually capture a reference to the `TypeRegistry` 
so that you can pass it down to the nested reflection serializers.
What this basically means is that you can't use custom logic along with reflection-based serialization.

Thankfully, Bevy 0.15 introduces the [`SerializeWithRegistry`] and [`DeserializeWithRegistry`] traits,
which work much like `Serialize` and `Deserialize` but with an additional `TypeRegistry` parameter.
This allows you to perform your custom logic while still being able to continue using reflection
for the rest.

```rust
impl SerializeWithRegistry for MyType {
    fn serialize<S>(&self, serializer: S, registry: &TypeRegistry) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let state = serializer.serialize_map(None)?;

        // ...custom logic...

        state.serialize_entry(
            "data",
            // Continue using reflection-based serialization
            &ReflectSerializer::new(
                self.data,
                registry,
            ),
        )?;

        state.end()
    }
}
```

With your custom serialization and deserialization logic in place, you can then register the 
[`ReflectSerializeWithRegistry`] and [`ReflectDeserializeWithRegistry`] type data for your type
to have the reflection serializer/deserializer make use of your custom logic for all instances
of your type.

#### Reflect de/serializer processors

Alongside `SerializeWithRegistry` and `DeserializeWithRegistry`, a new tool has been added for users
who use the reflect machinery for de/serialization. When using the `ReflectSerializer` or
`ReflectDeserializer`, you can now use `with_processor` and pass in a *de/serializer processor*.
This processor allows you to override the de/serialization logic for specific values and specific
types, while also capturing any context you might need inside the processor itself.

The motivating example for this is being able to deserialize `Handle<T>`s properly inside an asset
loader when reflect-deserializing. Let's imagine that we have an asset that looks like this:

```rust
#[derive(Debug, Clone, Reflect)]
struct AnimationGraph {
    nodes: Vec<Box<dyn AnimationNode>>,
}

trait AnimationNode: Send + Sync + Reflect { /* .. */ }

#[derive(Debug, Clone, Reflect)]
struct ClipNode {
    clip: Handle<AnimationClip>
}

impl AnimationNode for ClipNode { /* .. */ }

#[derive(Debug, Clone, Reflect)]
struct AdjustSpeedNode {
    speed_multiplier: f32,
}

impl AnimationNode for AdjustSpeedNode { /* .. */ }
```

```ron
(
    animation_graph: (
        nodes: [
            {
                "my_app::animation::node::ClipNode": (
                    clip: "animations/run.anim.ron",
                )
            },
            {
                "my_app::animation::node::AdjustSpeedNode": (
                    speed_multiplier: 1.5,
                )
            }
        ]
    )
)
```

When we write an `AssetLoader` for this `AnimationGraph`, we have access to a `&mut LoadContext`
which we can use to start new asset load operations, and get a `Handle` to that asset. We can also
use the existing `ReflectDeserializer` to deserialize `Box<dyn AnimationNode>`s. However, when the
deserializer encounters a `Handle<AnimationClip>`, this will be deserialized as `Handle::default`
and no asset load will be kicked off, making the handle useless.

With a [`ReflectDeserializerProcessor`], we can pass in a processor which captures the
`&mut LoadContext` and, if it encounters a `Handle<T>`, it will kick off an asset load for `T`,
and assigns the result of that load to the field it's deserializing.

```rust
struct HandleProcessor<'a> {
    load_context: &'a mut LoadContext,
}

impl ReflectDeserializerProcessor for HandleProcessor<'_> {
    fn try_deserialize<'de, D>(
        &mut self,
        registration: &TypeRegistration,
        _registry: &TypeRegistry,
        deserializer: D,
    ) -> Result<Result<Box<dyn PartialReflect>, D>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let Some(reflect_handle) = registration.data::<ReflectHandle>() else {
            // we don't want to deserialize this - give the deserializer back
            // and do default deserialization logic
            return Ok(Err(deserializer));
        };

        let asset_type_id = reflect_handle.asset_type_id();
        let asset_path = deserializer.deserialize_str(AssetPathVisitor)?;

        let handle: Handle<LoadedUntypedAsset> = self.load_context
            .loader()
            .with_dynamic_type(asset_type_id)
            .load(asset_path);
        Ok(Box::new(handle))
    }
}
```

Combined with [`ReflectSerializerProcessor`], this can be used to round-trip `Handle`s to/from string
asset paths.

The processors take priority over all other serde logic, including `De/SerializeWithRegistry`, so it
can be used to override any reflect serialization logic.


[`bevy_reflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/
[`Reflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.Reflect.html
[`TypeRegistry`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/struct.TypeRegistry.html
[`ReflectSerialize`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.ReflectSerialize.html
[`ReflectDeserialize`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.ReflectDeserialize.html
[`SerializeWithRegistry`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/serde/trait.SerializeWithRegistry.html
[`DeserializeWithRegistry`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/serde/trait.DeserializeWithRegistry.html
[`ReflectSerializeWithRegistry`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/serde/trait.ReflectSerializeWithRegistry.html
[`ReflectDeserializeWithRegistry`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/serde/trait.ReflectDeserializeWithRegistry.html
[`ReflectDeserializerProcessor`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/serde/trait.ReflectDeserializerProcessor.html
[`ReflectSerializerProcessor`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/serde/trait.ReflectSerializerProcessor.html