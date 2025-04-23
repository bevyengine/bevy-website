<!-- Trait tags on docs.rs -->
<!-- https://github.com/bevyengine/bevy/pull/17758 -->

Bevy provides several core traits that define how a type is used. For example, to attach data to an entity it must implement `Component`. When reading the Rust API docs, determining whether a type is a `Component` (or some other core Bevy trait) requires scrolling through all of the docs until you find the relevant trait. But in **Bevy 0.16**, on docs.rs Bevy now displays labels indicating which core Bevy traits a type implements:

![Rustdoc showing a "Component" label below "Camera" type](trait-tags.jpg)

This happens for the traits
`Plugin` / `PluginGroup`,
`Component`,
`Resource`,
`Asset`,
`Event`,
`ScheduleLabel`,
`SystemSet`,
`SystemParam`,
`Relationship` and
`RelationshipTarget`.

This is done for now through [javascript](https://github.com/bevyengine/bevy/tree/release-0.16.0/docs-rs). This should be useful for other Rust frameworks than Bevy, and we'll work with the rustdoc team on how to make this built in and more generic. Get in touch if you're interested so we can start a specification!
