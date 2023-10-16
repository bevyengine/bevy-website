+++
title = "Bevy 0.12"
date = 2023-10-21
[extra]
author = "Bevy Contributors"
+++

Thanks to **X** contributors, **X** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.12** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.12**, check out our [0.11 to 0.12 Migration Guide](/learn/migration-guides/0.11-0.12/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

## Feature Name

<div class="release-feature-authors">authors: @author</div>

## Reflect Commands

<div class="release-feature-authors">authors: @NoahShomette</div>

It is now possible to insert and remove reflect components from an entity in a normal system via new functions on [`Commands`]!

```rust
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
struct Component(u32);

fn reflect_commands(mut commands: Commands) {
    let boxed_reflect_component: Box<dyn Reflect> = Box::new(Component(916));

    let entity = commands
        .spawn_empty()
        .insert_reflect(boxed_reflect_component.clone_value()).id();

    commands.entity(entity).remove_reflect(boxed_reflect_component.type_name().to_owned());

}
```

The above commands use the [`AppTypeRegistry`] by default. If you use a different TypeRegistry then you can use the ...`with_registry` commands instead.

```rust
 #[derive(Resource)]
 struct TypeRegistryResource {
     type_registry: TypeRegistry,
 }

 impl AsRef<TypeRegistry> for TypeRegistryResource {
     fn as_ref(&self) -> &TypeRegistry {
         &self.type_registry
     }
 }

 fn reflect_commands_with_registry(mut commands: Commands) {
    let boxed_reflect_component: Box<dyn Reflect> = Box::new(Component(916));

    let entity = commands
        .spawn_empty()
        .insert_reflect_with_registry::<TypeRegistryResource>(boxed_reflect_component.clone_value()).id();

    commands.entity(entity).remove_reflect_with_registry::<TypeRegistryResource>(boxed_reflect_component.type_name().to_owned());

}
```

See [`ReflectCommandExt`] for more examples and documentation


[`Commands`]: https://docs.rs/bevy/0.12.0/bevy/ecs/system/struct.Commands.html

[`AppTypeRegistry`]: https://docs.rs/bevy/0.12.0/bevy/ecs/reflect/struct.AppTypeRegistry.html

[`ReflectCommandExt`]: https://docs.rs/bevy/0.12.0/bevy/ecs/reflect/trait.ReflectCommandExt.html


## <a name="what-s-next"></a>What's Next?

We have plenty of work that is pretty much finished and is therefore very likely to land in **Bevy 0.13**:

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the X contributors that made this release (and associated docs) possible! In random order:
