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
* Improved clustered forward rendering: dynamic/adaptive clustering and faster cluster assignment
* Compressed texture support (KTX2 / DDS / .basis): load more textures in a scene, faster
* Compute shader / pipeline specialization: Bevy's flexible shader system was ported to compute shaders, enabling hot reloading, shader defs, and shader imports
* Render to texture: cameras can now be configured to render to a texture instead of a window
* Flexible mesh vertex layouts in shaders
* ECS improvements: Order systems using their names, Query::many_mut, use conflicting parameters in systems via ParamSets, WorldQuery derives
* Documentation improvements: better examples, more doc tests and more coverage
* More audio control: pause, volume, speed, and looping
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

fn start_dancing(mut query: Query<(&Animations, &mut AnimationPlayer)>) {
    for (animations, mut animation_player) in query.iter_mut() {
        animation_player.play(animations.dance.clone());
    }
}
```

[`AnimationPlayer`] can also be used to animate arbitrary [`Transform`] components, not just skeletons!

This critical feature has been a long time coming, but we wanted to build it in a way that meshed nicely with the [new Bevy renderer](/news/bevy-0-6/#the-new-bevy-renderer) and didn't just "hack things in". This builds on our new [Flexible Mesh Vertex Layouts](/news/bevy-0-7/#flexible-mesh-vertex-layouts), [Shader Imports](/news/bevy-0-6/#shader-imports), and [Material](/news/bevy-0-6/#materials) systems, which ensures that this logic is flexible and reusable, even with non-standard meshes and custom render pipelines.

And we're just getting started! Multi-track animation blending and higher level animation state management should arrive in the very near future. Now is a great time to start contributing animation features to Bevy. We've smashed through most of the foundational technical hurdles and what remains is largely high level api design choices. We already have a couple of draft RFCs open in these areas: [Animation Composition](https://github.com/bevyengine/rfcs/pull/51) and [Animation Primitives](https://github.com/bevyengine/rfcs/pull/49). Feel free to join the conversation!

[`AnimationPlayer`]: https://docs.rs/bevy/0.7.0/bevy/animation/struct.AnimationPlayer.html
[`AnimationClip`]: https://docs.rs/bevy/0.7.0/bevy/animation/struct.AnimationClip.html
[`Transform`]: https://docs.rs/bevy/0.7.0/bevy/transform/components/struct.Transform.html

## GLTF Animation Importing

<div class="release-feature-authors">authors: @mockersf</div>

Bevy's GLTF importer was extended to import GLTF animations into the new [`AnimationPlayer`] system. This supports both "skeletal animation" and arbitrary transform animations:

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

Here is a video illustrating a progression from the old limit of 256 point lights to 25,000 point lights at 60fps!

(Note that the 25,000 lights example disables the debug light spheres to ensure that light calculations are the bottleneck)

<video controls loop><source  src="many_lights.mp4" type="video/mp4"/></video>

And we have [even more clustering optimizations](https://github.com/bevyengine/bevy/pull/4345) in the works! 

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

## Compressed GPU Textures

<div class="release-feature-authors">authors: Rob Swain (@superdump)</div>

As scenes grow larger, so do their assets. Compressing these assets is a great way to save space. The Amazon Bistro scene featured below has well over 1GB of compressed textures.

PNG is a popular compressed format, but it must be decompressed before the GPU can use it. This can be a slow process for large scenes. Those textures are then used in their uncompressed form, taking up large quantities of limited memory. Compressed GPU textures can be used directly in their compressed format by the GPU and can be loaded without any additional processing. This reduces load times significantly. As they remain compressed, this also reduces RAM usage significantly.

The Bistro scene took a total of 12.9s to load with PNG textures, but only 1.5s with compressed textures - taking approximately a tenth of the load time! The total RAM usage was ~12GB with uncompressed textures, and 5GB with compressed textures, less than half!

The benefits don't stop there either - because the textures are compressed and can be used by the GPU in that format, reading from them uses less memory bandwidth, which can bring performance benefits. The Bistro scene gains about 10% in frame rate from using compressed textures.

![bistro compressed](bistro_compressed.png)


Another benefit is that mipmaps are supported, which makes for smoother, less noisy textures. Bevy currently doesn't have automatic support for generating mipmaps for "normal" textures, so using compressed textures is a nice way to have mipmaps now, even though we don't support them for standard textures yet!

In summary, Bevy now supports loading compressed textures from `.dds`, `.ktx2`, and `.basis` files. This includes support for the standard ASTC, BCn, and ETC2 formats, as well as "universal" formats like ETC1S and UASTC that can be transcoded to formats supported by specific systems at runtime. The GLTF loader was also extended to support loading these formats.

These features can be enabled using the `dds`, `ktx2`, and `basis-universal` cargo features.

## Render To Texture

<div class="release-feature-authors">authors: @HackerFoo</div>

Bevy now has initial support for rendering to texture by configuring the `render_target` field on `Camera`. This enables scenarios such as mirrors, split screen, 2d UI in 3d space, portals, etc.

<video controls loop><source  src="render_to_texture.mp4" type="video/mp4"/></video>

Note that the current implementation is relatively low level. It will generally require interacting with Bevy's Render Graph and defining new camera types. If you would like to use this feature now, the [render_to_texture example](https://github.com/bevyengine/bevy/blob/main/examples/3d/render_to_texture.rs) illustrates the steps required. We have plans for ["high level render targets"](https://github.com/bevyengine/bevy/discussions/4191) that will make rendering to textures possible in just a few lines of code. Stay tuned for details!

## Bevy-Native Compute Shaders

<div class="release-feature-authors">authors: @Ku95</div>

Bevy's flexible asset-driven shader system was ported to compute shaders/pipelines, enabling hot reloading, [shader defs](https://bevyengine.org/news/bevy-0-6/#shader-preprocessor), [shader imports](https://bevyengine.org/news/bevy-0-6/#shader-imports), and [pipeline specialization](https://bevyengine.org/news/bevy-0-6/#pipeline-specialization) based on user-configurable keys:

```rust
#import "shaders/game_of_life_texture_bind_group.wgsl"

[[stage(compute), workgroup_size(8, 8, 1)]]
fn game_of_life_update([[builtin(global_invocation_id)]] invocation_id: vec3<u32>) {
    let location = vec2<i32>(i32(invocation_id.x), i32(invocation_id.y));
    let alive = is_location_alive(location);

    // shader defs are configurable at runtime, prompting new variants of the shader to be compiled
#ifdef WRITE_OUTPUT
    storageBarrier();
    textureStore(texture, location, vec4<f32>(f32(alive)));
#endif
}
```

## Flexible Mesh Vertex Layouts

<div class="release-feature-authors">authors: @cart, @parasyte</div>

In **Bevy 0.7**, it is now easy to make shaders support any Mesh vertex layout and arbitrary vertex attributes. Bevy's "shader pipeline specialization" system was extended to support "specializing on mesh vertex layouts".

For most Bevy users, this means that [Materials](/news/bevy-0-6/#materials), including the built in [`StandardMaterial`] and custom shader materials now support arbitrary Meshes automatically, provided those Meshes have the vertex attributes required by the material shaders.

We also made use of this system to implement joint weights and indices for our new [Skeletal Animation](/news/bevy-0-7/#skeletal-animation) implementation.

For Bevy users that like to write lower level graphics pipelines, this feature makes it possible to easily and efficiently specialize your pipelines according to Mesh vertex layouts:

```rust
impl SpecializedMeshPipeline for SomeCustomPipeline {
    type Key = SomeCustomKey;

    fn specialize(
        &self,
        key: Self::Key,
        layout: &MeshVertexBufferLayout,
    ) -> Result<RenderPipelineDescriptor, SpecializedMeshPipelineError> {
        // this is a layout that matches the requirements requested,
        // but catered to whatever mesh is currently being rendered
        let vertex_buffer_layout = layout.get_layout(&[
            Mesh::ATTRIBUTE_POSITION.at_shader_location(0),
            Mesh::ATTRIBUTE_NORMAL.at_shader_location(1),
            Mesh::ATTRIBUTE_UV_0.at_shader_location(2),
        ])?;

        Ok(RenderPipelineDescriptor {
            vertex: VertexState {
                buffers: vec![vertex_buffer_layout],
                /* define the rest of the vertex state here */
            },
            /* define the rest of the mesh pipeline here */
        })
    }
```

[`StandardMaterial`]: https://docs.rs/bevy/0.7.0/bevy/pbr/struct.StandardMaterial.html

## Camera Marker Components

<div class="release-feature-authors">authors: @jakobhellermann</div>

In **Bevy 0.7**, Cameras now use the "marker component" pattern to determine the "camera type" (ex: 3D, 2D, UI), rather than using string names.

This means that it is now cheaper and easier to select cameras of a specific type:

```rust
fn move_3d_camera_system(transforms: Query<&mut Transform, With<Camera3d>>) {
    for mut camera in transforms.iter_mut() {
        // move camera here
    }
}
```

## Ergonomic System Ordering

<div class="release-feature-authors">authors: @cart, @aevyrie, @alice-i-cecile, @DJMcNab</div>

Bevy uses "labels" to define ordering constraints between its ECS systems when they run in parallel. In previous versions of Bevy, the only way to order systems was to define custom labels:

```rust
#[derive(SystemLabel, Clone, Hash, Debug, PartialEq, Eq)]
struct UpdateVelocity;

app
  .add_system(update_velocity.label(UpdateVelocity))
  .add_system(movement.after(UpdateVelocity))
```

In **Bevy 0.7**, manually defining labels is no longer required. You can order systems using functions, just like you do when adding systems!

```rust
app
  .add_system(update_velocity)
  .add_system(movement.after(update_velocity))
```

This is accomplished by "auto-labeling" systems with their [`TypeId`] (the label type is [`SystemTypeIdLabel`]). Internally ordering still uses labels.

The Bevy ECS labeling system is powerful and there are still legitimate use cases for custom labels (such as labeling multiple systems with the same label and exporting a stable public API as a plugin author). But most common use cases can take advantage of the ergonomic auto-labeling functionality.

[`TypeId`]: https://doc.rust-lang.org/std/any/struct.TypeId.html
[`SystemTypeIdLabel`]: https://docs.rs/bevy/0.7.0/bevy/ecs/system/struct.SystemTypeIdLabel.html

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

This is equivalent in functionality to `..Default::default()`, it's just more compressed. And you can still use the longer form if you prefer. The `default()` function is included in Bevy's prelude, so you don't need to manually import it. Ergonomics for the win!

## Query::many

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
    // This line fails to compile because `query` is already mutably borrowed above
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
fn move_players(mut players: Query<(&mut Transform, &mut Velocity, &mut PlayerStats)>) {
    for (transform, velocity, stats) in players.iter_mut() {
    }
}

fn player_gravity(mut players: Query<(Entity, &mut Transform, &mut Velocity, &mut PlayerStats)>) {
    for (entity, transform, velocity, stats) in players.iter_mut() {
    }
}
```

Maybe you've gotten tired of typing the same components over and over. In **Bevy 0.7**, you can now easily create your own custom [`WorldQuery`] trait implementations with the [`WorldQuery`] derive:

```rust
#[derive(WorldQuery)]
struct PlayerMovementQuery<'w> {
    transform: &'w mut Transform,
    velocity: &'w mut Velocity,
    stats: &'w mut PlayerStats,
}

fn move_players(mut players: Query<PlayerMovementQuery>) {
    for player in players.iter_mut() {
    }
}

fn player_gravity(mut players: Query<(Entity, PlayerMovementQuery)>) {
    for (entity, player) in players.iter_mut() {
    }
}
```

[`WorldQuery`]: http://docs.rs/bevy/0.7.0/bevy/ecs/query/trait.WorldQuery.html

## World::resource

<div class="release-feature-authors">authors: @alice-i-cecile</div>

We've noticed that the majority of direct [`World`] resource access immediately unwraps the results of `get_resource`:

```rust
let time = world.get_resource::<Time>().unwrap();
```

In **Bevy 0.7** we added an ergonomic variant that internally panics:

```rust
let time = world.resource::<Time>();
```

There is also a mutable variant:

```rust
let mut time = world.resource_mut::<Time>();
```

The `get_resource` variants are still available for cases where users still want to manually handle the returned `Option`.

[`World`]: https://docs.rs/bevy/0.7.0/bevy/ecs/world/struct.World.html

## AnyOf Queries

<div class="release-feature-authors">authors: @TheRawMeatball</div>

Bevy ECS Queries now support [`AnyOf`], which will return results for entities that match "any of" the given component queries:

```rust
fn system(query: Query<AnyOf<(&A, &B)>>) {
    for (a, b) in query.iter() {
        // Either A or B is guaranteed to be Some
        assert!(a.is_some() || b.is_some())
    }
}
```

For the example above [`AnyOf`] will return entities that have A and not B, B and not A, and both A and B.

[`AnyOf`]: http://docs.rs/bevy/0.7.0/bevy/ecs/query/struct.AnyOf.html

## &World System Param

<div class="release-feature-authors">authors: @bilsen</div>

It is now possible for "normal systems" have `&World` system params, which provide full read-only access to the entire [`World`]:

```rust
fn system(world: &World, transforms: Query<&Transform>) {
}
```

Just keep in mind that `&World` will conflict with _any_ mutable Query:

```rust
fn invalid_system(world: &World, transforms: Query<&mut Transform>) {
}
```

In these cases, consider using our new [ParamSets](/news/bevy-0-7/#paramsets) to resolve the conflict:

```rust
fn valid_system(set: ParamSet<(&World, Query<&mut Transform>)>) {
}
```

## ECS Soundness / Correctness Improvements

<div class="release-feature-authors">authors: @BoxyUwU, @TheRawMeatball, @bjorn3</div>

Bevy ECS received a solid number of soundness and correctness bug fixes this release:
* Removed unsound lifetime annotations on `EntityMut` and `Query`, which could be used to get aliased mutability in some situations.
* Labeled `World::entities_mut` unsafe (because manually modifying entity metadata can invalidate safety assumptions)
* Removed unsound `World::components_mut` (which allowed replacing component metadata, invalidating assumptions made elsewhere in World)
* Fixed a `World::resource_scope` soundness bug
* Used `ManuallyDrop` in resource id initialization instead of `forget()` to avoid invalidating a data pointer before it is used.

We now also run the [miri](https://github.com/rust-lang/miri) interpreter on Bevy ECS in our CI to help detect and prevent future soundness / correctness issues. 

As Bevy ECS matures, our bar for unsafe code blocks and soundness must also mature. Bevy ECS will probably never be 100% free of unsafe code blocks because we are modeling parallel data access that Rust cannot reason about without our help. But we are committed to removing as much unsafe code as we can and improving the quality and scope of our unsafe code.

## Audio Control

<div class="release-feature-authors">authors: @mockersf</div>

Bevy's audio system has been in a ... minimalist state since our first release. Until now, it only supported pressing "play" on audio assets. Third party plugins such as [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio) have filled in the gaps with much more flexible audio solutions.

In **Bevy 0.7** we've started expanding what our built in audio plugin can do. It is now possible to pause, adjust volume, and set playback speed using [`AudioSink`] assets.

Playing audio now returns a `Handle<AudioSink>` which can be used to play/pause/set_speed/set_volume:

```rust
struct BeautifulMusic(Handle<AudioSink>);

fn setup_audio(
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
    audio_sinks: Res<Assets<AudioSink>>,
) {
    let music = asset_server.load("BeautifulMusic.ogg");
    // play audio and upgrade to a strong handle
    let sink_handle = audio_sinks.get_handle(audio.play(music));
    commands.insert_resource(BeautifulMusic(sink_handle));
}

// later in another system
fn adjust_audio(music: Res<BeautifulMusic>, mut audio_sinks: ResMut<Assets<AudioSink>>) {
    if let Some(sink) = audio_sinks.get(music.0) {
        // pause playback
        sink.pause();
        // start playback again
        sink.play();
        // increase the volume
        sink.set_volume(sink.volume() + 0.1);
        // slow down playback
        sink.set_speed(0.5);
    }
}
```

You can also now loop audio playback:

```rust
audio.play_with_settings(music, PlaybackSettings::LOOP.with_volume(0.75));
```

We plan to continue iterating on these APIs with even more functionality and usability improvements!

[`AudioSink`]: http://docs.rs/bevy/0.7.0/bevy/audio/struct.AudioSink.html

## EventLoop Power Saving Modes

<div class="release-feature-authors">authors: @aevyrie</div>

By default Bevy will run updates "as fast as it can" (limited by the monitors' refresh rate). This is great for most games, but some application types (such as GUI apps) need to prioritize CPU and GPU power usage.

**Bevy 0.7** adds the ability to configure the [`UpdateMode`] in [`WinitConfig`] to configure how Bevy Apps run updates:

* **Continuous**: always update "as soon as possible" (honoring vsync configuration)
* **Reactive**: only update when there is a window event, a redraw is requested, or a configurable wait time has elapsed 
* **ReactiveLowPower**: only update when there is user input (mouse movement, keyboard input, etc), a redraw is requested, or a configurable wait time has elapsed

These settings can be configured separately for focused windows and unfocused windows (enabling you to save power when a window loses focus). 

**ReactiveLowPower** can _significantly_ reduce power / resource usage, but it won't be suitable for every app type, as some apps need to assume that they are constantly being updated as quickly as possible. Therefore these settings are opt-in.

This app demos the various modes available. Note that Game mode was configured to lower its tick rate when it loses focus, which is not the default:

<video controls loop><source  src="power_settings.mp4" type="video/mp4"/></video>

## Documentation improvements

<div class="release-feature-authors">authors: @alice-i-cecile and many more</div>

Great docs make learning, using and building Bevy better.
But as a young engine, they're still a work-in-progress.

### deny-missing-docs

Our docs team (led by `@alice-i-cecile`) has started to [systematically fix that](https://github.com/bevyengine/bevy/issues/3492), with the help of Rust's `#[warn(missing_docs)]` lint.
Since 0.6, we've fully documented (and prevented doc-regressions for):

* `bevy_tasks` by `@james7132`
* `bevy_app` by `@dbearden`
* `bevy_dylib` by `@KDecay`
* `bevy_internal` by `@sheepyhead`

There have been [many other doc improvements](https://github.com/bevyengine/bevy/pulls?q=is%3Apr+is%3Aclosed+label%3AC-Docs) over this time period as well, including the addition of many helpful [doc tests](https://doc.rust-lang.org/rustdoc/documentation-tests.html), and our bar for docs in new code continues to rise.
A huge thanks to everyone making Bevy's docs better.

### New contributors

If you're [interested in contributing](https://github.com/bevyengine/bevy/blob/main/CONTRIBUTING.md), the docs team is always ready to help new contributors get their first Bevy PR merged ASAP.
There have been a _ton_ of new contributors who've helped out with docs, either as a writer or a reviewer.
If this is you: thanks!

### Better examples

For many people, the best way to learn a tool is to see it in action.
We've been steadily polishing our [examples](https://github.com/bevyengine/bevy/tree/latest/examples) with better explanations, more coverage, and higher code quality.
If you're new to Bevy, check out the much-improved [Breakout example](https://github.com/bevyengine/bevy/blob/latest/examples/games/breakout.rs)!

## Dev Docs

<div class="release-feature-authors">authors: @james7132, @mockersf, @aevyrie</div>

We now automatically deploy Bevy's `main` development branch to [https://dev-docs.bevyengine.org](https://dev-docs.bevyengine.org) whenever a change is merged. This will help Bevy documentation authors easily validate their changes. And "bleeding edge" Bevy users can learn about API changes we're working on.

![dev docs](dev_docs.png)

## Support Bevy

Sponsorships help make my full time work on Bevy sustainable. If you believe in Bevy's mission, consider sponsoring me (@cart) ... every bit helps!

<a class="header-item header-button header-button-donate" style="margin-left: 0px;" href="https://github.com/sponsors/cart">Donate <img src="/assets/heart.svg" class="header-button-donate-heart" alt="heart icon"/></a>

## Contributors

A huge thanks to the **X contributors** that made this release (and associated docs) possible! In random order:

## Full Change Log
