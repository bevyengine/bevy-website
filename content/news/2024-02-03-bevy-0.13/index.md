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
And to see what the engine has to offer hands-on, check out the entries in the [latest Bevy Jam](https://itch.io/jam/bevy-jam-4/entries), including the winner [That's a lot of beeeeees](https://andrewb330.itch.io/thats-a-lot-of-beeeeees)

To update an existing Bevy App or Plugin to **Bevy 0.13**, check out our [0.12 to 0.13 Migration Guide](/learn/migration-guides/0.12-0.13/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **First-party primitive shapes:** basic shapes are a core building block of both game engines and video games: we've added a polished collection of them for you to use!
* **System stepping:** completely pause and advance through your game frame-by-frame or system-by-system to interactively debug game logic, all while rendering continues to update.
* **Dynamic queries:** refining queries from within systems is extremely expressive, and is the last big puzzle piece for runtime-defined types and third-party modding and scripting integration.
* **Automatically inferred command flush points:** tired of reasoning about where to put `apply_deferred` and confused about why your commands weren't being applied? Us too! Now, Bevy's scheduler uses ordinary `.before` and `.after` constraints and inspects the system parameters to automatically infer (and deduplicate) synchronization points.
* **Slicing, tiling and nine-patch sprites:** ninepatch layout is a popular tool for smoothly scaling stylized tilesets and UIs. Now in Bevy!
* **Lightmaps:** the first step towards baked global illumination: a fast, popular and pretty lighting technique.
* **Animation interpolation modes:** Bevy now supports non-linear interpolation modes in exported glTF animations.

## Primitive Shapes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## System Stepping

<div class="release-feature-authors">authors: @TODO</div>

## Dynamic Queries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Entity Optimizations

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## `WorldQuery` Trait Split

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Automatically Insert `apply_deferred` Systems

<div class="release-feature-authors">authors: @hymm</div>

A common scheduling issue is that one system needs to see the effects of commands
queued in another system. Before 0.13, you would have to manually insert an
`apply_deferred` system between the two. Bevy now detects when a system with commands
is ordered relative to other systems and inserts the `apply_deferred` for you.

```rust
// Before 0.13
app.add_systems(
    Update,
    (
        system_with_commands,
        apply_deferred,
        another_system,
    ).chain()
);
```

```rust
// After 0.13
app.add_systems(
    Update,
    (
        system_with_commands,
        another_system,
    ).chain()
);
```

It also optimizes the automatically inserted `apply_deferred` systems by merging them if
possible. In most cases, it is recommended to remove all manually inserted
`apply_deferred` systems, as allowing Bevy to insert and merge these systems as needed will
usually be faster.

```rust
// This will only add one apply_deferred system.
app.add_systems(
    Update,
    (
        (system_1_with_commands, system_2).chain(),
        (system_3_with_commands, system_4).chain(),
    )
);
```

If this new behavior does not work for you, please consult the migration guide.
There are several new APIs that allow you to opt-out.

## Input for One-Shot Systems

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## WGPU Upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Texture Atlas Rework

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Sprite Slicing and Tiling

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Exposure Settings

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Minimal Reflection Probes

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light Maps

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Light `RenderLayers`

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Approximate Indirect Specular Occlusion

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Unload Render Assets From RAM

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Bind Group Layout Entries

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Type-Safe Labels for the `RenderGraph`

<div class="release-feature-authors">authors: @bardt, @oceantume</div>

Historically, Bevy's UI elements have been scaled and positioned in the context of the primary window, regardless of the camera settings. This approach made some UI experiences like split-screen multiplayer difficult to implement, and others such as having UI in multiple windows impossible.

We are now introducing a more flexible way of rendering the user interface: Camera-driven UI. Each camera can now have its own UI root, rendering according to its viewport, scale factor, and a target which can be a secondary window or even a texture.

This change unlocks a variety of new UI experiences, including split-screen multiplayer, UI in multiple windows, displaying non-interactive UI in a 3D world, and more.

![Split-screen with independent UI roots](split-screen.png)

If there is one camera in the world, you don't need to do anything; your UI will be displayed in that camera's viewport.

```rust
commands.spawn(Camera3dBundle {
    // Camera can have custom viewport, target, etc.
});
commands.spawn(NodeBundle {
    // UI will be rendered to the singular camera's viewport
});
```

For when more control is desirable, or there are multiple cameras, we introduce [`TargetCamera`] component. This component can be added to a root UI node to specify which camera it should be rendered to.

```rust
// For split-screen multiplayer, we set up 2 cameras and 2 UI roots
let left_camera = commands.spawn(Camera3dBundle {
    // Viewport is set to left half of the screen
}).id();

commands
    .spawn((
        TargetCamera(left_camera),
        NodeBundle {
            //...
        }
    ));

let right_camera = commands.spawn(Camera3dBundle {
    // Viewport is set to right half of the screen
}).id();

commands
    .spawn((
        TargetCamera(right_camera),
        NodeBundle {
            //...
        })
    );
```

With this change, we also remove [`UiCameraConfig`] component. If you were using it to hide UI nodes, you can achieve the same outcome by setting [`Visibility`] component on the root node.

```rust
commands.spawn(Camera3dBundle::default());
commands.spawn(NodeBundle {
    visibility: Visibility::Hidden, // UI will be hidden
    // ...
});
```

[`TargetCamera`]: https://docs.rs/bevy/0.13.0/bevy/ui/struct.TargetCamera.html
[`Visibility`]: https://docs.rs/bevy/0.13.0/bevy/render/view/enum.Visibility.html
[`UiCameraConfig`]: https://docs.rs/bevy/0.12.1/bevy/ui/camera_config/struct.UiCameraConfig.html

## Camera-Driven UI

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Winit Upgrade

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Animation Interpolation

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## `Animatable` Trait

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## glTF Extensions

<div class="release-feature-authors">authors: @TODO</div>

TODO.

## Extensionless Asset Support

<div class="release-feature-authors">authors: @bushrat011899</div>

In prior versions of Bevy, the default way to choose an [`AssetLoader`] for a particular asset was entirely based around file extensions. The recent addition of [`meta` files] allowed for specifying more granular loading behavior, but file extensions were still required. In Bevy 0.13, the asset type can now be used to infer the [`AssetLoader`].

```rust
// Uses AudioSettingsAssetLoader
let audio = asset_server.load("data/audio.json");

// Uses GraphicsSettingsAssetLoader
let graphics = asset_server.load("data/graphics.json");
```

This is possible because every [`AssetLoader`] is required to declare what **type** of asset it loads, not just the extensions it supports. Since the [`load`] method on [`AssetServer`] was already generic over the type of asset to return, this information is already available to the [`AssetServer`] so that the appropriate [`Handle`] type is returned.

```rust
// The above example with types shown
let audio: Handle<AudioSettings> = asset_server.load::<AudioSettings>("data/audio.json");
let graphics: Handle<GraphicsSettings> = asset_server.load::<GraphicsSettings>("data/graphics.json");
```

Now we can also use it to choose the [`AssetLoader`] itself.

```rust
#[derive(Resource)]
struct Settings {
    audio: Handle<AudioSettings>,
//                ^^^^^^^^^^^^^
//                | This type...
}

fn setup(mut settings: ResMut<Settings>, asset_server: Res<AssetServer>) {
    settings.audio = asset_server.load("data/audio.json");
//                                ^^^^
//                                | ...is passed here...
}

impl AssetLoader for AudioSettingsAssetLoader {
    type Asset = AudioSettings;
//               ^^^^^^^^^^^^^
//               | ...and checked against AssetLoader::Asset

    /* snip */
}
```

When loading an asset, the loader is chosen by checking (in order):

1. The asset `meta` file
2. The type of `Handle<A>` to return
3. The file extension

```rust
// This will be inferred from context to be a glTF asset, ignoring the file extension
let gltf_handle = asset_server.load("models/cube/cube.gltf");

// This still relies on file extension due to the label
let cube_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
//                                                        ^^^^^^^^^^^^^^^^^
//                                                        | Asset path label
```

### File extensions are now optional

Since the asset type can be used to infer the loader, neither the file to be loaded nor the [`AssetLoader`] need to have file extensions.

```rust
pub trait AssetLoader: Send + Sync + 'static {
    /* snip */

    /// Returns a list of extensions supported by this [`AssetLoader`], without the preceding dot.
    fn extensions(&self) -> &[&str] {
        // A default implementation is now provided
        &[]
    }
}
```

Previously, an asset loader with no extensions was very cumbersome to use. Now, they can be used just as easily as any other loader. Likewise, if a file is missing its extension, Bevy can now choose the appropriate loader.

```rust
let license = asset_server.load::<Text>("LICENSE");
```

Appropriate file extensions are still recommended for good project management, but this is now a recommendation rather than a hard requirement.

### Multiple `AssetLoader`'s can be selected for the same asset

Now, a single path can be used by multiple asset handles as long as they are distinct asset types.

```rust
// Load the sound effect for playback
let bang = asset_server.load::<AudioSource>("sound/bang.ogg");

// Load the raw bytes of the same sound effect (e.g, to send over the network)
let bang_blob = asset_server.load::<Blob>("sound/bang.ogg");

// Returns the bang handle since it was already loaded
let bang_again = asset_server.load::<AudioSource>("sound/bang.ogg");
```

Note that the above example uses [turbofish] syntax for clarity. In practice, it's not required, since the type of asset loaded can usually be inferred by surrounding context at the call site.

```rust
#[derive(Resource)]
struct SoundEffects {
    bang: Handle<AudioSource>,
    bang_blog: Handle<Blob>,
}

fn setup(mut effects: ResMut<SoundEffects>, asset_server: Res<AssetServer>) {
    effects.bang = asset_server.load("sound/bang.ogg");
    effects.bang_blob = asset_server.load("sound/bang.ogg");
}
```

### More information

The [`custom_asset` example] has been updated to demonstrate these new features.

[`meta` files]: https://bevyengine.org/news/bevy-0-12/#asset-meta-files
[`AssetServer`]: https://dev-docs.bevyengine.org/bevy/asset/struct.AssetServer.html
[`AssetLoader`]: https://dev-docs.bevyengine.org/bevy/asset/trait.AssetLoader.html
[`load`]: https://dev-docs.bevyengine.org/bevy/asset/struct.AssetServer.html#method.load
[`Handle`]: https://dev-docs.bevyengine.org/bevy/asset/enum.Handle.html
[turbofish]: https://turbo.fish/
[`custom_asset` example]: https://bevyengine.org/examples/Assets/custom-asset/

## Gizmo Configuration

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
