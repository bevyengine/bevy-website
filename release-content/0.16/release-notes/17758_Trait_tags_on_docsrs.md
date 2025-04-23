<!-- Trait tags on docs.rs -->
<!-- https://github.com/bevyengine/bevy/pull/17758 -->

Being a framework, Bevy provides several traits that define how a type is used – for example, to attach data to an entity, it must implement `Component`. When reading the docs, that meant scrolling down to the type's trait implementation section and searching for relevant traits. With 0.16 however, on docs.rs, Bevy displays labels indicating which relevant a type implements:
![Rustdoc showing a "Component" label below "Camera" type](trait-tags.png)

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
