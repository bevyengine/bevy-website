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

<div class="release-feature-authors">authors: @TODO</div>

TODO.

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

## Type-safe labels for the `RenderGraph`

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

## Extensionless asset support

<div class="release-feature-authors">authors: @bushrat011899</div>

### Some background

In Bevy 0.12 and earlier, all assets needed a file extension which uniquely mapped to a single asset type. Conceptually, this makes sense: a `.json` file should be loaded by the `JsonLoader` as a `JsonFile` asset.

```rust
let settings = asset_server.load("data/settings.json");
```

Peeking behind the curtain, this is achieved by passing the **path** to the [`AssetServer`], which selects an [`AssetLoader`] based on the file extension. This is possible because every [`AssetLoader`] is required to declare up-front what file types it supports.

```rust
impl AssetLoader for JsonAssetLoader {
    /* snip */

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
```

But, what happens when you have two different asset types use the same extension? Consider an `AudioSettings` asset and a `GraphicsSettings` asset: both will be stored as JSON files, but they need to parsed as different asset types.

```rust
impl AssetLoader for AudioSettingsAssetLoader {
    type Asset = AudioSettings;

    /* snip */

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}

impl AssetLoader for GraphicsSettingsAssetLoader {
    type Asset = GraphicsSettings;

    /* snip */

    fn extensions(&self) -> &[&str] {
        &["json"]
    }
}
```

Previously, Bevy would pick one of the above asset loaders to be the canonical `json` loader based on registration order.

```rust
app.init_asset_loader::<AudioSettingsAssetLoader>();

// All JSON files will be loaded by AudioSettingsAssetLoader

app.init_asset_loader::<GraphicsSettingsAssetLoader>();

// All JSON files will now be loaded by GraphicsSettingsAssetLoader
```

The workaround for this is to create a compound file extension for each unique asset type, such as `.audio.json` and `.graphics.json`.

### Relaxing constraints

But we can do better! In Bevy 0.13, the asset type can now used to infer the [`AssetLoader`].

```rust
// Uses AudioSettingsAssetLoader
let audio_settings = asset_server.load("data/audio.json");

// Uses GraphicsSettingsAssetLoader
let graphics_settings = asset_server.load("data/graphics.json");
```

This is possible because every [`AssetLoader`] is also required to declare what **type** of asset it loads, not just the extensions it supports. Since the [`load`] method on [`AssetServer`] was already generic over the type of asset to return, this information was already available to the [`AssetServer`]. All that was missing was connecting this type information that was available on both sides of the [`AssetServer`] together!

```rust
let audio_settings = asset_server.load("data/audio.json");
//  ^^^^^^^^^^^^^^                ^^^^
//  | This has the type           |
//  | Handle<AudioSettings>       |
//                                | Which is passed into load...

impl AssetLoader for AudioSettingsAssetLoader {
    type Asset = AudioSettings;
//               ^^^^^^^^^^^^^
//               | ...and checked against AssetLoader::Asset

    /* snip */
}
```

Now, when loading, first the [`AssetServer`] attempts to infer type of asset by the type of [`Handle`] to return. If that can't be done (for example, labelled paths are permitted to return different asset types), then as a fallback the file extension is checked just like before.

```rust
// This will be inferred from context to be a Gltf asset, ignoring the file extension
let gltf_handle = asset_server.load("models/cube/cube.gltf");

// This still relies on file extension due to the label
let cube_handle = asset_server.load("models/cube/cube.gltf#Mesh0/Primitive0");
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
fn setup(mut effects: ResMut<SoundEffects>, asset_server: Res<AssetServer>) {
    effects.bang = asset_server.load("sound/bang.ogg");
    effects.bang_blob = asset_server.load("sound/bang.ogg");
}
```

### More information

The [`custom_asset` example] has been updated to demonstrate these new features.

[`AssetServer`]: https://dev-docs.bevyengine.org/bevy/asset/struct.AssetServer.html
[`AssetLoader`]: https://dev-docs.bevyengine.org/bevy/asset/trait.AssetLoader.html
[`load`]: https://dev-docs.bevyengine.org/bevy/asset/struct.AssetServer.html#method.load
[`Handle`]: https://dev-docs.bevyengine.org/bevy/asset/enum.Handle.html
[turbofish]: https://turbo.fish/
[`custom_asset` example]: https://bevyengine.org/examples/Assets/custom-asset/

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
