+++
title = "Bevy 0.13"
date = 2024-02-03
[extra]
author = "Bevy Contributors"
image = "TODO.gif"
show_image = true
image_subtitle = "TODO"
image_subtitle_link = "TODO"

+++

Thanks to **TODO** contributors, **TODO** pull requests, community reviewers, and our [**generous sponsors**](/community/donate), we're happy to announce the **Bevy 0.13** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/book/getting-started/) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.13**, check out our [0.12 to 0.13 Migration Guide](/learn/migration-guides/0.12-0.13/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

* **First-party primitive shapes:** basic shapes are a core building block of both game engines and video games: we've added a polished collection of them for you to use!
* **Dynamic queries:** refining queries from within systems is extremely expressive, and is the last big puzzle piece for runtime-defined types and third-party modding and scripting integration.
* **Automatically inferred command flush points:** tired of reasoning about where to put `apply_deferred` and confused about why your commands weren't being applied? Us too! Now, Bevy's scheduler uses ordinary `.before` and `.after` constraints and inspects the system parameters to automatically infer (and deduplicate) synchronization points.
* **Slicing, tiling and ninepatch sprites and UI:** ninepatch layout is a popular tool for smoothly scaling stylized tilesets and UIs. Now in Bevy!
* **Lightmaps:** the first step towards baked global illumination: a fast, popular and pretty lighting technique.
* **Animation interpolation modes:** Bevy now supports non-linear interpolation modes in exported glTF animations.

## Primitive shapes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Dynamic queries

<div class="release-feature-authors">authors: @james-j-obrien, @jakobhellermann, @Suficio</div>

In bevy ECS, queries use a type DSL. The full type of the query — meaning:
what component to access, which filter to use — must be specified at compile time.

Sometimes, we don't know what the query accesses at compile time.
A UI with a dynamic list filtering by component, bindings for a scripting language,
*entity relationships* (more on this later), all of those, are impossible to accomplish without
creating queries at runtime.

They are now possible thanks to dynamic queries.

The standard way of defining a `Query` is by using them as system parameters:

```rust
fn take_damange(mut player_health: Query<(Entity, &mut Health), With<Player>>) {
    // ...
}
```

**This won't change**. And for most — if not all — gameplay use cases, you will
continue to happily use the [`Query`] API, which made bevy's reputation as a delightful game
engine.

However, now, consider this: As a developer, or game modder, I want to list entities
with a specific component, through a text prompt. Similarly to how the Quake console works.
How does that look like?

```rust
#[derive(Resource)]
struct UserQuery(String);

// user_query is entered as a text prompt by the user when the game is running.
// Using a system, we quickly find out we can't use `Query`.
fn list_entites_system(user_query: Res<UserQuery>, query: Query<TODO, With<TODO>>) {}

// using the more advanced `World` API, we are still stuck.
fn list_entities(user_query: String, world: &mut World) {
    //      What to put here? vvv
    let query = world.query::<TODO>();
}
```

It's impossible to chose a type based on the value of `user_query`!
[`QueryBuilder`] solves this problem.

```rust
fn list_entities(
    user_query: String,
    type_registry: &TypeRegistry,
    world: &mut World,
) -> Option<()> {
    let name = user_query.split(' ').next()?;
    let type_id = type_registry.get_with_short_type_path(name)?.type_id();
    let component_id = world.components().get_id(type_id)?;

    let query = QueryBuilder::<FilteredEntityRef>::new(&mut world)
        .ref_id(component_id)
        .build();

    for entity_ref in query.iter(world) {
        let ptr = entity_ref.get_by_id(component_id);
        // Convert `ptr` into a `&dyn Reflect` and use it.
    }
    Some(())
}
```

It is still an error-prone, complex, and unsafe API, but it makes something that was previously
impossible now possible. There are more ways of using `QueryBuilder`, check out
[The dynamic query pull request] for a detailed breakdown of the API.

[`QueryBuilder`] is here for people who need queries with runtime access specification,
maybe you need to:

- Add a runtime filter to [`bevy-inspector-egui`]'s entity inspector.
- Define queries in a scripting language such as Lua or JavaScript.
- Define new components from a scripting language and query them.
- Add a [Quake-style console] to modify or query components from a prompt at runtime.
- Create an [editor with remote capabilities].
- And those are only what people came up with *before* the 0.13 release!

We expect third party crates to provide convenient wrappers around the `QueryBuilder` API.


### Relations

I mentioned *entity relationships* earlier. What are relations? They are a way to associate
entities to other entities. For example, the `Parent` and `Children` components
in `bevy_hierarchy` are relations. They describe a relation between several entities.

`bevy_hierarchy` is fairly robust, but if you yourself want to create your own
relation (say, a group of units in an RTS game), the road ahead is a tar pit of footguns
and synchronization bugs.

*Entity relationships* encode relations in the ECS. They are a staple of the [Flecs] C
ECS. This makes it *a pleasure* to describe relations, as opposed to the tar pit
of footguns that it is today in bevy.

Sander Mertens, of Flecs fame, [describes in details] the prerequisites for an
entity relationship implementation.
One of those prerequisites is the ability to use entity ids as query parameters.
Dynamic queries allow just that.


### A long wait

Given how useful dynamic queries are, you might be wondering why we added them this late.
But dynamic queries have a long history: they date back from **November 2022**,
when Suficio proposed [a simple change]. It was deemed too unsafe, [a counter-proposal]
was made by Jakob Hellermann. It was stalled due to complexity and the lack of qualified and
interested reviewers. They were finally merged in **January 2024**
thanks to [James O'Brien's stupendous effort][The dynamic query pull request].

For an in-depth technical and historical breakdown of dynamic queries, check
out [this GitHub discussion thread](https://github.com/bevyengine/bevy/discussions/9816).

They have been *a long time* coming and they are finally here!

[`bevy-inspector-egui`]: https://crates.io/crates/bevy-inspector-egui
[Quake-style console]: https://github.com/doonv/bevy_dev_console
[editor with remote capabilities]: https://makeshift-bevy-web-editor.vercel.app/
[a simple change]: https://github.com/bevyengine/bevy/pull/6240
[The dynamic query pull request]: https://github.com/bevyengine/bevy/pull/9774
[a counter-proposal]: https://github.com/bevyengine/bevy/pull/6390
[`QueryBuilder`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.QueryBuilder.html
[`Query`]: https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.Query.html
[describes in details]: https://ajmmertens.medium.com/a-roadmap-to-entity-relationships-5b1d11ebb4eb
[Flecs]: https://www.flecs.dev/flecs/


## Entity optimizations

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WorldQuery trait split

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Automatically inserted sync points

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Input for one-shot systems

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WGPU upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Texture atlas rework

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Sprite slicing and tiling

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Exposure settings

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Minimal reflection probes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light maps

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light RenderLayers

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Approximate indirect specular occlusion

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Unload render assets from RAM

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Bind group layout entries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Camera-driven UI

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Winit upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Animation interpolation

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## gltF extensions

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Gizmo configuration

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## <a name="what-s-next"></a>What's Next?

We have plenty of work in progress! Some of this will likely land in **Bevy 0.14**.

Check out the [**Bevy 0.14 Milestone**](https://github.com/bevyengine/bevy/milestone/20) for an up-to-date list of current work that contributors are focusing on for **Bevy 0.14**.

* **More editor experimentation:** TODO
* **bevy_dev_tools:** TODO
* **A revised scene format:** TODO
* **bevy_ui improvements:** TODO
* **The steady march towards relations:** TODO
* **Animation blending:** TODO
* **Irradiance volumes:** TODO

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/community/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/community/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the 185 contributors that made this release (and associated docs) possible! In random order:

TODO: add contributors

## Full Changelog

The changes mentioned above are only the most appealing, highest impact changes that we've made this cycle.
Innumerable bug fixes, documentation changes and API usability tweaks made it in too.
For a complete list of changes, check out the PRs listed below.

TODO: add full changelog, sorting by area.
