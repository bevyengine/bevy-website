<!-- Dedicated `Reflect` implementation for `Set`-like things -->
<!-- https://github.com/bevyengine/bevy/pull/13014 -->

Inside of `bevy_reflect`, every reflected Rust object ends up being mapped to one of a handful of [`ReflectKind`] variants.

Before Bevy 0.15, sets (like [`HashSet`]) were treated as opaque "values": there was no way to view or modify their contents via reflection.
With these changes, we can now properly represent sets of all kinds, which is particularly handy for runtime debugging tools like [`bevy-inspector-egui`]!

[`ReflectKind`]: https://docs.rs/bevy/0.15.0/bevy/reflect/enum.ReflectKind.html
[`HashSet`]: https://doc.rust-lang.org/stable/std/collections/struct.HashSet.html
[`bevy-inspector-egui`]: https://github.com/jakobhellermann/bevy-inspector-egui
