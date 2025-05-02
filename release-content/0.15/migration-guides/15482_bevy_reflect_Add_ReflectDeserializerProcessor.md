(Since I added `P = ()`, I don’t think this is actually a breaking change anymore, but I’ll leave this in)

`bevy_reflect`’s `ReflectDeserializer` and `TypedReflectDeserializer` now take a `ReflectDeserializerProcessor` as the type parameter `P`, which allows you to customize deserialization for specific types when they are found. However, the rest of the API surface (`new`) remains the same.
<details>
<summary>Original implementation</summary>

Add `ReflectDeserializerProcessor`:

```rs
struct ReflectDeserializerProcessor {
    pub can_deserialize: Box<dyn FnMut(&TypeRegistration) -> bool + 'p>,
    pub deserialize: Box<
        dyn FnMut(
                &TypeRegistration,
                &mut dyn erased_serde::Deserializer,
            ) -> Result<Box<dyn PartialReflect>, erased_serde::Error>
            + 'p,
}
```

Along with `ReflectDeserializer::new_with_processor` and `TypedReflectDeserializer::new_with_processor`. This does not touch the public API of the existing `new` fns.

This is stored as an `Option<&mut ReflectDeserializerProcessor>` on the deserializer and any of the private `-Visitor` structs, and when we attempt to deserialize a value, we first pass it through this processor.

Also added a very comprehensive doc test to `ReflectDeserializerProcessor`, which is actually a scaled down version of the code for the `bevy_animation_graph` loader. This should give users a good motivating example for when and why to use this feature.

__Why `Box<dyn ..>`?__

When I originally implemented this, I added a type parameter to `ReflectDeserializer` to determine the processor used, with `()` being “no processor”. However when using this, I kept running into rustc errors where it failed to validate certain type bounds and led to overflows. I then switched to a dynamic dispatch approach.

The dynamic dispatch should not be that expensive, nor should it be a performance regression, since it’s only used if there is `Some` processor. (Note: I have not benchmarked this, I am just speculating.) Also, it means that we don’t infect the rest of the code with an extra type parameter, which is nicer to maintain.

__Why the `'p` on `ReflectDeserializerProcessor<'p>`?__

Without a lifetime here, the `Box`es would automatically become `Box<dyn FnMut(..) + 'static>`. This makes them practically useless, since any local data you would want to pass in must then be `'static`. In the motivating example, you couldn’t pass in that `&mut LoadContext` to the function.

This means that the `'p` infects the rest of the Visitor types, but this is acceptable IMO. This PR also elides the lifetimes in the `impl<'de> Visitor<'de> for -Visitor` blocks where possible.

__Future possibilities__

I think it’s technically possible to turn the processor into a trait, and make the deserializers generic over that trait. This would also open the door to an API like:

```rs
type Seed;

fn seed_deserialize(&mut self, r: &TypeRegistration) -> Option<Self::Seed>;

fn deserialize(&mut self, r: &TypeRegistration, d: &mut dyn erased_serde::Deserializer, s: Self::Seed) -> ...;
```

A similar processor system should also be added to the serialization side, but that’s for another PR. Ideally, both PRs will be in the same release, since one isn’t very useful without the other.
</details>