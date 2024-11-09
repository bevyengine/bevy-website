First: buckle up because **Required Components** is one of the most profound improvements to the Bevy API surface since Bevy was first released.

Since Bevy's creation, `Bundle` has been our abstraction for spawning an entity of a given "type". A `Bundle` is just a Rust type, where each field is a `Component`:

```rust
#[derive(Bundle)]
struct PlayerBundle {
    player: Player,
    team: Team,
    sprite: Sprite,
    transform: Transform,
    global_transform: GlobalTransform,
    visibility: Visibility,
    inherited_visibility: InheritedVisibility,
    view_visibility: ViewVisibility,
}
```

Then whenever a new player needs to be spawned, developers would initialize and insert a `PlayerBundle` on an entity:

```rust
commands.spawn(PlayerBundle {
    player: Player { 
        name: "hello".into(),
        ..default()
    },
    team: Team::Blue,
    ..default()
});
```

This inserts all of the components in `PlayerBundle`, including the ones not explicitly being set. The `Bundle` concept is functional (it has gotten us this far), but it is also far from ideal:

1. It is an entirely new set of APIs that developers need to learn. Someone that wants to spawn a `Player` entity needs to know that `PlayerBundle` exists.
2. Bundle APIs don't exist at runtime after insertion ... they are an additional spawn-only concept that developers need to think about. You don't write `PlayerBundle` behaviors. You write `Player` behaviors.
3. The `Player` component _needs_ the components in `PlayerBundle` to function as a `Player`. Spawning `Player` on its own is possible, and it likely (depending on the implementation) wouldn't function as intended.
4. Bundles are always "flat" (by convention). The person defining the `Player` component needs to define _all of the component dependencies_. `Sprite` needs `Transform` and `Visibility`, `Transform` needs `GlobalTransform`, `Visibility` needs `InheritedVisibility` and `ViewVisibility`. This lack of "dependency inheritance" makes defining bundles much harder and error prone than it needs to be. It requires consumers of APIs to be intimately aware of what amounts to implementation details. And when these details change, _the developer of the `Bundle` needs to be aware and update the `Bundle` accordingly_. Nested bundles are supported, but they are a _pain_ for users to work with and we have disallowed them in upstream Bevy bundles for a while now.
5. `PlayerBundle` is effectively defined by the needs of the `Player` component, but when spawning it is possible to _never mention the `Player` symbol_. Ex: `commands.spawn(PlayerBundle::default())`. This is odd given that `Player` is the "driving concept".
6. Bundles introduce significant "stutter" to the API. Notice the `player: Player` and `team: Team` in the example above.
7. Bundles introduce additional (arguably excessive) nesting and `..default()` usage.

Every one of these points has a sizable impact on what it feels like to use Bevy on a day-to-day basis. In **Bevy 0.15** we've landed **Required Components**, which solves these problems by fundamentally rethinking how this all works.

**Required Components** are the first step in our [Next Generation Scene / UI](https://github.com/bevyengine/bevy/discussions/14437) effort, which aims to make Bevy a best-in-class app / scene / UI development framework. **Required Components** stand on their own as a direct improvement to Bevy developers' lives, but they also help set the stage for making Bevy's upcoming next generation scene system (and the upcoming Bevy Editor) something truly special.

### What are they?

**Required Components** enable developers to define which components a given component needs:

```rust
#[derive(Component, Default)]
#[require(Team, Sprite)]
struct Player {
    name: String,
}
```

When the `Player` component is inserted, its **Required Components** _and the components required by those components_ are automatically inserted as well!

```rust
commands.spawn(Player::default());
```

The code above automatically inserts `Team` and `Sprite`. `Sprite` requires `Transform` and `Visibility`, so those are automatically inserted as well. Likewise `Transform` requires `GlobalTransform` and `Visibility` requires `InheritedVisibility` and `ViewVisibility`.

This code produces the same result as the `PlayerBundle` code in the previous section:

```rust
commands.spawn((
    Player {
        name: "hello".into(),
        ..default()
    },
    Team::Blue,
))
```

Much better right? The `Player` type is easier and less error prone to define, and spawning it takes less typing and is easier to read.

### Efficiency

We've implemented **Required Components** in a way that makes them effectively "free":

1. Required Components are only initialized and inserted if the caller did not insert them manually. No redundancy!
2. Required Components are inserted alongside the normal components, meaning (for you ECS nerds out there) there are no additional archetype changes or table moves. From this perspective, the Required Components version of the `Player` example is identical to the `PlayerBundle` approach, which manually defines all of the components up front.
3. Required Components are cached on the archetype graph, meaning computing what required components are necessary for a given type of insert only happens once.

### Component Initialization

By default, **Required Components** will use the `Default` impl for the component (and fail to compile if one does not exist):

```rust
#[derive(Component)]
#[require(Team)] // Team::Red is the default value
struct Player {
    name: String,
}

#[derive(Default)]
enum Team {
    #[default]
    Red,
    Blue,
}
```

This can be overridden by passing in a function that returns the component:

```rust
#[derive(Component)]
#[require(Team(blue_team)]
struct Player {
    name: String,
}

fn blue_team() -> Team {
    Team::Blue
}
```

To save space, you can also pass a closure to the require directly:

```rust
#[derive(Component)]
#[require(Team(|| Team::Blue))]
struct Player {
    name: String,
}
```

### Isn't this a bit like inheritance?

**Required Components** _can_ be considered a form of inheritance. But it is notably _not_ traditional object-oriented inheritance. Instead it is "inheritance by composition". A `Button` widget can (and should) require `Node` to make it a "UI node". In a way, a `Button` "is a" `Node` like it would be in traditional inheritance. But unlike traditional inheritance:

1. It is expressed as a "has a" relationship, not an "is a" relationship.
2. `Button` and `Node` are still two entirely separate types (with their own data), which you query for separately in the ECS.
3. A `Button` can require more components in _addition_ to `Node`. You aren't limited to "straight line" standard object-oriented inheritance. Composition is still the dominating pattern.
4. You don't _need_ to require components to add them. You can still tack on whatever additional components you want during spawn to add behaviors in the normal "composition style".

### What is happening to Bundles?

The `Bundle` trait will continue to exist, and it is still the fundamental building block for insert APIs (tuples of components still implement `Bundle`). Developers are still free to define their own custom bundles using the `Bundle` derive. Bundles play nicely with **Required Components**, so you can use them with each other.

That being said, as of Bevy **0.15** we have deprecated all built-in bundles like `SpriteBundle`, `NodeBundle`, `PbrBundle`, etc. in favor of **Required Components**. In general, **Required Components** are now the preferred / idiomatic approach. We encourage Bevy plugin and app developers to port their bundles over to **Required Components**.

### Porting Bevy to Required Components

As mentioned above, _all_ built-in Bevy bundles have been deprecated in favor of **Required Components**. We've also made API changes to take advantage of this new paradigm. This does mean breakage in a few places, but the changes are so nice that we think people won't complain too much :)

In general, we are moving in the direction specified by our [Next Generation Scene / UI](https://github.com/bevyengine/bevy/discussions/14437) document. Some general design guidelines:

1. When spawning an entity, generally there should be a "driving concept" component.  When implementing a new entity type / behavior, give it a concept name ... that is the name of your "driving component" (ex: the "player" concept is a `Player` component). That component should require any additional components necessary to perform its functionality.
2. People should think directly in terms of components and their fields when spawning. Prefer using component fields directly on the "concept component" as the "public API" for the feature.
3. Prefer simple APIs / don't over-componentize. By default, if you need to attach new properties to a concept, just add them as fields to that concept's component. Only break out new components / concepts when you have a good reason, and that reason is motivated by user experience or performance (and weight user experience highly). If a given "concept" (ex: a `Sprite`) is broken up into 10 components, that is _very_ hard for users to reason about and work with.
4. Instead of using Asset handles directly as components, define new components that hold the necessary handles. Raw asset handles as components were problematic for a variety of reasons (a big one is that you can't define context-specific **Required Components** for them), so we have removed the `Component` implementation for `Handle<T>` to encourage (well ... force) people to adopt this pattern.

#### UI

Bevy UI has benefitted tremendously from **Required Components**. UI nodes require a variety of components to function, and now all of those requirements are consolidated on `Node`. Defining a new UI node type is now as simple as adding `#[require(Node)]` to your component.

```rust
#[derive(Component)]
#[require(Node)]
struct MyNode;

commands.spawn(MyNode);
```

The `Style` component fields have been moved into `Node`. `Style` was never a comprehensive "style sheet", but rather just a collection of properties shared by all UI nodes. A "true" ECS style system would style properties _across_ components (`Node`, `Button`, etc), and we [do have plans to build a true style system](https://github.com/bevyengine/bevy/discussions/14437). All "computed" node properties (such as the size of the node after it has been laid out) have been moved to the `ComputedNode` component.

This change has made spawning UI nodes in Bevy _much_ cleaner and clearer:

```rust
commands.spawn(Node {
    width: Val::Px(100.),
    ..default()
});
```

Compare that to what it was before!

```rust
commands.spawn(NodeBundle {
    style: Style {
        width: Val::Px(100.),
        ..default()
    },
    ..default()
})
```

UI components like `Button`, `ImageNode` (previously `UiImage`), and `Text` now require `Node`. Notably, `Text` has been reworked to be easier to use and more component driven (we'll cover this more in the next section):

```rust
commands.spawn(Text::new("Hello there!"));
```

`MaterialNode<M: UiMaterial>` is now a proper component for "UI material shaders", and it also requires `Node`:

```rust
commands.spawn(MaterialNode(my_material));
```

#### Text

Bevy's Text API has been reworked to be simpler and more component driven. There are still two primary text components: `Text` (the UI text component) and `Text2d` (the world-space 2D text component).

The first thing that has changed is that these primary components are literally _just_ a `String` newtype:

```rust
commands.spawn(Text("hello".to_string()))
commands.spawn(Text::new("hello"))
commands.spawn(Text2d("hello".to_string()))
commands.spawn(Text2d::new("hello"))
```

Spawn one of these components, and you have text! Both of these components now require the following components:

* `TextFont`: configures the font / size
* `TextColor`: configures the color
* `TextLayout`: configures how the text is laid out.

`Text`, which is the UI component, also requires `Node` because it _is_ a node. Similarly, `Text2d` requires a `Transform` because it is positioned in world space.

Both `Text` and `Text2d` are a standalone "block" of text. These top level text components also contribute a single "span" of text, which is added to the "block". If you need "rich text" with multiple colors / fonts / sizes, you can add `TextSpan` entities as children of either `Text` or `Text2d`. `TextSpans` use the same `TextFont` / `TextLayout` components to configure text. Each `TextSpan` will contribute its span to its parent text:

```rust
// The `Text` UI node will render "hello world!", where "hello" is red and "world!" is blue
commands.spawn(Text::default())
    .with_child((
        TextSpan::new("hello"),
        TextColor::from(RED),
    ))
    .with_child((
        TextSpan::new(" world!"),
        TextColor::from(BLUE),
    ));
```

This produces the exact same output, but uses the "default" span on the top-level `Text` component:

```rust
commands.spawn((
    Text::new("hello"),
    TextColor::from(RED),
))
.with_child((
    TextSpan::new(" world!"),
    TextColor::from(BLUE),
));
```

This "entity driven" approach to text spans replaces the "internal span array" approach used in previous Bevy versions. This yields significant benefits. First, it lets you use the normal Bevy ECS tools, such as marker components and queries, to mark a text span and access it directly. This is much easier (and more resilient) than using indices in an array, which are hard to guess and unstable as the array contents change:

```rust
#[derive(Component)]
struct NameText;

commands.spawn(Text::new("Name: "))
    .with_child((
        TextSpan::new("Unknown"),
        NameText, 
    ));

fn set_name(mut names: Query<&mut TextSpan, With<NameText>>) {
    names.single_mut().0 = "George".to_string();
}
```

Text spans as entities play nicer with Bevy Scenes (including the upcoming [Next Generation Scene / UI](https://github.com/bevyengine/bevy/discussions/14437) system), and allow it to integrate nicely with existing tools like entity inspectors, animation systems, timers, etc.

#### Sprites

Sprites are largely unchanged. In addition to the **Required Components** port (`Sprite` now requires `Transform` and `Visibility`), we've also done some component consolidation. The `TextureAtlas` component is now an optional `Sprite::texture_atlas` field. Likewise the `ImageScaleMode` component is now a `Sprite::image_mode` field. Spawning sprites is now super simple!

```rust
commands.spawn(Sprite {
    image: assets.load("player.png"),
    ..default()
});
```

#### Transforms

`Transform` now requires `GlobalTransform`. If you want your entity to have "hierarchical transforms", require `Transform` (and it will add `GlobalTransform`). If you just want your entity to have a "flat" global transform, require `GlobalTransform`.

Most Bevy components that are intended to exist in world space now require `Transform`.

#### Visibility

The `Visibility` component now requires `InheritedVisibility` and `ViewVisibility`, meaning that you can now just require `Visibility` if you want your entity to be visible. Bevy's built-in "visible" components, such as `Sprite`, require `Visibility`.

#### Cameras

The `Camera2d` and `Camera3d` components now each require `Camera`. `Camera` requires the various camera components (`Frustum`, `Transform`, etc.). This means that you can spawn a 2D or 3D camera like this:

```rust
commands.spawn(Camera2d::default());
commands.spawn(Camera3d::default());
```

`Camera2d` and `Camera3d` also require the components that set the relevant default render graphs and enable the default render features relevant to the 2D and 3D contexts (respectively).

You can of course explicitly set the values of the other components:

```rust
commands.spawn((
    Camera3d::default(),
    Camera {
        hdr: true,
        ..default()
    },
    Transform {
        translation: Vec3::new(1.0, 2.0, 3.0),
        ..default()
    },
));
```

Opt-in Camera render feature components (`MotionBlur`, `TemporalAntiAliasing`, `ScreenSpaceAmbientOcclusion`, and `ScreenSpaceReflections`) now require the relevant camera render feature components. For example, `MotionBlur` now requires `DepthPrepass` and `MotionVectorPrepass`. This makes enabling camera features much easier!

```rust
commands.spawn((
    Camera3d::default(),
    MotionBlur,
))
```

#### Meshes

The old mesh approach relied on adding `Handle<Mesh>` and `Handle<M: Material>` components directly (via `PbrBundle` and `MaterialMeshBundle`), neither of which were compatible with required components.

In **Bevy 0.15** you use `Mesh3d` and `MeshMaterial3d<M: Material>` to render a mesh in 3D:

```rust
commands.spawn((
    Mesh3d(mesh),
    MeshMaterial3d(material),
));
```

`Mesh3d` requires `Transform` and `Visibility`.

There are also 2D equivalents:

```rust
commands.spawn((
    Mesh2d(mesh),
    MeshMaterial2d(material),
));
```

#### Meshlets

Bevy's "virtual geometry" implementation (similar to Nanite), has also been ported. It uses the same pattern as `Mesh3d` and `Mesh2d`:

```rust
commands.spawn((
    MeshletMesh3d(mesh),
    MeshMaterial3d(material),
));
```

#### Lights

The light port involved no major changes to the component structure. All of the spatial light types (`PointLight`, `DirectionalLight`, `SpotLight`) now require `Transform` and `Visibility`, and each light component requires the relevant light-specific configuration components (ex: `PointLight` requires `CubemapFrusta` and `CubemapVisibleEntities`).

Spawning a light of a given type is now as simple as:

```rust
commands.spawn(PointLight {
    intensity: 1000.0,
    ..default()
});
```

The `LightProbe` component now also requires `Transform` and `Visibility`.

#### Volumetric Fog

The `FogVolume` component now requires `Transform` and `Visibility`, meaning you can now add volumetric fog like this:

```rust
commands.spawn(FogVolume {
    density_factor: 0.2,
    ..default()
});
```

#### Scenes

Scenes previously used raw `Handle<Scene>` components, spawned via `SceneBundle`. **Bevy 0.15** introduces the `SceneRoot` component, which wraps the scene handle and requires `Transform` and `Visibility`:

```rust
commands.spawn(SceneRoot(some_scene));
```

#### Audio

Audio also used a raw `Handle<AudioSource>` spawned via an  `AudioBundle`. We've added a new `AudioPlayer` component, which will trigger audio playback when spawned:

```rust
command.spawn(AudioPlayer(assets.load("music.mp3")));
```

`AudioPlayer` requires the `PlaybackSettings` component.

Non-standard audio from arbitrary `Decodable` trait impls can use the `AudioSourcePlayer` component, which also requires `PlaybackSettings`.  

### IDE Integration

**Required Components** play nicely with Rust Analyzer. You can "go to definition" / press `F12` on required components to navigate to where they are defined in code.

### Runtime Required Components

In some cases, developers without direct control over a component might want to add _additional_ **Required Components** on top of the ones provided directly on the type via `#[require(Thing)]`. This is supported!

```rust
// Make `Bird` require `Wings` with a `Default` constructor.
app.register_required_components::<Bird, Wings>();

// Make `Wings` require `FlapSpeed` with a custom constructor.
app.register_required_components_with::<Wings, FlapSpeed>(|| FlapSpeed::from_duration(1.0 / 80.0));
```

Note that only _adding_ Required Components is allowed. Removing Required Components from a type you do not own is explicitly not supported, as that could invalidate assumptions made upstream.

In general, this is intended to be used in very specific, targeted contexts, such as a physics plugin adding additional metadata to a core type it does not control. Adding a new component requirement could change the performance characteristics of the app or break it in unexpected ways. When in doubt, don't do it!
