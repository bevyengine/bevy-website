Bevy boasts a powerful [reflection] system that allows you to introspect and build types at runtime.
It works by passing around data as [`Reflect`] trait objects like `Box<dyn Reflect>`.
This has the effect of erasing the compile-time type information,
allowing data to be stored and moved around without having to know the exact type
behind the trait object.

Because of this type erasure, `bevy_reflect` can also get away with some interesting tricks.
For instance, there are many cases where a type needs to be built up field-by-field,
such as during deserialization.
This works fine when you know the type at compile-time, but it becomes very challenging to do
at runtime.
To solve this, `bevy_reflect` has a concept of _dynamic_ types.

Dynamic types exist as a way to dynamically construct and store reflected data in a way
that appears like a concrete type.
Behind the scenes, `bevy_reflect` uses these types to build up a representation of the target type.
And it can do so since we hide the actual type behind the `dyn Reflect` trait object.

Unfortunately, this comes with a very common issue: it becomes very easy to accidentally
believe a `dyn Reflect` is a concrete type when it's actually a dynamic type representing
that concrete type.

To address this problem, Bevy 0.15 has reworked the `Reflect` trait based on the [Unique Reflect RFC].
This splits it into two separate traits: `Reflect` and [`PartialReflect`].

`PartialReflect` is much like the `Reflect` trait of previous versions.
It allows access to fundamental reflection capabilities and allows for type-erasure behind
a `dyn PartialReflect` trait object.
It allows for both concrete types and dynamic types to be used interchangeably.

`Reflect`, on the other hand, has become a much stricter trait.
It's a subset of `PartialReflect` that guarantees the underlying type beneath the trait object
is exactly the concrete type it says it is.

This split allows reflection-based APIs and user code to be more explicit about the dynamic-ness of
the trait objects they're working with. It moves the knowledge of whether a type is dynamic or not
to compile-time, preventing many common pitfalls of working with dynamic types,
including knowing when they need to be handled separately.

[reflection]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/
[`Reflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.Reflect.html
[`PartialReflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.PartialReflect.html
[Unique Reflect RFC]: https://github.com/bevyengine/rfcs/pull/56
