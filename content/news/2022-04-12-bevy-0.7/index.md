+++
title = "Bevy 0.7"
date = 2022-04-12
[extra]
author = "Carter Anderson"
twitter = "cart_cart"
github = "cart"
youtube = "cartdev"
image = "FIXME.png"
show_image = true
image_subtitle = ""
image_subtitle_link = ""
+++

Thanks to **X** contributors, **X** pull requests, and our [**generous sponsors**](https://github.com/sponsors/cart), I'm happy to announce the **Bevy 0.7** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out [Quick Start Guide](/learn/book/getting-started/) to get started. Bevy is also free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.7**, check out our [0.6 to 0.7 Migration Guide](/learn/book/migration-guides/0.6-0.7/).

As always, there are a _ton_ of new features, bug fixes, and quality of life tweaks in this release, but here are some of the highlights: 

* Skeletal animation and mesh skinning
* GLTF animation importing
* Unlimited* point lights in a scene
* Improved clustered forward rendering: dynamic/adaptive clustering, faster and more accurate cluster assignment
* Compressed texture support (KTX2 / DDS / .basis): load more textures in a scene, faster
* Compute shader / pipeline specialization: Bevy's flexible shader system was ported to compute shaders, enabling hot reloading, shader defs, and shader imports
* Render to texture: cameras can now be configured to render to a texture instead of a window
* Flexible user-customizable mesh vertex layouts in shaders
* ECS improvements: Order systems using their names, Query::get_many, use conflicting parameters in systems via ParamSets, WorldQuery derives
* More audio control: pause, volume, speed, and looping
* Hot reloading for "plugin-provided / built-in assets"
* Power usage options to enable only updating Bevy Apps when input occurs 

<!-- more -->

## Skeletal Animation

<div class="release-feature-authors">authors: @james7132, @mockersf, @lassade, @Looooong</div>

Bevy finally supports 3D skeletal animation!

<video controls loop><source  src="skeletal_animation.mp4" type="video/mp4"/></video>

<div style="font-size: 1.0rem" class="release-feature-authors">Scene Credits: <a href="https://skfb.ly/6TsvL">Tanabata evening - Kyoto inspired city scene</a> by Mathias Tossens is licensed under <a href="http://creativecommons.org/licenses/by/4.0/">Creative Commons Attribution</a>. Character model and animation are royalty free assets from Mixamo. 
</div>

Skeletal animations can now be played, paused, scrubbed, looped, reversed, and speed controlled using the new [`AnimationPlayer`] component and [`AnimationClip`] asset:

```rust
#[derive(Component)]
struct Animations {
    dance: Handle<AnimationClip>,
}

fn system(mut query: Query<(&Animations, &mut AnimationPlayer)>) {
    for (animations, mut animation_player) in query.iter_mut() {
        animation_player.play(animations.dance.clone());
    }
}
```

[`AnimationPlayer`] can also be used to animate arbitrary [`Transform`] components, not just skeletons!

This critical feature has been a long time coming, but we wanted to build it in a way that meshed nicely with the [new Bevy renderer](/news/bevy-0-6/#the-new-bevy-renderer) and didn't just "hack things in". This builds on our new [User-Customizable Mesh Vertex Layouts](LINKME), [Shader Imports](/news/bevy-0-6/#shader-imports), and [Material](/news/bevy-0-6/#materials) systems, which ensures that this logic is flexible and reusable, even with non-standard meshes and custom render pipelines.

And we're just getting started! Multi-track animation blending and higher level animation state management should arrive in the very near future. Now is a great time to start contributing animation features to Bevy. We've smashed through most of the foundational technical hurdles and what remains is largely high level api design choices!

[`AnimationPlayer`]: https://docs.rs/bevy/0.7.0/bevy/animation/struct.AnimationPlayer.html
[`AnimationClip`]: https://docs.rs/bevy/0.7.0/bevy/animation/struct.AnimationClip.html
[`Transform`]: https://docs.rs/bevy/0.7.0/bevy/transform/components/struct.Transform.html

## GLTF Animation Importing

<div class="release-feature-authors">authors: @mockersf</div>

Bevy's GLTF importer was extended to import GLTF animations into the new [`AnimationPlayer`] system. This supports both "skeletal animation" and arbitrary transform animations.

<video controls loop><source  src="fox.mp4" type="video/mp4"/></video>

```rust
struct FoxAnimations {
    walk: Handle<AnimationClip>,
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(asset_server.load("models/animated/Fox.glb#Scene0"));
    commands.insert_resource(FoxAnimations {
        walk: asset_server.load("models/animated/Fox.glb#Animation0"),
    });
}

fn play_on_load(
    animations: Res<FoxAnimations>,
    mut players: Query<&mut AnimationPlayer, Added<AnimationPlayer>>,
) {
    for mut player in players.iter_mut() {
        player.play(animations.walk.clone()).repeat();
    }
}
```

## Unlimited* Point Lights

<div class="release-feature-authors">authors: Rob Swain (@superdump), @robtfm</div>

Bevy can now render scenes with arbitrary numbers of point lights on platforms that support storage buffers (which is basically everything but WebGL). In the last Bevy release (0.6) we added [Clustered Forward Rendering](/news/bevy-0-6/#clustered-forward-rendering), which is a rendering technique that optimizes each fragment's light calculation costs by assigning lights to "clusters". However in the interest of platform compatibility (WebGL), we initially limited ourselves to 256 lights because that is what fit in a uniform buffer binding.

In **Bevy 0.7**, we added the ability to automatically "upgrade" to using unbounded storage buffers for Clustered Forward Rendering on platforms that support them, enabling unlimited* point lights. There is an asterisk there because in practice this is limited by memory and hardware constraints. 

## Light Clustering Features and Optimizations

<div class="release-feature-authors">authors: Rob Swain (@superdump), @robtfm, @dataphract, @cart</div>

With the upper limit of 256 point lights removed, the only limit on lights is what the hardware can support and bottlenecks in our algorithms. To increase the number of lights, we made a number of optimizations to our clustering algorithms.

* **Dynamic Light Clusters**: By default cluster x/y slices are now dynamically configured based on the lights in the scene, which can significantly increase performance in some scenes. The clustering behavior is now also user-configurable as FixedZ (the new default dynamic x/y behavior), custom fixed x/y/z slice values, single-cluster, and "no clustering".
* **Light Frustum Change Detection**: We now use Bevy ECS's change detection feature to only recalculate the view frustum of lights that have changed.
* **Cluster Assignment Optimizations**: The cluster assignment data access patterns and data structures received a variety of tweaks that improved performance.

// TODO: Show many lights video here
// TODO: Add iterative sphere refinement here if it is merged

## Configurable Light Visibility 

<div class="release-feature-authors">authors: @robtfm</div>

Lights can now be turned on and off using Bevy's standard [`Visibility`] component:

```rust
commands.spawn(PointLightBundle {
    visibility: Visibility {
        is_visible: false,
    },
    ..default()
});
```

[`Visibility`]: https://docs.rs/bevy/0.7.0/bevy/render/view/struct.Visibility.html

## Auto-Labeled Systems For Nicer System Ordering

<div class="release-feature-authors">authors: @cart, @aevyrie, @alice-i-cecile, @DJMcNab</div>

Bevy uses "labels" to define ordering constraints between its ECS systems when they run in parallel. In previous versions of Bevy, the only option was to define custom labels:

```rust
#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
struct UpdateVelocity;

app
  .add_system(update_velocity.label(UpdateVelocity))
  .add_system(movement.after(UpdateVelocity))
```

In **Bevy 0.7**, systems are now automatically labeled with a `SystemTypeIdLabel` tied to their type. This enables much more ergonomic and clear system ordering:

```rust
app
  .add_system(update_velocity)
  .add_system(movement.after(update_velocity))
```

The Bevy ECS labeling system is powerful and there are still legitimate use cases for custom labels (such as labeling multiple systems with the same label and exporting a stable public API as a plugin author). But most common use cases can take advantage of the ergonomic auto-labeling functionality.

## Default Shorthand

<div class="release-feature-authors">authors: @cart</div>

Bevy makes heavy use of Rust's [struct update pattern](https://doc.rust-lang.org/book/ch05-01-defining-structs.html#creating-instances-from-other-instances-with-struct-update-syntax) in combination with the `Default` trait when initializing entities. This significantly reduces the amount of typing required by enabling developers to only fill in the fields they want to change.

The standard way of doing this is to write out `..Default::default()`:

```rust
commands.spawn_bundle(SpriteBundle {
    texture: some_texture,
    ..Default::default()
})
```

This is much better than filling in each field's component manually:

```rust
commands.spawn(SpriteBundle {
    texture: some_texture,
    sprite: Default::default(),
    transform: Default::default(),
    global_transform: Default::default(),
    visibility: Default::default(),
});
```

However this can feel repetitive when you're doing it for tens or hundreds of entities. We added a way to make this even easier, without needing to resort to macros:

```rust
commands.spawn_bundle(SpriteBundle {
    texture: some_texture,
    ..default()
})
```

This is equivalent in functionality to `..Default::default()`, it's just more compressed. And you can still use the longer form if you prefer. The `default()` function is included in Bevy's prelude by default, so you don't need to manually import it. Ergonomics for the win!

## Query::get_many

<div class="release-feature-authors">authors: @alice-i-cecile</div>

Bevy ECS solves a hard problem: providing easy and fast access to data in parallel while still respecting Rust's strict mutability and ownership rules. Since our first release, we've supported efficiently accessing specific entities in our ECS Queries:

```rust
struct SomeEntities {
    a: Entity,
    b: Entity,
}

fn system(mut query: Query<&mut Transform>, entities: Res<SomeEntities>) {
    let a_transform = query.get_mut(entities.a).unwrap();
}
```

However, to respect Rust's mutability rules, we need to disallow apis that might produce "aliased mutability". Seasoned Bevy users will probably recognize this Rust borrow checker error:

```rust
fn system(mut query: Query<&mut Transform>, entities: Res<SomeEntities>) {
    let a_transform = query.get_mut(entities.a).unwrap();
    // This line files to compile because `query` is already mutably borrowed above
    let b_transform = query.get_mut(entities.b).unwrap();
}
```

_You_ know Entity A and Entity B are different entities at runtime. But Rust's borrow checker has no way to know that at compile time! I'm sure you can imagine game development scenarios that would benefit from having mutable access to multiple components at the same time. This borrow checker restriction was a common pain point and the workarounds were ... not fun (using scopes to ensure conflicting accesses are dropped, copying data, re-querying things, etc).

Fortunately, **Bevy 0.7** introduces a brand new set of apis to save the day!

```rust
fn system(mut query: Query<&mut Transform>, entities: Res<SomeEntities>) {
    // Takes an array of entities and returns an array of mutable Query results
    // This will panic if there are entity conflicts or the entities do not exist
    let [a_transform, b_transform] = query.many_mut([entities.a, entities.b]);
}
```

There are plenty of variants:

```rust
// Same as many_mut, but returns a Result instead of panicking
if let Ok([a_transform, b_transform]) = query.get_many_mut([entities.a, entities.b]) {
}

// There are also immutable/read-only variants
let [a_transform, b_transform] = query.many([entities.a, entities.b]);
if let Ok([a_transform, b_transform]) = query.get_many([entities.a, entities.b]) {
}
```

And they all support arbitrary numbers of entities:

```rust
let [a, b, c] = query.many([entity_a, entity_b, entity_c]);
```

## ParamSets

<div class="release-feature-authors">authors: @bilsen</div>

To prevent aliased mutability, Bevy ECS disallows systems that have parameters that conflict with each other. For example, if two Queries both request write access to the same component in the same "archetype", that could result in aliased mutable access, so Bevy disallows that system and errors out.

Previous versions of Bevy supported conflicting Queries in the same system using QuerySets, which only allow access to one Query in the set at a time:

```rust
// These queries could each return a mutable A component for the same entity, so they must be put in a set to be considered a valid system. 
fn system(mut set: QuerySet<(QueryState<(&mut A, &B)>, QueryState<(&mut A, &C)>)>) {
    for (a, b) in set.q0().iter_mut() {
    }
}
```

**Bevy 0.7** removes `QuerySet` in favor of `ParamSet`, which generalizes the QuerySet pattern for _any_ system parameter:

```rust
fn system(mut set: ParamSet<(Query<(&mut A, &B)>, Query<(&mut A, &C)>)>) {
    for (a, b) in set.p0().iter_mut() {
    }
}
```

But ParamSets aren't just limited to Queries! Consider this example, where the `EventWriter<Jump>` parameter (which internally accesses the `Events<Jump>` resource) conflicts with the raw access to that resource. Previously, expressing this wouldn't be possible. But with ParamSets, it is!

```rust
fn system(mut set: ParamSet<(EventWriter<Jump>, ResMut<Events<Jump>>)>) {
    for jump_event in set.p1().drain() {
    }
}
```

We still recommend avoiding ParamSets where possible for clarity's sake. But every so often they are a necessary and useful tool!

## Deref / DerefMut Derives

<div class="release-feature-authors">authors: @MrGVSV</div>

Rust encourages the use of the [newtype pattern](https://doc.rust-lang.org/rust-by-example/generics/new_types.html) when expanding a type with new functionality or meaning. This is also a useful tool in Bevy:

```rust
#[derive(Component)]
struct Items(Vec<Item>);

fn give_sword(mut query: Query<&mut Items>) { 
    for mut items in query.iter_mut() {
        items.0.push(Item::new("Flaming Poisoning Raging Sword of Doom"));
    }
}
```

This works just fine, but that `0` at the end of `items.0` sticks out like a sore thumb. Many of us in the Bevy Org think `.0` has no place in public apis. But the newtype pattern is still useful! Ideally, Rust would provide a way to express that `Items` is a new type, while transparently provided access to the `Vec<Item>` stored within. There are designs being discussed by the Rust team, but we don't want to wait for nice things!

Fortunately, the Deref / DerefMut traits in std provide the behavior we want. Users can already manually implement these traits, but for such a common pattern, we decided that providing our own trait derives was worth it. In **Bevy 0.7**, you can now derive Deref and DerefMut, enabling much nicer public apis:

```rust
#[derive(Component, Deref, DerefMut)]
struct Items(Vec<Item>);

fn give_sword(mut query: Query<&mut Items>) { 
    for mut items in query.iter_mut() {
        // No more .0!
        items.push(Item::new("Flaming Poisoning Raging Sword of Doom"));
    }
}
```

Astute `std` doc readers might notice that the Rust team [recommends only using `Deref`/`DerefMut` for smart pointers, to avoid confusion](https://doc.rust-lang.org/std/ops/trait.Deref.html). Components like `Items` _are not_ smart pointers. We choose to ignore this advice, as this pattern works, is already widely used in the Rust ecosystem, and Good UX Comes First.

## WorldQuery Derives

<div class="release-feature-authors">authors: @mvlabat</div>

Sometimes when building Bevy Apps you might find yourself repeating the same sets of components over and over in your queries:

```rust
fn move_players(mut players: Query<(&mut Name, &mut Transform, &mut Item)>) {
    for (name, transform, item) in players.iter_mut() {
    }
}

fn despawn_players(mut players: Query<(Entity, &mut Name, &mut Transform, &mut Item)>) {
    for (entity, name, transform, item) in players.iter_mut() {
    }
}
```

Maybe you've gotten tired of typing the same components over and over. In **Bevy 0.7**, you can now easily create your own custom `WorldQuery` trait implementations with the `WorldQuery` derive:

```rust
#[derive(WorldQuery)]
struct PlayerQuery<'w> {
    name: &'w mut Name,
    transform: &'w mut Transform,
    items: &'w mut Items,
}

fn move_players(mut players: Query<PlayerQuery>) {
    for player in players.iter_mut() {
    }
}

fn despawn_players(mut players: Query<(Entity, PlayerQuery)>) {
    for (entity, player) in players.iter_mut() {
    }
}
```

## Support Bevy

Sponsorships help make my full time work on Bevy sustainable. If you believe in Bevy's mission, consider sponsoring me (@cart) ... every bit helps!

<a class="header-item header-button header-button-donate" style="margin-left: 0px;" href="https://github.com/sponsors/cart">Donate <img src="/assets/heart.svg" class="header-button-donate-heart" alt="heart icon"/></a>

## Contributors

A huge thanks to the **X contributors** that made this release (and associated docs) possible! In random order:

## Full Change Log
