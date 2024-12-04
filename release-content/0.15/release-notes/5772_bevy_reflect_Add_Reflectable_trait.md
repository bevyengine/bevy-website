[`bevy_reflect`] is powered by many different traits working together
to provide the full reflection API. These include traits like [`Reflect`], but also other traits
like [`TypePath`], [`Typed`], and [`GetTypeRegistration`].

This can make adding the right bounds on generic parameters a bit confusing,
and it's easy to forget to include one of these traits.

To make this simpler, 0.15 introduces the [`Reflectable`] trait. All the traits listed above are
supertraits of `Reflectable`, allowing it to be used in place of all of them where necessary.

[`bevy_reflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/
[`Reflect`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.Reflect.html
[`TypePath`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.TypePath.html
[`Typed`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.Typed.html
[`GetTypeRegistration`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.GetTypeRegistration.html
[`Reflectable`]: https://docs.rs/bevy_reflect/0.15/bevy_reflect/trait.Reflectable.html
