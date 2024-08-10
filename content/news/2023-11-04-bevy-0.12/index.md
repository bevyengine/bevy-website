+++
title = "Bevy 0.12"
date = 2023-11-04
[extra]
image = "cover.gif"
show_image = true
image_subtitle = "A relaxing 'tiny planet' city builder by Jos Feenstra (made with Bevy)"
image_subtitle_link = "https://twitter.com/i_am_feenster"

+++

Thanks to **185** contributors, **567** pull requests, community reviewers, and our [**generous sponsors**](/donate), we're happy to announce the **Bevy 0.12** release on [crates.io](https://crates.io/crates/bevy)!

For those who don't know, Bevy is a refreshingly simple data-driven game engine built in Rust. You can check out our [Quick Start Guide](/learn/quick-start/introduction) to try it today. It's free and open source forever! You can grab the full [source code](https://github.com/bevyengine/bevy) on GitHub. Check out [Bevy Assets](https://bevyengine.org/assets) for a collection of community-developed plugins, games, and learning resources.

To update an existing Bevy App or Plugin to **Bevy 0.12**, check out our [0.11 to 0.12 Migration Guide](/learn/migration-guides/0.11-0.12/).

Since our last release a few months ago we've added a _ton_ of new features, bug fixes, and quality of life tweaks, but here are some of the highlights:

<!-- more -->

* **Deferred Rendering**: (Optional) support for rendering in a Deferred style, which complements Bevy's existing Forward+ renderer by adding support for new effects and different performance tradeoffs. Bevy is now a "hybrid" renderer, meaning you can use both at the same time!
* **Bevy Asset V2**: A brand new asset system that adds support for asset preprocessing, asset configuration (via .meta files), multiple asset sources, recursive dependency load tracking, and more!
* **PCF Shadow Filtering**: Bevy now has smoother shadows thanks to Percentage-Closer Filtering.
* **StandardMaterial Light Transmission**: Bevy's PBR material now supports light transmission, making it possible to simulate materials like glass, water, plastic, foliage, paper, wax, marble, etc.  
* **Material Extensions**: Materials can now build on other materials. You can now easily write shaders that build on existing materials, such as Bevy's PBR StandardMaterial.
* **Rusty Shader Imports**: Bevy's granular shader import system now uses Rust-style imports, expanding the capabilities and usability of the import system.
* **Suspend and Resume on Android**: Bevy now supports suspend and resume events on Android, which was the last big missing piece in our Android story. Bevy now supports Android!
* **Automatic Batching and Instancing of Draw Commands**: Draw commands are now automatically batched / instanced when possible, yielding significant render performance wins.
* **Renderer Optimizations**: Bevy's renderer dataflow has been reworked to squeeze out more performance and prepare the way for future GPU-driven rendering.
* **One Shot Systems**: ECS Systems can now be run on-demand from other systems!
* **UI Materials**: Add custom material shaders to Bevy UI nodes.

## Deferred Rendering

<div class="release-feature-authors">authors: @DGriffin91</div>

The two most popular "rendering styles" are:

* **Forward Rendering**: do all material/lighting calculations in a single render pass
  * **Pros**: Simpler to work with. Works on / performs better on more hardware. Supports MSAA. Handles transparency nicely.
  * **Cons**: Lighting is more expensive / fewer lights supported in a scene, some rendering effects are impossible (or harder) without a prepass
* **Deferred Rendering**: do one or more pre-passes that collect relevant information about a scene, then do material/lighting calculations in _screen space_ in a final pass after that.
  * **Pros**: Enables some rendering effects that are not possible in forward rendering. This is especially important for GI techniques, cuts down on shading cost by only shading visible fragments, can support more lights in a scene
  * **Cons**: More complicated to work with. Requires doing prepasses, which can be more expensive than an equivalent forward renderer in some situations (although the reverse can also be true), uses more texture bandwidth (which can be prohibitive on some devices), doesn't support MSAA, transparency is harder / less straightforward.

Bevy's renderer has historically been a "forward renderer". More specifically, it is a [Clustered Forward / Forward+](/news/bevy-0-7/#unlimited-point-lights) renderer, which means we break the view frustum up into clusters and assign lights to those clusters, allowing us to render many more lights than a traditional forward renderer.

However, as Bevy has grown, it has slowly moved into "hybrid renderer" territory. In previous releases, we added a [Depth and Normal Prepass](/news/bevy-0-10/#depth-and-normal-prepass) to enable [TAA](/news/bevy-0-11/#temporal-anti-aliasing), [SSAO](/news/bevy-0-11/#screen-space-ambient-occlusion), and [Alpha Texture Shadow Maps](/news/bevy-0-10/#shadow-mapping-using-prepass-shaders). We also added a Motion Vector Prepass to enable TAA.

In **Bevy 0.12** we added optional support for Deferred Rendering (building on the existing prepass work). Each material can choose whether it will go through the forward or deferred path, and this can be configured per-material-instance. Bevy also has a new [`DefaultOpaqueRendererMethod`] resource, which configures the global default. This is set to "forward" by default. The global default can be overridden per-material.

Lets break down the components of this deferred render:

![deferred](deferred.png)

When deferred is enabled for the PBR [`StandardMaterial`], the deferred prepass packs PBR information into the Gbuffer, which can be broken up into:

**Base Color**
![base color](base_color.png)

**Depth**
![depth](depth.png)

**Normals**
![normals](normals.png)

**Perceptual Roughness**
![perceptual roughness](perceptual_roughness.png)

**Metallic**
![metallic](metallic.png)

The deferred prepass also produces a "deferred lighting pass ID" texture, which determines what lighting shader to run for the fragment:

![lighting pass ID texture](deferred_pass2.png)

These are passed into the final deferred lighting shader.

Note that the cube in front of the flight helmet model and the ground plane are using forward rendering, which is why they are black in both of the deferred lighting textures above. This illustrates that you can use both forward and deferred materials in the same scene!

Note that for most use cases, we recommend using forward by default, unless a feature explicitly needs deferred or your rendering conditions benefit from deferred style. Forward has the fewest surprises and will work better on more devices.

[`StandardMaterial`]: https://docs.rs/bevy/0.12.0/bevy/pbr/struct.StandardMaterial.html
[`DefaultOpaqueRendererMethod`]: https://docs.rs/bevy/0.12.0/bevy/pbr/struct.DefaultOpaqueRendererMethod.html

## PCF Shadow Filtering

<div class="release-feature-authors">authors: @superdump (Rob Swain), @JMS55</div>

Shadow aliasing is a very common problem in 3D apps:

![no pcf](no_pcf.png)

Those "jagged lines" in the shadow are the result of the shadow map being "too small" to accurately represent the shadow from this perspective. The shadow map above is stored in a 512x512 texture, which is a lower resolution than most people will use for most of their shadows. This was selected to show a "bad" case of jaggies. Note that Bevy defaults to 2048x2048 shadowmaps.

One "solution" is to bump up the resolution. Here is what it looks like with a 4096x4096 shadow map.

![no pcf high resolution](no_pcf_high.png)

Looking better! However this still isn't a perfect solution. Large shadowmaps aren't feasible on all hardware. They are significantly more expensive. And even if you can afford super high resolution shadows, you can still encounter this issue if you place an object in the wrong place, or point your light in the wrong direction. You can use Bevy's [Cascaded Shadow Maps](/news/bevy-0-10/#cascaded-shadow-maps) (which are enabled by default) to cover a larger area, with higher detail close to the camera and less detail farther away. However even under these conditions, you will still probably encounter these aliasing issues.

**Bevy 0.12** introduces **PCF Shadow Filtering** (Percentage-Closer Filtering), which is a popular technique that takes multiple samples from the shadow map and compares with an interpolated mesh surface depth-projected into the frame of reference of the light. It then calculates the percentage of samples in the depth buffer that are closer to the light than the mesh surface. In short, this creates a "blur" effect that improves shadow quality, which is especially evident when a given shadow doesn't have enough "shadow map detail". Note that PCF is currently only supported for [`DirectionalLight`] and [`SpotLight`].

**Bevy 0.12**'s default PCF approach is the [`ShadowMapFilter::Castano13`] method by Ignacio Castaño (used in The Witness). Here it is with a 512x512 shadow map:

{{ compare_slider(
    left_title="Castano 13 PCF",
    left_image="pcf_castano.png",
    right_title="PCF Off",
    right_image="no_pcf.png"
) }}

Much better!

We also implemented the [`ShadowMapFilter::Jimenez14`] method by Jorge Jimenez (used in Call of Duty Advanced Warfare). This can be slightly cheaper than Castano, but it can flicker. It benefits from [Temporal Anti-Aliasing (TAA)](/news/bevy-0-11/#temporal-anti-aliasing) which can reduce the flickering. It can also blend shadow cascades a bit more smoothly than Castano.

{{ compare_slider(
    left_title="Jimenez 14 PCF",
    left_image="pcf_jimenez.png",
    right_title="PCF Off",
    right_image="no_pcf.png"
) }}

[`ShadowMapFilter::Castano13`]: https://docs.rs/bevy/0.12.0/bevy/pbr/enum.ShadowFilteringMethod.html#variant.Castano13
[`ShadowMapFilter::Jimenez14`]: https://docs.rs/bevy/0.12.0/bevy/pbr/enum.ShadowFilteringMethod.html#variant.Jimenez14
[`DirectionalLight`]: https://docs.rs/bevy/0.12.0/bevy/pbr/struct.DirectionalLight.html
[`SpotLight`]: https://docs.rs/bevy/0.12.0/bevy/pbr/struct.SpotLight.html

## `StandardMaterial` Light Transmission

<div class="release-feature-authors">author: Marco Buono (@coreh)</div>

The [`StandardMaterial`] now supports a number of light transmission-related properties:

* `specular_transmission`
* `diffuse_transmission`
* `thickness`
* `ior`
* `attenuation_color`
* `attenuation_distance`

These allow you to more realistically represent a wide variety of physical materials, including **clear and frosted glass, water, plastic, foliage, paper, wax, marble, porcelain and more**.

Diffuse transmission is an inexpensive addition to the PBR lighting model, while specular transmission is a somewhat more resource-intensive screen-space effect, that can accurately model refraction and blur effects.

![transmission](transmission.jpg)

<div style="font-size: 1.0rem" class="release-feature-authors">
    Different light transmission properties and their interactions with existing PBR properties.
</div>

To complement the new transmission properties, a new [`TransmittedShadowReceiver`] component has been introduced, which can be added to entities with diffuse transmissive materials to receive shadows cast from the opposite side of the mesh. This is most useful for rendering thin, two-sided translucent objects like tree leaves or paper.

Additionally, two extra fields have been added to the [`Camera3d`] component: `screen_space_specular_transmission_quality` and `screen_space_specular_transmission_steps`. These are used to control the quality of the screen-space specular transmission effect (number of taps), and how many “layers of transparency” are supported when multiple transmissive objects are in front of each other.

> **Important:** Each additional “layer of transparency” incurs in a texture copy behind the scenes, adding to the bandwidth cost, so it's recommended to keep this value as low as possible.

Finally, importer support for the following glTF extensions has been added:

* `KHR_materials_transmission`
* `KHR_materials_ior`
* `KHR_materials_volume`

[Check out this video](https://www.youtube.com/watch?v=t1XdxZKZ-us) to see it in action!

### Compatibility

Both specular and diffuse transmission are compatible with all supported platforms, including mobile and Web.

The optional `pbr_transmission_textures` Cargo feature allows using textures to modulate the `specular_transmission`, `diffuse_transmission` and `thickness` properties. It's disabled by default in order to reduce the number of texture bindings used by the standard material. (These are
severely constrained on lower-end platforms and older GPUs!)

[`DepthPrepass`] and TAA can greatly improve the quality of the screen-space specular transmission effect, and are recommended to be used with it, on the platforms where they are supported.

### Implementation Details

Specular transmission is implemented via a new `Transmissive3d` screen space refraction phase, which joins the existing `Opaque3d`, `AlphaMask3d` and `Transparent3d` phases. During this phase, one or more snapshots of the main texture are taken, which are used as “backgrounds” for the refraction effects.

Each fragment's surface normal and IOR (index of refraction) used along with the view direction to calculate a refracted ray. (Via Snell's law.)
This ray is then propagated through the mesh's volume (by a distance controlled by the `thickness` property), producing an exit point.
The “background” texture is then sampled at that point. Perceptual roughness is used along with interleaved gradient noise and multiple spiral taps, to produce a blur effect.

Diffuse transmission is implemented via a second, reversed and displaced fully-diffuse Lambertian lobe, which is added to the existing PBR lighting calculations. This is a simple and relatively cheap approximation, but works reasonably well.

[`TransmittedShadowReceiver`]: https://docs.rs/bevy/0.12.0/bevy/pbr/struct.TransmittedShadowReceiver.html
[`Camera3d`]: https://docs.rs/bevy/0.12.0/bevy/core_pipeline/core_3d/struct.Camera3d.html
[`DepthPrepass`]: https://docs.rs/bevy/0.12.0/bevy/core_pipeline/prepass/struct.DepthPrepass.html

## Bevy Asset V2

<div class="release-feature-authors">authors: @cart</div>

Asset pipelines are a central part of the gamedev process. Bevy's old asset system was suitable for some classes of app, but it had a number of limitations that prevented it from serving the needs of other classes, especially higher end 3D apps.

Bevy Asset V2 is a completely new asset system that learns from the best parts of Bevy Asset V1 while adding support for a number of important scenarios: **Asset Importing/Preprocessing**, **Asset Meta Files**, **Multiple Asset Sources**, **Recursive Asset Dependency Load Events**, **Async Asset I/O**, **Faster and More Featureful Asset Handles**, and more!

Most existing user-facing asset code will either require no changes at all, or minimal changes. Custom [`AssetLoader`] or [`AssetReader`] code will need to change slightly, but generally the changes should be very minimal. Bevy Asset V2 (despite being a completely new implementation) largely just expands what Bevy is capable of.

[`AssetLoader`]: https://docs.rs/bevy/0.12.0/bevy/asset/trait.AssetLoader.html
[`AssetReader`]: https://docs.rs/bevy/0.12.0/bevy/asset/io/trait.AssetReader.html

### Asset Preprocessing

<img alt="image process diagram" src="bevy-0-12/image_process.png" class="inverted">

Asset preprocessing is the ability to take an input asset of a given type, process it in some way (generally during development time), and then use the result as the final asset in your application. Think of it as an "asset compiler".

This enables a number of scenarios:

* **Reduce Work In Released Apps**: Many assets aren't _composed_ in their ideal form for release. Scenes might be defined in a human-readable text format that is slower to load. Images might be defined in formats that require more work to decode and upload to the GPU, or take up more space on the GPU when compared to GPU-friendly formats (ex: PNG images vs Basis Universal). Preprocessing enables developers to convert to release-optimal formats ahead of time, making apps start up faster, take up fewer resources, and perform better. It also enables moving computation work that _would_ have been done at runtime to development time. For example, generating mipmaps for images.
* **Compression**: Minimize the disk space and/or bandwidth that an asset takes up in deployed apps
* **Transformation**: Some "asset source files" aren't in the right format by default. You can have an asset of type `A` and transform it into type `B`.

If you are building an app that tests the limits of your hardware with optimal formats ... or you just want to cut down on startup / loading times, asset preprocessing is for you.

For an in-depth technical breakdown of the implementation we chose, check out the [Bevy Asset V2 pull request](https://github.com/bevyengine/bevy/pull/8624).

### Enabling Pre-Processing

To enable asset pre-processing, just configure your [`AssetPlugin`] like this:

```rust
app.add_plugins(DefaultPlugins.set(
    AssetPlugin {
        mode: AssetMode::Processed,
        ..default()
    }
))
```

This will configure the asset system to look for assets in the `imported_assets` folder instead of the `assets` "source folder". During development, enable the `asset_processor` cargo feature flag like this:

```sh
cargo run --features bevy/asset_processor
```

This will start the [`AssetProcessor`] in parallel with your app. It will run until all assets are read from their source (defaults to the `assets` folder), processed, and the results have been written to their destination (defaults to the `imported_assets` folder). This pairs with asset hot-reloading. If you make a change, this will be detected by the [`AssetProcessor`], the asset will be reprocessed, and the result will be hot-reloaded in your app.

[`AssetPlugin`]: https://docs.rs/bevy/0.12.0/bevy/asset/struct.AssetPlugin.html
[`AssetProcessor`]: https://docs.rs/bevy/0.12.0/bevy/asset/processor/struct.AssetProcessor.html

### Should You Enable Pre-Processing Today?

In future Bevy releases we will recommended enabling processing for the majority of apps. We don't _yet_ recommend it for most use cases for a few reasons:

1. Most of our built-in assets don't have processors implemented for them yet. The [`CompressedImageSaver`] is the only built-in processor and it has a bare-minimum set of features.
2. We have not implemented "asset migrations" yet. Whenever an asset changes its settings format (which is used in meta files), we need to be able to automatically migrate existing asset meta files to the new version.
3. As people adopt processing, we expect some flux as we respond to feedback.

### Incremental and Dependency Aware

**Bevy Asset V2** will only process assets that have changed. To accomplish this, it computes and stores hashes of each asset source file:

```rust
hash: (132, 61, 201, 41, 85, 80, 72, 189, 132, 81, 252, 156, 4, 227, 196, 207),
```

It also tracks the asset dependencies used when processing an asset. If a dependency has changed, the dependant asset will also be re-processed!

### Transactional and Reliable

**Bevy Asset V2** uses write-ahead logging (a technique commonly used by databases) to recover from crashes / forced exists. Whenever possible it avoids full-reprocessing and only reprocesses incomplete transactions.

The [`AssetProcessor`] can close (either intentionally or unintentionally) at any point in time and it will pick up right where it left off!

If a Bevy App asks to load an asset that is currently being processed (or re-processed), the load will asynchronously wait until both the processed asset and its meta file have been written. This ensures that a loaded asset file and meta file always "match" for a given load.

### Asset Meta Files

Assets now support (optional) `.meta` files. This enables configuring things like:

* **The asset "action"**
  * This configures how Bevy's asset system should handle the asset:
    * `Load`: Load the asset without processing
    * `Process`: Pre-process the asset prior to loading
    * `Ignore`: Do not process or load the asset
* **[`AssetLoader`] settings**
  * You can use meta files to set any [`AssetLoader`] you want
  * Configure loader settings like "how to filter an image", "adjusting the up axis in 3D scenes", etc
* **[`Process`] settings** (if using the `Process` action)
  * You can use meta files to set any [`Process`] implementation you want
  * Configure processor settings like "what type of compression to use", "whether or not to generate mipmaps", etc

A meta file for an unprocessed image looks like this:

```rust
(
    meta_format_version: "1.0",
    asset: Load(
        loader: "bevy_render::texture::image_loader::ImageLoader",
        settings: (
            format: FromExtension,
            is_srgb: true,
            sampler: Default,
        ),
    ),
)
```

A meta file for an image configured to be processed looks like this:

```rust
(
    meta_format_version: "1.0",
    asset: Process(
        processor: "bevy_asset::processor::process::LoadAndSave<bevy_render::texture::image_loader::ImageLoader, bevy_render::texture::compressed_image_saver::CompressedImageSaver>",
        settings: (
            loader_settings: (
                format: FromExtension,
                is_srgb: true,
                sampler: Default,
            ),
            saver_settings: (),
        ),
    ),
)
```

If the asset processor is enabled, meta files will be automatically generated for assets.

The final "output" metadata for the processed image looks like this:

```rust
(
    meta_format_version: "1.0",
    processed_info: Some((
        hash: (132, 61, 201, 41, 85, 80, 72, 189, 132, 81, 252, 156, 4, 227, 196, 207),
        full_hash: (81, 90, 244, 190, 16, 134, 202, 154, 3, 211, 78, 199, 216, 21, 132, 216),
        process_dependencies: [],
    )),
    asset: Load(
        loader: "bevy_render::texture::image_loader::ImageLoader",
        settings: (
            format: Format(Basis),
            is_srgb: true,
            sampler: Default,
        ),
    ),
)
```

This is what is written to the `imported_assets` folder.

Note that the `Process` asset mode has changed to `Load`. This is because in the released app, we will load the final processed image "normally" like any other image asset. Note that in this case, the input and the output asset _both_ use [`ImageLoader`]. However the processed asset _can_ use a different loader if the context demands it. Also note the addition of the `processed_info` metadata, which is used to determine if an asset needs to be reprocessed.

The final processed asset and metadata files can be viewed and interacted with like any other file. However they are intended to be read-only. Configuration should happen on the _source asset_, not the _final processed asset_.

[`Process`]: https://docs.rs/bevy/0.12.0/bevy/asset/processor/trait.Process.html
[`ImageLoader`]: https://docs.rs/bevy/0.12.0/bevy/render/texture/struct.ImageLoader.html

### `CompressedImageSaver`

![processed sponza](processed_sponza.png)

<div style="font-size: 1.0rem" class="release-feature-authors">Sponza scene with textures processed into Basis Universal (with mipmaps) using Bevy Asset V2</div>

**Bevy 0.12** ships with a barebones [`CompressedImageSaver`] that writes images to [Basis Universal](https://github.com/BinomialLLC/basis_universal) (a GPU-friendly image interchange format) and generates [mipmaps](https://en.wikipedia.org/wiki/Mipmap). Mipmaps reduce aliasing artifacts when sampling images from different distances. This fills an important gap, as Bevy previously had no way to generate mipmaps on its own (it relied on external tooling). This can be enabled with the `basis-universal` cargo feature.

[`CompressedImageSaver`]: https://docs.rs/bevy/0.12.0/bevy/render/texture/struct.CompressedImageSaver.html

### Preprocessing is Optional!

Despite eventually ([in future Bevy releases](#should-you-enable-pre-processing-today)) recommending that most people enable asset processing, we also acknowledge that Bevy is used in a variety of applications. Asset processing introduces additional complexity and workflow changes that some people will not want!

This is why Bevy offers two asset modes:

* [`AssetMode::Unprocessed`]: Assets will be loaded directly from the asset source folder (defaults to `assets`) without any preprocessing. They are assumed to be in their "final format". This is the mode/workflow Bevy users are currently used to.
* [`AssetMode::Processed`]: Assets will be pre-processed at development time. They will be read from their source folder (defaults to `assets`) and then written to their destination folder (defaults to `imported_assets`).

To enable this, Bevy uses a novel approach to assets: the difference between a processed and unprocessed asset is perspective. They both use the same `.meta` format and they use the same [`AssetLoader`] interface.

A [`Process`] implementation can be defined using arbitrary logic, but we heavily encourage using the [`LoadAndSave`] [`Process`] implementation. [`LoadAndSave`] takes any [`AssetLoader`] and passes the results to an [`AssetSaver`].

That means if you already have an [`ImageLoader`], which loads images, all you need to do is write some `ImageSaver` which will write the image in some optimized format. This both saves development work and makes it easy to support both processed and unprocessed scenarios.

[`AssetMode::Unprocessed`]: https://docs.rs/bevy/0.12.0/bevy/asset/enum.AssetMode.html
[`AssetMode::Processed`]: https://docs.rs/bevy/0.12.0/bevy/asset/enum.AssetMode.html
[`LoadAndSave`]: https://docs.rs/bevy/0.12.0/bevy/asset/processor/struct.LoadAndSave.html
[`AssetSaver`]: https://docs.rs/bevy/0.12.0/bevy/asset/saver/trait.AssetSaver.html

### Built To Run Anywhere

Unlike many other asset processors in the gamedev space, Bevy Asset V2's [`AssetProcessor`] is intentionally architected to run on any platform. It doesn't use platform-limited databases or require the ability/permission to run a networked server. It can be deployed alongside a released app if your application logic requires processing at runtime.

One notable exception: we still need to make a few changes before it can run on the web, but it was built with web support in mind.

### Recursive Asset Dependency Load Events

The [`AssetEvent`] enum now has an [`AssetEvent::LoadedWithDependencies`] variant. This is emitted when an [`Asset`], its dependencies, and all descendant / recursive dependencies have loaded.

This makes it easy to wait until an [`Asset`] is "fully loaded" before doing something.

[`AssetEvent`]: https://docs.rs/bevy/0.12.0/bevy/asset/enum.AssetEvent.html
[`AssetEvent::LoadedWithDependencies`]: https://docs.rs/bevy/0.12.0/bevy/asset/enum.AssetEvent.html

### Multiple Asset Sources

It is now possible to register more than one [`AssetSource`] (which replaces the old monolithic "asset provider" system).

Loading from the "default" [`AssetSource`] looks exactly like it does in previous Bevy versions:

```rust
sprite.texture = assets.load("path/to/sprite.png");
```

But in **Bevy 0.12** you can now register named [`AssetSource`] entries. For example you could register a `remote` [`AssetSource`] that loads assets from an HTTP server:

```rust
sprite.texture = assets.load("remote://path/to/sprite.png");
```

Features like hot-reloading, meta files, and asset processing are supported across all sources.

You can register a new [`AssetSource`] like this:

```rust
// reads assets from the "other" folder, rather than the default "assets" folder
app.register_asset_source(
    // This is the "name" of the new source, used in asset paths.
    // Ex: "other://path/to/sprite.png"
    "other",
    // This is a repeatable source builder. You can configure readers, writers,
    // processed readers, processed writers, asset watchers, etc.
    AssetSource::build()
        .with_reader(|| Box::new(FileAssetReader::new("other")))
    )
)
```

[`AssetSource`]: https://docs.rs/bevy/0.12.0/bevy/asset/io/struct.AssetSource.html

### Embedded Assets

One of the features motivating **Multiple Asset Sources** was improving our "embedded-in-binary" asset loading. The old `load_internal_asset!` approach had a number of issues (see the relevant section in [this PR](https://github.com/bevyengine/bevy/pull/9885)).

The old system looked like this:

```rust
pub const MESH_SHADER_HANDLE: Handle<Shader> = Handle::weak_from_u128(3252377289100772450);

load_internal_asset!(app, MESH_SHADER_HANDLE, "mesh.wgsl", Shader::from_wgsl);
```

This required a lot of boilerplate and didn't integrate cleanly with the rest of the asset system. The [`AssetServer`] was not aware of these assets, hot-reloading required a special-cased second [`AssetServer`], and you couldn't load assets using an [`AssetLoader`] (they had to be constructed in memory). Not ideal!

To prove out the **Multiple Asset Sources** implementation, we built a new `embedded` [`AssetSource`], which replaces the old `load_internal_asset!` system with something that naturally fits into the asset system:

```rust
// Called in `crates/bevy_pbr/src/render/mesh.rs`
embedded_asset!(app, "mesh.wgsl");

// later in the app
let shader: Handle<Shader> = asset_server.load("embedded://bevy_pbr/render/mesh.wgsl");
```

That is a lot less boilerplate than the old approach!

And because the `embedded` source is just like any other asset source, it can support hot-reloading cleanly ... unlike the old system. To hot-reload assets embedded in the binary (ex: to get live updates on a shader you have embedded in the binary), just enable the new `embedded_watcher` cargo feature.

Much better!

### Extendable

Almost everything in **Bevy Asset V2** can be extended with trait impls:

* **[`Asset`]**: Define new asset types
* **[`AssetReader`]**: Define custom [`AssetSource`] read logic
* **[`AssetWriter`]**: Define custom [`AssetSource`] write logic
* **[`AssetWatcher`]**: Define custom [`AssetSource`] watching / hot-reloading logic
* **[`AssetLoader`]**: Define custom load logic for a given [`Asset`] type
* **[`AssetSaver`]**: Define custom save logic for a given [`Asset`] type
* **[`Process`]**: Define fully bespoke processor logic (or use the more opinionated [`LoadAndSave`] [`Process`] impl)

[`Asset`]: https://docs.rs/bevy/0.12.0/bevy/asset/trait.Asset.html
[`AssetWatcher`]: https://docs.rs/bevy/0.12.0/bevy/asset/io/trait.AssetWatcher.html

### Async Asset I/O

The new [`AssetReader`] and [`AssetWriter`] APIs are async! This means naturally async backends (like networked APIs) can directly call `await` on futures.

The filesystem impls (such as [`FileAssetReader`]) offload file IO to a separate thread and the future resolves when the file operation has finished.

[`AssetWriter`]: https://docs.rs/bevy/0.12.0/bevy/asset/io/trait.AssetWriter.html
[`FileAssetReader`]: https://docs.rs/bevy/0.12.0/bevy/asset/io/file/struct.FileAssetReader.html

### Improved Hot-Reloading Workflow

Previous versions of Bevy required manually enabling asset hot-reloading in your app code (in addition to enabling the `filesystem_watcher` cargo feature):

```rust
// Enabling hot reloading in old versions of Bevy
app.add_plugins(DefaultPlugins.set(AssetPlugin::default().watch_for_changes()))
```

This was suboptimal because released versions of apps generally don't want filesystem watching enabled.

In **Bevy 0.12** we've improved this workflow by making the new `file_watcher` cargo feature enable file watching in your app by default. During development, just run your app with the feature enabled:

```sh
cargo run --features bevy/file_watcher
```

When releasing, just omit that feature. No code changes required!

```sh
cargo build --release
```

### Better Asset Handles

Asset handles now use a single [`Arc`] at their core to manage the lifetime of an asset. This simplifies the internals significantly and also enables us to make more asset information available directly from handles.

Notably, in **Bevy 0.12** we use this to provide direct [`AssetPath`] access from the [`Handle`]:

```rust
// Previous version of Bevy
let path = asset_server.get_handle_path(&handle);

// Bevy 0.12
let path = handle.path();
```

Handles now also use a smaller / cheaper-to-look-up [`AssetIndex`] internally, which uses generational indices to look up assets in dense storage.

[`Arc`]: https://doc.rust-lang.org/std/sync/struct.Arc.html
[`AssetPath`]: https://docs.rs/bevy/0.12.0/bevy/asset/struct.AssetPath.html
[`Handle`]: https://docs.rs/bevy/0.12.0/bevy/asset/enum.Handle.html
[`AssetIndex`]: https://docs.rs/bevy/0.12.0/bevy/asset/struct.AssetIndex.html

### True Copy-on-Write Asset Paths

The [`AssetServer`] and [`AssetProcessor`] do a lot of [`AssetPath`] cloning (across many threads). In previous versions of Bevy, [`AssetPath`] was backed by Rust's [`Cow`] type. However in Rust, cloning an "owned" [`Cow`] results in a clone of the internal value. This is _not_ the "clone on write" behavior we want for [`AssetPath`]. We use [`AssetPath`] across threads, so we _need_ to start with an "owned" value.

To prevent all of this cloning and re-allocating of strings, we've built our own [`CowArc`] type, which [`AssetPath`] uses internally. It has two tricks up its sleeve:

1. The "owned" variant is an `Arc<str>`, which we can cheaply clone without reallocating the string.
2. Almost _all_ [`AssetPath`] values defined in code come from a `&'static str`. We've created a special [`CowArc::Static`] variant that retains this static-ness, meaning we do _zero_ allocations even when turning a borrow into an "owned [`AssetPath`]".

[`Cow`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html
[`AssetServer`]: https://docs.rs/bevy/0.12.0/bevy/asset/struct.AssetServer.html
[`CowArc`]: https://docs.rs/bevy/0.12.0/bevy/utils/enum.CowArc.html
[`CowArc::Static`]: https://docs.rs/bevy/0.12.0/bevy/utils/enum.CowArc.html#variant.Static

## Suspend and Resume on Android

<div class="release-feature-authors">authors: @mockersf</div>

On Android, applications no longer crash on suspend. Instead, they are paused, and no systems
will run until the application is resumed.

This resolves the last "big" showstopper for Android apps! Bevy now supports Android!

<video controls><source src="suspend-resume.mp4" type="video/mp4"/></video>

Background tasks working in other threads, like playing audio, won't be stopped. When the
application will be suspended, a [`Lifetime`] event `ApplicationLifetime::Suspended` is sent,
corresponding to the [`onStop()`] callback. You should take care to pause tasks that shouldn't
run in the background, and resume them when receiving the `ApplicationLifetime::Resumed` event
(corresponding to the [`onRestart()`] callback).

```rust
fn handle_lifetime_events(
    mut lifetime_events: EventReader<ApplicationLifetime>,
    music_controller: Query<&AudioSink>,
) {
    for event in lifetime_events.read() {
        match event {
            // Upon receiving the `Suspended` event, the application has 1 frame before it is paused
            // As audio happens in an independent thread, it needs to be stopped
            ApplicationLifetime::Suspended => music_controller.single().pause(),
            // On `Resumed``, audio can continue playing
            ApplicationLifetime::Resumed => music_controller.single().play(),
            // `Started` is the only other event for now, more to come in the next Bevy version
            _ => (),
        }
    }
}
```

[`Lifetime`]: https://docs.rs/bevy/0.12.0/bevy/window/enum.Lifetime.html
[`onStop()`]: https://developer.android.com/reference/android/app/Activity#onStop()
[`onRestart()`]: https://developer.android.com/reference/android/app/Activity#onRestart()

## Material Extensions

<div class="release-feature-authors">authors: @robtfm</div>

Bevy has a powerful shader import system, allowing modular (and granular) shader code reuse. In previous versions of Bevy, this meant that in theory, you could import Bevy's PBR shader logic and use it in your own shaders. However in practice this was challenging, as you had to re-wire everything up yourself, which required intimate knowledge of the base material. For complicated materials like Bevy's PBR [`StandardMaterial`], this was full of boilerplate, resulted in code duplication, and was prone to errors.

In **Bevy 0.12**, we've built a **Material Extensions** system, which enables defining new materials that build on existing materials:

![material extension](material_extension.png)

This is accomplished via a new [`ExtendedMaterial`] type:

```rust
app.add_plugin(
    MaterialPlugin::<ExtendedMaterial<StandardMaterial, QuantizedMaterial>>::default()
);

#[derive(Asset, AsBindGroup, TypePath, Debug, Clone)]
struct QuantizedMaterial {
    // Start at a high binding number to ensure bindings don't conflict
    // with the base material
    #[uniform(100)]
    quantize_steps: u32,
}

impl MaterialExtension for QuantizedMaterial {
    fn fragment_shader() -> ShaderRef {
        "quantized_material.wgsl".into()
    }
}

let material = ExtendedMaterial<StandardMaterial, QuantizedMaterial> {
    base: StandardMaterial::from(Color::rgb(0.1, 0.1, 0.8)),
    extension: QuantizedMaterial { quantize_steps: 2 },
};
```

We also paired this with some [`StandardMaterial`] shader refactors to make it much easier to pick and choose which parts you want:

```rust
// quantized_material.wgsl

struct QuantizedMaterial {
    quantize_steps: u32,
}

@group(1) @binding(100)
var<uniform> my_extended_material: QuantizedMaterial;

@fragment
fn fragment(
    input: VertexOutput,
    @builtin(front_facing) is_front: bool,
) -> FragmentOutput {
    // Generate a PbrInput struct from the StandardMaterial bindings
    var pbr_input = pbr_input_from_standard_material(input, is_front);

    // Alpha discard
    pbr_input.material.base_color = alpha_discard(
        pbr_input.material,
        pbr_input.material.base_color
    );

    var out: FragmentOutput;

    // Apply lighting
    out.color = apply_pbr_lighting(pbr_input);

    // Our "quantize" effect
    out.color = vec4<f32>(vec4<u32>(out.color * f32(my_extended_material.quantize_steps))) / f32(my_extended_material.quantize_steps);

    // Apply in-shader post processing.
    // Ex: fog, alpha-premultiply, etc. For non-hdr cameras: tonemapping and debanding
    out.color = main_pass_post_lighting_processing(pbr_input, out.color);

    return out;
}
```

This _vastly_ simplifies writing custom PBR materials, making it accessible to pretty much everyone!

[`ExtendedMaterial`]: https://docs.rs/bevy/0.12.0/bevy/pbr/struct.ExtendedMaterial.html

## Automatic Batching and Instancing of Draw Commands

<div class="release-feature-authors">authors: @superdump (Rob Swain)</div>

**Bevy 0.12** now automatically batches/instances draw commands where possible. This cuts down the number of draw calls, which yields significant performance wins!

This required a number of architectural changes, including how we store and access per-entity mesh data (more on this later).

Here are some benches of the old unbatched approach (0.11) to the new batched approach (0.12):

### 2D Mesh Bevymark (frames per second, more is better)

This renders 160,000 entities with textured quad meshes (160 groups of 1,000 entities each, each group sharing a material). This means we can batch each group, resulting in only 160 instanced draw calls when batching is enabled. This gives a **200% increase in frame rate (3x)**!

![0.12-2DMeshes](0.12-2DMeshes.svg)
<div style="font-size: 1.0rem" class="release-feature-authors">Tested on an M1 Max at 1080p.</div>

### 3D Mesh "Many Cubes" (frames per second, more is better)

This renders 160,000 cubes, of which ~11,700 are visible in the view. These are drawn using a single instanced draw of all visible cubes which enables up to **100% increase in frame rate (2x)**!

![0.12-3DMeshes](0.12-3DMeshes.svg)
<div style="font-size: 1.0rem" class="release-feature-authors">Tested on an M1 Max at 1080p.</div>

These performance benefits can be leveraged on all platforms, including WebGL2!

### What can be batched?

Batching/Instancing can only happen for GPU data that doesn't require "rebinding" (binding is making data available to shaders / pipelines, which incurs a runtime cost). This means if something like a pipeline (shaders), bind group (shader-accessible bound data), vertex / index buffer (mesh) is different, it cannot be batched.

From a high level, currently entities with the same material and mesh can be batched.

We are investigating ways to make more data accessible without rebinds, such as bindless textures, combining meshes into larger buffers, etc.

### Opting Out

If you would like to opt out an entity from automatic batching, you can add the new [`NoAutomaticBatching`] component to it.

This is generally for cases where you are doing custom, non-standard renderer features that don't play nicely with batching's assumptions. For example, it assumes view bindings are constant across draws and that Bevy's-built-in entity batching logic is used.

[`NoAutomaticBatching`]: https://docs.rs/bevy/0.12.0/bevy/render/batching/struct.NoAutomaticBatching.html

## The Road to GPU-driven Rendering

<div class="release-feature-authors">authors: @superdump (Rob Swain), @james-j-obrien, @JMS55, @inodentry, @robtfm, @nicopap, @teoxoy, @IceSentry, @Elabajaba</div>

Bevy's renderer performance for 2D and 3D meshes can improve a lot. There are bottlenecks on both the CPU and GPU side, which can be lessened to give significantly higher frame rates. As always with Bevy, we want to make the most of the platforms you use, from the constraints of WebGL2 and mobile devices, to the highest-end native discrete graphics cards. A solid foundation can support all of this.

In **Bevy 0.12** we have started reworking rendering data structures, data flow, and draw patterns to unlock new optimizations. This enabled the **Automatic Batching and Instancing** we landed in **Bevy 0.12** and also helps pave the way for other significant wins in the future, such as GPU-driven rendering. We aren't quite ready for GPU-driven rendering, but we've started down that path in **Bevy 0.12**!

### What are CPU and GPU-driven rendering?

CPU-driven rendering is where draw commands are created on the CPU. In Bevy this means "in Rust code", more specifically in render graph nodes. This is how Bevy currently kicks off draws.

In GPU-driven rendering, the draw commands are encoded on the GPU by [compute shaders](https://www.khronos.org/opengl/wiki/Compute_Shader). This leverages GPU parallelism, and unlocks more advanced culling optimizations that are infeasible to do on the CPU, among many other methods that bring large performance benefits.

### What needs to change?

Historically Bevy's general GPU data pattern has been to bind each piece of data per-entity and issue a draw call per-entity. In some cases we did store data in uniform buffers in "array style" and accessed with dynamic offsets, but this still resulted in rebinding at each offset.

All of this rebinding has performance implications, both on the CPU and the GPU. On the CPU, it means encoding draw commands has many more steps to process, taking more time than necessary. On the GPU (and in the graphics API), it means many more rebinds and separate draw commands.

Avoiding rebinding is both a big performance benefit for CPU-driven rendering and is necessary to enable GPU-driven rendering.

To avoid rebinds, the general data pattern we are aiming for is:

* For each data type (meshes, materials, transforms, textures), create a single array (or a small number of arrays) containing all of the items of that data type
* Bind these arrays a small number of times (ideally once), avoiding per-entity/per-draw rebinds

In **Bevy 0.12** we've started this process in earnest! We've made a number of architectural changes that are already yielding fruit. Thanks to these changes, we can now [automatically batch and instance draws](#automatic-batching-and-instancing-of-draw-commands) for entities with the exact same mesh and material. And as we progress further down this path, we can batch/instance across a wider variety of cases, cutting out more and more CPU work until eventually we are "fully GPU-driven".

### Reorder Render Sets

<div class="release-feature-authors">authors: @superdump (Rob Swain), @james-j-obrien, @inodentry</div>

The order of draws needs to be known for some methods of instanced draws so that the data can be laid out, and looked up in order. For example, when per-instance data is stored in an instance-rate vertex buffer.

The render set order before **Bevy 0.12** caused some problems with this as data had to be prepared (written to the GPU) before knowing the draw order. Not ideal when our plan is to have an ordered list of entity data on the GPU! The previous order of sets was:

![RenderSets-0.11](RenderSets-0.11.svg)

This caused friction (and suboptimal instancing) in a number of current (and planned) renderer features. Most notably in previous versions of Bevy, it caused these problems for sprite batching.  

The new render set order in 0.12 is:

![RenderSets-0.12](RenderSets-0.12.svg)

`PrepareAssets` was introduced because we only want to queue entities for drawing if their assets have been prepared. Per-frame data preparation still happens in the `Prepare` set, specifically in its `PrepareResources` subset. That is now after `Queue` and `Sort`, so the order of draws is known. This also made a lot more sense for batching, as it is now known at the point of batching whether an entity that is of another type in the render phase needs to be drawn. Bind groups now have a clear subset where they should be created ... `PrepareBindGroups`.

### BatchedUniformBuffer and GpuArrayBuffer

OK, so we need to put many pieces of data of the same type into buffers in a way that we can bind them as few times as possible and draw multiple instances from them. How can we do that?

In previous versions of Bevy, per-instance `MeshUniform` data is stored in a uniform buffer with each instance's data aligned to a dynamic offset. When drawing each mesh entity, we update the dynamic offset, which can be close in cost to rebinding. It looks like this:

![DynamicUniformBuffer](DynamicUniformBuffer.svg)
<div style="font-size: 1.0rem" class="release-feature-authors">Red arrows are 'rebinds' to update the dynamic offset, blue boxes are instance data, orange boxes are padding for dynamic offset alignment, which is a requirement of GPUs and graphics APIs.</div>

Instance-rate vertex buffers are one way, but they are very constrained to having a specific order. They are/may be suitable for per-instance data like mesh entity transforms, but they can't be used for material data. The other main options are uniform buffers, storage buffers, and data textures.

WebGL2 does not support storage buffers, only uniform buffers. Uniform buffers have a minimum guaranteed size per binding of 16kB on WebGL2. Storage buffers, where available, have a minimum guaranteed size of 128MB.

Data textures are far more awkward for structured data. And on platforms that don't support linear data layouts, they will perform worse.

Given these constraints, we want to use storage buffers on platforms where they are supported, and we want to use uniform buffers on platforms where they are not supported (ex: WebGL 2).

#### BatchedUniformBuffer

<div class="release-feature-authors">authors: @superdump (Rob Swain), @JMS55, @teoxoy, @robtfm, @konsolas</div>

For uniform buffers, we have to assume that on WebGL2 we may only be able to access 16kB of data at a time. Taking an example, `MeshUniform` requires 144 bytes per instance, which means we can have a batch of 113 instances per 16kB binding. If we want to draw more than 113 entities in total, we need a way of managing a uniform buffer of data that can be bound at a dynamic offset per batch of instances. This is what `BatchedUniformBuffer` is designed to solve.

`BatchedUniformBuffer` looks like this:

![BatchedUniformBuffer](BatchedUniformBuffer.svg)
<div style="font-size: 1.0rem" class="release-feature-authors">Red arrows are 'rebinds' to update the dynamic offset, blue boxes are instance data, orange boxes are padding for dynamic offset alignment.</div>

Notice how the instance data can be packed much more tightly, fitting the same amount of used data in less space. Also, we only need to update the dynamic offset of the binding for each batch.

#### GpuArrayBuffer

<div class="release-feature-authors">authors: @superdump (Rob Swain), @JMS55, @IceSentry, @mockersf</div>

Given that we need to support both uniform and storage buffers for a given data type, this increases the level of complexity required to implement new low-level renderer features (both in Rust code and in shaders). When confronted with this complexity, some developers might choose instead only use storage buffers (effectively dropping support for WebGL 2).

To make it as easy as possible to support both storage types, we developed [`GpuArrayBuffer`]. This is a generic collection of `T` values that abstracts over `BatchedUniformBuffer` and [`StorageBuffer`]. It will use the right storage for the current platform / GPU.

The data in a [`StorageBuffer`] looks like this:

![StorageBuffer](StorageBuffer.svg)
<div style="font-size: 1.0rem" class="release-feature-authors">Red arrows are 'rebinds', blue boxes are instance data.</div>

All the instance data can be placed directly one after the other, and we only have to bind once. There is no need for any dynamic offset binding, so there is no need for any padding for alignment.

[Check out this annotated code example](https://gist.github.com/cart/3a9f190bd5e789a7d42317c28843ffca) that illustrates using [`GpuArrayBuffer`] to support both uniform and storage buffer bindings.

[`GpuArrayBuffer`]: https://docs.rs/bevy/0.12.0/bevy/render/render_resource/enum.GpuArrayBuffer.html
[`StorageBuffer`]: https://docs.rs/bevy/0.12.0/bevy/render/render_resource/struct.StorageBuffer.html

### 2D / 3D Mesh Entities using GpuArrayBuffer

<div class="release-feature-authors">authors: @superdump (Rob Swain), @robtfm, @Elabajaba</div>

The 2D and 3D mesh entity rendering was migrated to use [`GpuArrayBuffer`] for the mesh uniform data.

Just avoiding the rebinding of the mesh uniform data buffer gives about a 6% increase in frame rates!

## EntityHashMap Renderer Optimization

<div class="release-feature-authors">authors: @superdump (Rob Swain), @robtfm, @pcwalton, @jancespivo, @SkiFire13, @nicopap</div>

Since **Bevy 0.6**, Bevy's renderer has extracted data from the "main world" into a separate "render world". This enables [Pipelined Rendering](/news/bevy-0-6/#pipelined-rendering-extract-prepare-queue-render), which renders frame N in the render app, while the main app simulates frame N+1.

Part of the design involves clearing the render world of all entities between frames. This enables consistent Entity mapping between the main and render worlds while still being able to spawn new entities in the render world that don't exist in the main world.

Unfortunately, this ECS usage pattern also incurred some significant performance problems. To get good "linear iteration read performance", we wanted to use "table storage" (Bevy's default ECS storage model). However in the renderer, entities are cleared and respawned each frame, components are inserted across many systems and different parts of the render app schedule. This resulted in a lot of "archetype moves" as new components were inserted from various renderer contexts. When an entity moves to a new archetype, all of its "table storage" components are copied into the new archetype's table. This can be expensive across many archetype moves and/or large table moves.

This was unfortunately leaving a lot of performance on the table. Many ideas were discussed over a long period for how to improve this.

### The Path Forward

The main two paths forward were:

1. Persist render world entities and their component data across frames
2. Stop using entity table storage for storing component data in the render world

We have decided to explore option (2) for **Bevy 0.12** as persisting entities involves solving other problems that have no simple and satisfactory answers (ex: how do we keep the worlds perfectly in sync without leaking data). We may find those answers eventually, but for now we chose the path of least resistance!

We landed on using `HashMap<Entity, T>` with an optimized hash function designed by @SkiFire13, and inspired by [`rustc-hash`](https://github.com/rust-lang/rustc-hash). This is exposed as [`EntityHashMap`] and is the new way to store component data in the render world.

This [yielded significant performance wins](https://github.com/bevyengine/bevy/pull/9903).

[`EntityHashMap`]: https://docs.rs/bevy/0.12.0/bevy/utils/type.EntityHashMap.html

### Usage

The easiest way to use it is to use the new [`ExtractInstancesPlugin`]. This will extract all entities matching a query, or only those that are visible, extracting multiple components at once into one target type.

It is a good idea to group component data that will be accessed together into one target type to avoid having to do multiple lookups.

To extract two components from visible entities:

```rust
struct MyType {
    a: ComponentA,
    b: ComponentB,
}

impl ExtractInstance for MyType {
    type Query = (Read<ComponentA>, Read<ComponentB>);
    type Filter = ();

    fn extract((a, b): QueryItem<'_, Self::Query>) -> Option<Self> {
        Some(MyType {
          a: a.clone(),
          b: b.clone(),
        })
    }
}

app.add_plugins(ExtractInstancesPlugin::<MyType>::extract_visible());
```

[`ExtractInstancesPlugin`]: https://docs.rs/bevy/0.12.0/bevy/render/extract_instances/struct.ExtractInstancesPlugin.html

## Sprite Instancing

<div class="release-feature-authors">authors: @superdump (Rob Swain)</div>

In previous versions of Bevy, Sprites were rendered by generating a vertex buffer containing 4 vertices per sprite with position, UV, and possibly color data. This has proven to be very effective. However, having to split batches of sprites into multiple draws because they use a different color is suboptimal.

Sprite rendering now uses an instance-rate vertex buffer to store the per-instance data. Instance-rate vertex buffers are stepped when the instance index changes, rather than when the vertex index changes. The new buffer contains an affine transformation matrix that enables translation, scaling, and rotation in one transform. It contains per-instance color, and UV offset and scale.

This retains all the functionality of the previous method, enables the additional flexibility of any sprite being able to have a color tint and all still be drawn in the same batch, and uses a total of 80 bytes per sprite, versus 144 bytes previously.

This resulted in a performance improvement of up to **40%** versus the previous method!

## Rusty Shader Imports

<div class="release-feature-authors">authors: @robtfm</div>

Bevy shaders now use Rust-like shader imports:

```rust
// old
#import bevy_pbr::forward_io VertexOutput

// new
#import bevy_pbr::forward_io::VertexOutput
```

Like Rust imports, you can use curly braces to import multiple items. Multi-level nesting is also now supported!

```rust
// old
#import bevy_pbr::pbr_functions alpha_discard, apply_pbr_lighting 
#import bevy_pbr                mesh_bindings

// new
#import bevy_pbr::{
    pbr_functions::{alpha_discard, apply_pbr_lighting}, 
    mesh_bindings,
}
```

Like Rust modules, you can now import partial paths:

```rust
#import part::of::path

// later in the shader
path::remainder::function();
```

You can also now use fully qualified paths without importing:

```rust
bevy_pbr::pbr_functions::pbr()
```

Rusty Imports remove a number of "API weirdness" gotchas from the old system and expand the capabilities of the import system. And by reusing Rust syntax and semantics, we remove the need for Bevy users to learn a new system.

## glTF Emissive Strength

<div class="release-feature-authors">authors: @JMS55</div>

Bevy now reads and uses the `KHR_materials_emissive_strength` glTF material extension when loading glTF assets. This adds support for emissive materials when importing glTF from programs like Blender. Each of these cubes has increasing emissive strength:

![gltf emissive](gltf_emissive.png)

## Import Second UV Map In glTF Files

<div class="release-feature-authors">authors: @pcwalton</div>

**Bevy 0.12** now imports the second UV map (`TEXCOORD1` or `UV1`) if it is defined in glTF files and exposes it to shaders. Conventionally this is often used for lightmap UVs. This was an often requested feature and it unlocks lightmapping scenarios (both in custom user shaders and in future Bevy releases).

## Wireframe Improvements

<div class="release-feature-authors">authors: @IceSentry</div>

The wireframes now use Bevy's [`Material`] abstraction. This means it will automatically use the new batching and instancing features while being easier to maintain. This change also made it easier to add support for colored wireframe. You can configure the color globally or per mesh using the [`WireframeColor`] component. It's also now possible to disable wireframe rendering by using the [`NoWireframe`] component.

![wireframe](wireframe.png)

[`Material`]: https://docs.rs/bevy/0.12.0/bevy/pbr/trait.Material.html
[`WireframeColor`]: https://docs.rs/bevy/0.12.0/bevy/pbr/wireframe/struct.WireframeColor.html
[`NoWireframe`]: https://docs.rs/bevy/0.12.0/bevy/pbr/wireframe/struct.NoWireframe.html

## External Renderer Context

<div class="release-feature-authors">authors: @awtterpip</div>

Historically Bevy's [`RenderPlugin`] has been fully responsible for initializing the [`wgpu`] render context. However some 3rd party Bevy Plugins, such as this work-in-progress [`bevy_openxr`](https://github.com/awtterpip/bevy_openxr) plugin, require more control over renderer initialization.

Therefore in **Bevy 0.12**, we've made it possible to pass in the [`wgpu`] render context at startup. This means the 3rd party [`bevy_openxr`] plugin can be a "normal" Bevy plugin without needing to fork Bevy!

Here is a quick video of Bevy VR, courtesy of [`bevy_openxr`]!

<video controls><source src="bevy_openxr.mp4" type="video/mp4"/></video>

[`bevy_openxr`]: https://github.com/awtterpip/bevy_openxr/
[`wgpu`]: https://github.com/gfx-rs/wgpu
[`RenderPlugin`]: https://docs.rs/bevy/0.12.0/bevy/render/struct.RenderPlugin.html

## Bind Group Ergonomics

<div class="release-feature-authors">authors: @robtfm, @JMS55</div>

When defining "bind groups" for low-level renderer features, we use the following API:

```rust
render_device.create_bind_group(
    "my_bind_group",
    &my_layout,
    &[
        BindGroupEntry {
            binding: 0,
            resource: BindingResource::Sampler(&my_sampler),
        },
        BindGroupEntry {
            binding: 1,
            resource: my_uniform,
        },
    ],
);
```

This works reasonably well, but for large numbers of bind groups, the `BindGroupEntry` boilerplate makes it harder than necessary to read and write everything (and keep the indices up to date).

**Bevy 0.12** adds additional options:

```rust
// Sets the indices automatically using the index of the tuple item
render_device.create_bind_group(
    "my_bind_group",
    &my_layout,
    &BindGroupEntries::sequential((&my_sampler, my_uniform)),
);
```

```rust
// Manually sets the indices, but without the BindGroupEntry boilerplate!
render_device.create_bind_group(
    "my_bind_group",
    &my_layout,
    &BindGroupEntries::with_indexes((
        (2, &my_sampler),
        (3, my_uniform),
    )),
);
```

## One-Shot Systems

<div class="release-feature-authors">authors: @alice-i-cecile @pascualex, @Trashtalk217, @Zeenobit</div>

Ordinarily, systems run once per frame, as part of a schedule.
But this isn't always the right fit.
Maybe you're responding to a very rare event like in a complex turn-based game, or simply don't want to clutter your schedule with a new system for every single button.
One-shot systems flip that logic on its head, and provide you the ability to run arbitrary logic on demand, using the powerful and familiar system syntax.

```rust
#[derive(Resource, Default, Debug)]
struct Counter(u8);

fn increment(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("{}", counter.0);
}

fn foo(world: &mut World) {
    world.init_resource::<Counter>();
    let id = world.register_system(increment);
    let _ = world.run_system(id); // prints 1
    let _ = world.run_system(id); // prints 2
}
```

There are three simple steps to using one-shot systems: register a system, store its `SystemId`, and then use either exclusive world access or commands to run the corresponding system.

A lot becomes possible with just that, however `SystemId`s really start showing their power, when they're wrapped into components.

```rust
use bevy::ecs::system::SystemId;

#[derive(Component)]
struct Callback(SystemId);

// calling all callbacks!
fn call_all(query: Query<&Callback>, mut commands: Commands) {
    for callback in query.iter() {
        commands.run_system(callback.0);
    }
}
```

One-shot systems can then be attached to UI elements, like buttons, actions in an RPG, or any other entity. You might even feel inspired to implement the Bevy scheduling graph with one-shot systems and [`aery`](https://docs.rs/aery/latest/aery/) (let us know how that goes, by the way).

One-shot systems are very flexible.
They can be nested, so you can call `run_system` from within a one-shot system.
It's possible to have multiple instances of one system registered at a time, each with their own `Local` variables and cached system state.
It also plays nice with asset-driven workflows: recording a mapping from a string to an identifier in a serialized callback is much nicer than trying to do so with Rust functions!

Still, one-shot systems are not without their limitations.
Currently, exclusive systems and systems designed for system piping (with either an `In` parameter or a return type) can't be used at all.
You also can't call a one-shot systems from itself, recursion isn't possible.
Lastly, one-shot systems are always evaluated sequentially, rather than in parallel.
While this reduces both complexity and overhead, for certain workloads this can be meaningfully slower than using a schedule with a parallel executor.

However, when you're just prototyping or writing a unit test, it can be a real hassle: two whole functions and some weird identifier?
For these situations, you can use the `World::run_system_once` method.

```rust
use bevy::ecs::system::RunSystemOnce;

#[derive(Resource, Default, Debug)]
struct Counter(u8);

fn increment(mut counter: ResMut<Counter>) {
    counter.0 += 1;
    println!("{}", counter.0);
}

let mut world = World::new();
world.init_resource::<Counter>();
world.run_system_once(increment); // prints 1
world.run_system_once(increment); // prints 2
```

This is great for unit testing systems and queries, and it's both lower overhead and simpler to use. However, there is one caveat. Some systems have state, either in the form of `Local` arguments, change detection, or `EventReader`s. This state isn't saved between two `run_system_once` calls, creating odd behavior. The `Local`s reset every run, while change detection will _always_ detect data as added/changed. Be careful and you'll be alright.

## system.map

<div class="release-feature-authors">authors: @JoJoJet</div>

**Bevy 0.12** adds a new [`system.map()`] function, which is a cheaper and more ergonomic alternative to [`system.pipe()`].

Unlike [`system.pipe()`], [`system.map()`] just takes a normal closure (instead of another system) that accepts the output of the system as its parameter:

```rust
app.add_systems(Update, my_system.map(error));

fn my_system(res: Res<T>) -> Result<(), Err> {
    // do something that might fail here
}

// An adapter that logs errors 
pub fn error<E: Debug>(result: Result<(), E>) {
    if let Err(warn) = result {
        error!("{:?}", warn);
    }
}
```

Bevy provides built in `error`, `warn`, `debug`, and `info` adapters that can be used with [`system.map()`] to log errors at each of these levels.

[`system.map()`]: https://docs.rs/bevy/0.12.0/bevy/ecs/system/trait.IntoSystem.html#method.map
[`system.pipe()`]: https://docs.rs/bevy/0.12.0/bevy/ecs/system/trait.IntoSystem.html#method.pipe

## Simplify Parallel Iteration Method

<div class="release-feature-authors">authors: @JoJoJet</div>

**Bevy 0.12** makes the parallel Query iterator [`for_each()`] compatible with both mutable and immutable queries, reducing API surface and removing the need to write `mut` twice:

```rust
// Before:
query.par_iter_mut().for_each_mut(|x| ...);

// After:
query.par_iter_mut().for_each(|x| ...);
```

[`for_each()`]: https://docs.rs/bevy/0.12.0/bevy/ecs/query/struct.QueryParIter.html#method.for_each

## Disjoint Mutable World Access Via EntityMut

<div class="release-feature-authors">authors: @JoJoJet</div>

**Bevy 0.12** supports safely accessing multiple [`EntityMut`] values at once, meaning you can mutate multiple entities (with access to _all of their components_) at the same time.

```rust
let [entity1, entity2] = world.many_entities_mut([id1, id2]);
*entity1.get_mut::<Transform>().unwrap() = *entity2.get::<Transform>().unwrap();
```

This also works in queries:

```rust
// This would not have been expressible in previous Bevy versions
// Now it is totally valid!
fn system(q1: Query<&mut A>, q2: Query<EntityMut, Without<A>>) {
}
```

You can now mutably iterate all entities and access arbitrary components within them:

```rust
for mut entity in world.iter_entities_mut() {
    let mut transform = entity.get_mut::<Transform>().unwrap();
    transform.translation.x += 2.0;
}
```

This required reducing the access scope of [`EntityMut`] to _only_ the entity it accesses (previously it had escape hatches that allowed direct [`World`] access). Use [`EntityWorldMut`] for an equivalent to the old "global access" approach.

[`EntityMut`]: https://docs.rs/bevy/0.12.0/bevy/ecs/world/struct.EntityMut.html
[`EntityWorldMut`]: https://docs.rs/bevy/0.12.0/bevy/ecs/world/struct.EntityWorldMut.html
[`World`]: https://docs.rs/bevy/0.12.0/bevy/ecs/world/struct.World.html

## Unified configure_sets API

<div class="release-feature-authors">authors: @geieredgar</div>

Bevy's [Schedule-First API](/news/bevy-0-11/#schedule-first-ecs-apis) introduced in **Bevy 0.11** unified most of the ECS scheduler API surface under a single `add_systems` API. However, we didn't do a unified API for `configure_sets`, meaning there were two different APIs:

```rust
app.configure_set(Update, A.after(B));
app.configure_sets(Update, (A.after(B), B.after(C));
```

In **Bevy 0.12**, we have unified these under a single API to align with the patterns we've used elsewhere and cut down on unnecessary API surface:

```rust
app.configure_sets(Update, A.after(B));
app.configure_sets(Update, (A.after(B), B.after(C));
```

## UI Materials

<div class="release-feature-authors">authors: @MarkusTheOrt</div>

Bevy's material system has been brought to Bevy UI thanks to the new [`UiMaterial`]:

![ui material](ui_material.png)

This "circle" UI Node is drawn with a custom shader:

```rust
#import bevy_ui::ui_vertex_output::UiVertexOutput

struct CircleMaterial {
    @location(0) color: vec4<f32>
}

@group(1) @binding(0)
var<uniform> input: CircleMaterial;

@fragment
fn fragment(in: UiVertexOutput) -> @location(0) vec4<f32> {
    let uv = in.uv * 2.0 - 1.0;
    let alpha = 1.0 - pow(sqrt(dot(uv, uv)), 100.0);
    return vec4<f32>(input.color.rgb, alpha);
}
```

And just like other Bevy material types, it is simple to set up in code!

```rust
#[derive(AsBindGroup, Asset, TypePath, Debug, Clone)]
struct CircleMaterial {
    #[uniform(0)]
    color: Vec4,
}

impl UiMaterial for CircleMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/circle_shader.wgsl".into()
    }
}

// Register the material plugin in your App
app.add_plugins(UiMaterialPlugin::<CircleMaterial>::default())

// Later in your app, spawn the UI node with your material!
commands.spawn(MaterialNodeBundle {
    style: Style {
        position_type: PositionType::Absolute,
        width: Val::Px(250.0),
        height: Val::Px(250.0),
        ..default()
    },
    material: materials.add(CircleMaterial {
        color: Color::rgb(0.0, 1.0, 0.58).into(),
    }),
    ..default()
});
```

[`UiMaterial`]: https://docs.rs/bevy/0.12.0/bevy/ui/trait.UiMaterial.html

## UI Node Outlines

<div class="release-feature-authors">authors: @ickshonpe</div>

Bevy's UI nodes now support outlines "outside the borders" of UI nodes via the new [`Outline`] component. [`Outline`] does not occupy any space in the layout. This is different than [`Style::border`], which exists "as part of" the node in the layout:

![ui outlines](ui_outlines.png)

```rust
commands.spawn((
    NodeBundle::default(),
    Outline {
        width: Val::Px(6.),
        offset: Val::Px(6.),
        color: Color::WHITE,
    },
))
```

[`Outline`]: https://docs.rs/bevy/0.12.0/bevy/ui/struct.Outline.html
[`Style::border`]: https://docs.rs/bevy/0.12.0/bevy/ui/struct.Style.html

## Unified `Time`

<div class="release-feature-authors">authors: @nakedible @maniwani @alice-i-cecile</div>

Bevy 0.12 brings two major quality of life improvements to [`FixedUpdate`].

* [`Time`] now returns the contextually correct values for systems running in [`FixedUpdate`]. (As such, `FixedTime` has been removed.)
* [`FixedUpdate`] can no longer snowball into a "death spiral" (where the app freezes because [`FixedUpdate`] steps are enqueued faster than it can run them).

The [`FixedUpdate`] schedule and its companion `FixedTime` resource were introduced in Bevy 0.10, and it soon became apparent that `FixedTime` was lacking. Its methods were different from [`Time`] and it didn't even track "total time elapsed" like [`Time`] did, to name a few examples. Having two different "time" APIs also meant you had to write systems to specifically support "fixed timestep" or "variable timestep" and not both. It was desirable to not have this split as it can lead to incompatibilities between plugins down the road (which is sometimes the case with plugins in other game engines).

Now, you can just write systems that read [`Time`] and schedule them in either context.

```rust
// This system will see a constant delta time if scheduled in `FixedUpdate` or
// a variable delta time if scheduled in `Update`.
fn integrate_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_seconds();
        transform.translation.y += velocity.y * time.delta_seconds();
    }
}
```

Most systems should continue to use [`Time`], but behind the scenes, the methods from previous APIs have been refactored into four clocks:

* `Time<Real>`
* `Time<Virtual>`
* `Time<Fixed>`
* `Time<()>`

`Time<Real>` measures the true, unedited frame and app durations. For diagnostics/profiling, use that one. It's also used to derive the others. `Time<Virtual>` can be sped up, slowed down, and paused, and `Time<Fixed>` chases `Time<Virtual>` in fixed increments. Lastly, `Time<()>` is automatically overwritten with the current value of `Time<Fixed>` or `Time<Virtual>` upon entering or exiting `FixedUpdate`. When a system borrows `Time`, it actually borrows `Time<()>`.

Try the new [time example](https://github.com/bevyengine/bevy/blob/main/examples/time/time.rs) to get a better feel for these resources.

The fix for the windup problem was limiting how much `Time<Virtual>` can advance from a single frame. This then limits how many times [`FixedUpdate`] can be queued for the next frame, and so things like frame lag or your computer waking up from a long sleep can no longer cause a death spiral. So now, the app won't freeze, but things happening in [`FixedUpdate`] will appear to slow down since it'll be running at a temporarily reduced rate.

[`FixedUpdate`]: https://docs.rs/bevy/0.12.0/bevy/app/struct.FixedUpdate.html
[`Time`]: https://docs.rs/bevy/0.12.0/bevy/time/struct.Time.html

## ImageLoader Settings

<div class="release-feature-authors">authors: @cart, @Kanabenki</div>

To take advantage of the new [`AssetLoader`] settings in **Bevy Asset V2**, we've added [`ImageLoaderSettings`] to  [`ImageLoader`].

This means that you can now configure the sampler, SRGB-ness, and the format, on a per-image basis. These are the defaults, as they appear in **Bevy Asset V2** meta files:

```rust
(
    format: FromExtension,
    is_srgb: true,
    sampler: Default,
)
```

When set to `Default`, the image will use whatever is configured in [`ImagePlugin::default_sampler`].

However, you can set these values to whatever you want!

```rust
(
    format: Format(Basis),
    is_srgb: true,
    sampler: Descriptor((
        label: None,
        address_mode_u: ClampToEdge,
        address_mode_v: ClampToEdge,
        address_mode_w: ClampToEdge,
        mag_filter: Nearest,
        min_filter: Nearest,
        mipmap_filter: Nearest,
        lod_min_clamp: 0.0,
        lod_max_clamp: 32.0,
        compare: None,
        anisotropy_clamp: 1,
        border_color: None,
    )),
)
```

[`ImagePlugin::default_sampler`]: https://docs.rs/bevy/0.12.0/bevy/render/prelude/struct.ImagePlugin.html#structfield.default_sampler
[`ImageLoaderSettings`]: https://docs.rs/bevy/0.12.0/bevy/render/texture/struct.ImageLoaderSettings.html

## GamepadButtonInput

<div class="release-feature-authors">authors: @bravely-beep</div>

Bevy generally provides two ways to handle input of a given type:

* Events: receive a stream of input events in the order they occur
* The [`Input`] Resource: read the _current_ state of the input

One notable exception was [`GamepadButton`], which was only available via the [`Input`] resource. **Bevy 0.12** adds a new [`GamepadButtonInput`] event, filling this gap.

[`Input`]: https://docs.rs/bevy/0.12.0/bevy/input/struct.Input.html
[`GamepadButton`]: https://docs.rs/bevy/0.12.0/bevy/input/gamepad/struct.GamepadButton.html
[`GamepadButtonInput`]: https://docs.rs/bevy/0.12.0/bevy/input/gamepad/struct.GamepadButtonInput.html

## SceneInstanceReady Event

<div class="release-feature-authors">authors: @Shatur</div>

**Bevy 0.12** adds a new [`SceneInstanceReady`] event, which makes it easy to listen for a specific scene instance to be ready. "Ready" in this case means "fully spawned as an entity".

```rust
#[derive(Resource)]
struct MyScene(Entity);

fn setup(mut commands: Commands, assets: Res<AssetServer>) {
    let scene = SceneBundle {
        scene: assets.load("some.gltf#MyScene"),
        ..default()
    };
    let entity = commands.spawn(scene).id();
    commands.insert_resource(MyScene(entity));
}

fn system(mut events: EventReader<SceneInstanceReady>, my_scene: Res<MyScene>) {
    for event in events.read() {
        if event.parent == my_scene.0 {
            // the scene instance is "ready"
        }
    }
}
```

[`SceneInstanceReady`]: https://docs.rs/bevy/0.12.0/bevy/scene/struct.SceneInstanceReady.html

## Split Computed Visibility

<div class="release-feature-authors">authors: @JoJoJet</div>

The `ComputedVisibility` component has now been split into [`InheritedVisibility`] (visible in the hierarchy) and [`ViewVisibility`] (visible from a view), making it possible to use Bevy's built-in change detection on both sets of data separately.

[`InheritedVisibility`]: https://docs.rs/bevy/0.12.0/bevy/render/view/struct.InheritedVisibility.html
[`ViewVisibility`]: https://docs.rs/bevy/0.12.0/bevy/render/view/struct.ViewVisibility.html

## ReflectBundle

<div class="release-feature-authors">authors: @Shatur</div>

Bevy now supports "Bundle reflection" via [`ReflectBundle`]:

```rust
#[derive(Bundle, Reflect)]
#[reflect(Bundle)]
struct SpriteBundle {
    image: Handle<Image>,
    // other components here
}
```

This makes it possible to create and interact with ECS bundles using Bevy Reflect, meaning you can do these operations dynamically at runtime. This is useful for scripting and asset scenarios.

[`ReflectBundle`]: https://docs.rs/bevy/0.12.0/bevy/ecs/reflect/struct.ReflectBundle.html

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

## Limit Background FPS

<div class="release-feature-authors">authors: @maniwani</div>

If an app has no window in focus, Bevy will now limit its update rate (to 60Hz by default).

Before, many Bevy apps running on desktop operating systems (particularly macOS) would see spikes in CPU usage whenever their windows were minimized or completely covered, even with VSync enabled. The reason for this is that many desktop window managers ignore VSync for windows that aren't visible. As VSync normally limits how often an app updates, that speed limit vanishes while it's effectively disabled.

Now, apps running in the background will sleep in between updates to limit their FPS.

The one caveat is that most operating systems will not report if a window is visible, only if it has focus. So the throttle is based on focus, not visibility. 60Hz was then chosen as the default to maintain high FPS in cases where the window is not focused but still visible.

## `AnimationPlayer` API Improvements

<div class="release-feature-authors">authors: @devinleamy</div>

The `AnimationPlayer` now has new methods for controlling playback, and utilities for checking
if an animation is playing or completed, and getting its `AnimationClip` handle.

`set_elapsed` has been removed in favor of `seek_to`. `elapsed` now
returns the actual elapsed time and is not affected by the animation speed. `stop_repeating` has been removed
in favor of `set_repeat(RepeatAnimation::Never)`.

```rust
let mut player = q_animation_player.single_mut();
// Check if an animation is complete.
if player.is_finished() {
    // Set the playback mode.
    player.set_repeat(RepeatAnimation::Forever);
    player.set_repeat(RepeatAnimation::Never);
    player.set_repeat(RepeatAnimation::Count(4));
}
// Get a handle to the playing AnimationClip.
let clip_handle = player.animation_clip();
// Seek to 1s in the current clip.
player.seek_to(1.0);
```

## Ignore Ambiguous Components and Resources

<div class="release-feature-authors">authors: @hymm</div>

Ambiguity Reporting is an optional feature of Bevy's scheduler. When enabled it reports conflicts between systems that modify the same data, but are not ordered in relation to each other. While some reported conflicts can cause subtle bugs, many do not. Bevy has a couple existing methods and two new ones for ignoring these.

The existing APIs: `ambiguous_with`, which ignores conflicts between specific sets, and `ambiguous_with_all`, which ignores all conflicts with the set it's applied to. In addition, there are now 2 new APIs that let you ignore conflicts on a type of data, `allow_ambiguous_component` and `allow_ambiguous_resource`. These ignore all conflicts between systems on that specific type, component or resource, in a world.

```rust
#[derive(Resource)]
struct R;

// These systems are ambiguous on R
fn system_1(_: ResMut<R>) {}
fn system_2(_: Res<R>) {}

let mut app = App::new();
app.configure_schedules(ScheduleBuildSettings {
  ambiguity_detection: LogLevel::Error,
  ..default()
});
app.insert_resource(R);

app.add_systems(Update, ( system_1, system_2 ));
app.allow_ambiguous_resource::<R>();

// Running the app does not error.
app.update();
```

Bevy is now using this to ignore conflicts between the `Assets<T>` resources. Most of these ambiguities are modifying different assets and thus do not matter.

## Spatial Audio API Ergonomics

<div class="release-feature-authors">authors: @rparrett, @hymm, @mockersf</div>

A simple "stereo" (non-HRTF) spatial audio implementation was heroically [put together](https://bevyengine.org/news/bevy-0-10/#spatial-audio) at the last minute for Bevy 0.10, but the implementation was somewhat bare-bones and not very user-friendly. Users needed to write their own systems to update audio sinks with emitter and listener positions.

Now users can just add a `TransformBundle` to their `AudioBundle`s and Bevy will take care of the rest!

```rust
commands.spawn((
    TransformBundle::default(),
    AudioBundle {
        source: asset_server.load("sounds/bonk.ogg"),
        settings: PlaybackSettings::DESPAWN.with_spatial(true),
    },
));
```

## Pitch Audio Source

<div class="release-feature-authors">authors: @basilefff</div>

Audio can now be played by pitch, which is useful to debug audio issues, to use as a placeholder, or for programmatic audio.

A `Pitch` audio source can be created from its frequency and its duration, and then be used as a source in a `PitchBundle`.

```rust
fn play_pitch(
    mut pitch_assets: ResMut<Assets<Pitch>>,
    mut commands: Commands,
) {
    // This is a A for 1 second
    let pitch_handle = pitch_assets.add(Pitch::new(440.0, Duration::new(1, 0)));
    // Play it now
    commands.spawn(PitchBundle {
        source: pitch_handle,
        ..default()
    });
}
```

Audio is generated at the given frequency using a [sine wave](https://en.wikipedia.org/wiki/Sine_wave#Audio_example). More complex sounds can be created by playing several pitch audio sources at the same time, like chords or hamonics.

## Added HSL methods to `Color` struct

<div class="release-feature-authors">authors: @idedary</div>

You can now use `h()`, `s()`, `l()` together with their `set_h()`, `set_s()`, `set_l()` and `with_h()`, `with_s()`, `with_l()` variants to manipulate _Hue_, _Saturation_ and _Lightness_ values of a `Color` struct without cloning. Previously you could do that with only RGBA values.

```rust
// Returns HSL component values
let color = Color::ORANGE;
let hue = color.h();
// ...

// Changes the HSL component values
let mut color = Color::PINK;
color.set_s(0.5);
// ...

// Modifies existing colors and returns them
let color = Color::VIOLET.with_l(0.7);
// ...
```

## Reduced Tracing Overhead

<div class="release-feature-authors">authors: @hymm, @james7132</div>

Bevy uses the [tracing](https://crates.io/crates/tracing) library to measure system running time (among other things). This is useful for determining where bottlenecks in frame time are and measuring performance improvements. These traces can be visualized using the [tracy](https://github.com/wolfpld/tracy) tool. However, using tracing's spans has a significant overhead to it. A large part of the per-span overhead is due to allocating the string description of the span. By caching the spans for systems, commands, and parallel iteration, we have significantly reduced the CPU time overhead when using tracing. In the PR that introduced system span caching, our "many foxes" stress test went from 5.35 ms to 4.54 ms. In the PR that added caching for the parallel iteration spans, our "many cubes" stress test went from 8.89 ms to 6.8 ms.

![tracing overhead](tracing-overhead-reduction.png)

## AccessKit Integration Improvements

<div class="release-feature-authors">authors: @ndarilek</div>

Bevy 0.10's [AccessKit](https://accesskit.dev) integration made it incredibly easy for the engine to take the lead and push updates to the accessibility tree. But as any good dance partner knows, sometimes it's best not to lead but to follow.

This release adds the `ManageAccessibilityUpdates` resource which, when set to `false`, stops the engine from updating the tree on its own. This paves the way for third-party UIs with Bevy and AccessKit integration to send updates directly to Bevy. When the UI is ready to return control, `ManageAccessibilityUpdates` is set to `true` Bevy picks up where it left off and starts sending updates again.

AccessKit itself was also simplified, and this release capitalizes on that to shrink the surface area of our integration. If you're curious about how things work internally or want to help, the `bevy_a11y` crate is now more approachable than ever.

## TypePath Migration

<div class="release-feature-authors">authors: @soqb</div>

As a followup to the introduction of [Stable TypePath](/news/bevy-0-11/#stable-typepath) in **Bevy 0.11**, Bevy Reflect now uses [`TypePath`] instead of [`type_name`]. A reflected type's [`TypePath`] is now accessible via [`TypeInfo`] and [`DynamicTypePath`] and [`type_name`] methods have been removed.

[`TypeInfo`]: https://docs.rs/bevy/0.12.0/bevy/reflect/enum.TypeInfo.html
[`TypePath`]: https://docs.rs/bevy/0.12.0/bevy/reflect/trait.TypePath.html
[`DynamicTypePath`]: https://docs.rs/bevy/0.12.0/bevy/reflect/trait.DynamicTypePath.html
[`type_name`]: https://doc.rust-lang.org/std/any/fn.type_name.html

## Improved bevymark Example

<div class="release-feature-authors">authors: @superdump (Rob Swain), @IceSentry</div>

The bevymark example needed to be improved to enable benchmarking the batching / instanced draw changes. Modes were added to:

* draw 2D quad meshes instead of sprites: `--mode mesh2d`
* vary the per-instance color data instead of only varying the colour per wave of birds: `--vary-per-instance`
* generate a number of material / sprite textures and randomly choose from them either per wave or per instance depending on the vary per instance setting: `--material-texture-count 10`
* spawn the birds in random z order (new default), or in draw order: `--ordered-z`

This allows benchmarking of different situations for batching / instancing in the next section.

## CI Improvements

<div class="release-feature-authors">authors: @ameknite, @mockersf</div>

To help ensure examples are reusable outside of the Bevy repository, CI will now fail if an example
uses an import from `bevy_internal` instead of `bevy`.

Additionally, the daily mobile check job now builds on more iOS and Android devices:

* iPhone 13 on iOS 15
* iPhone 14 on iOS 16
* iPhone 15 on iOS 17
* Xiaomi Redmi Note 11 on Android 11
* Google Pixel 6 on Android 12
* Samsung Galaxy S23 on Android 13
* Google Pixel 8 on Android 14

## Example tooling improvements

<div class="release-feature-authors">authors: @mockersf</div>

The example showcase tool can now build all examples for WebGL2 or WebGPU. This is used to update
the website with all Wasm-compatible examples, which you can find
[here](https://bevyengine.org/examples/) for WebGL2, or
[here](https://bevyengine.org/examples-webgpu/) for WebGPU.

It is now also capable of capturing a screenshot while running all examples:

```sh
cargo run -p example-showcase -- run --screenshot
```

Some options are available to help with the execution, you can check them with `--help`.

Those screenshots are displayed on the example pages on the website, and can be used to check that
a PR didn't introduce a visible regression.

## Example execution in CI

<div class="release-feature-authors">authors: @mockersf, @rparrett</div>

All examples are now executed in CI on Windows with DX12, and on Linux with Vulkan. When possible,
a screenshot is taken and compared to the last execution. If an example crashes, the log is saved.
The mobile example is also executed on the same devices as the daily mobile check job.

A report of all those executions is built and available
[here](https://thebevyflock.github.io/bevy-example-runner/).

[![Example Report](example-report.png)](https://thebevyflock.github.io/bevy-example-runner/)

If you want to help sponsor tests on more platforms, get in touch!

## <a name="what-s-next"></a>What's Next?

We have plenty of work in progress! Some of this will likely land in **Bevy 0.13**.

Check out the [**Bevy 0.13 Milestone**](https://github.com/bevyengine/bevy/milestone/17) for an up-to-date list of current work being considered for **Bevy 0.13**.

* **Bevy Scene and UI Evolution**: We are hard at work building out a new Scene and UI system for Bevy. We're experimenting with a brand new [holistic Scene / UI system](https://github.com/bevyengine/bevy/discussions/9538) that will hopefully serve as the foundation for the Bevy Editor and make defining scenes in Bevy much more flexible, capable, and ergonomic.  
* **More Batching/Instancing Improvements**: Put skinned mesh data into storage buffers to enable instanced drawing of skinned mesh entities with the same mesh/skin/material. Put material data in the new GpuArrayBuffer to enable batching of draws of entities with the same mesh, material type, and textures, but different material data.
* **GPU-Driven Rendering**: We plan on driving rendering via the GPU by creating draw calls in compute shaders (on platforms that support it). We have [experiments using meshlets](https://github.com/bevyengine/bevy/pull/10164) and plan to explore other approaches as well. This will involve putting textures into bindless texture arrays and putting meshes in one big buffer to avoid rebinds.
* **Exposure Settings**: Control [camera exposure settings](https://github.com/bevyengine/bevy/pull/8407) to change the feel and mood of your renders!
* **GPU Picking**: [Efficiently select objects](https://github.com/bevyengine/bevy/pull/8784) with pixel perfect accuracy on the GPU!
* **Per-Object Motion Blur**: [Blur objects as they move](https://github.com/bevyengine/bevy/pull/9924) using their motion vectors
* **UI Node Border Radius and Shadows**: Support for [border radius and shadows](https://github.com/bevyengine/bevy/pull/8973) in Bevy UI
* **System Stepping**: Debug your app by [running systems step by step](https://github.com/bevyengine/bevy/pull/8453) for a given frame
* **Automatic Sync Points**: Support for [automatically inserting sync points](https://github.com/bevyengine/bevy/pull/9822) between systems with dependencies, removing the need for manual insertion and resolving a common source of errors.
* **Lightmap Support**: Support for [rendering pre-baked lightmaps](https://github.com/bevyengine/bevy/pull/10231)

## Support Bevy

Sponsorships help make our work on Bevy sustainable. If you believe in Bevy's mission, consider [sponsoring us](/donate) ... every bit helps!

<a class="button button--pink header__cta" href="/donate">Donate <img class="button__icon" src="/assets/heart.svg" alt="heart icon"></a>

## Contributors

Bevy is made by a [large group of people](/community/people/). A huge thanks to the 185 contributors that made this release (and associated docs) possible! In random order:

* @miketwenty1
* @viridia
* @d-bucur
* @mamekoro
* @jpsikstus
* @johanhelsing
* @ChristopherBiscardi
* @GitGhillie
* @superdump
* @RCoder01
* @photex
* @geieredgar
* @cbournhonesque-sc
* @A-Walrus
* @Nilirad
* @nicoburns
* @hate
* @CrumbsTrace
* @SykikXO
* @DevinLeamy
* @jancespivo
* @ethereumdegen
* @Trashtalk217
* @pcwalton
* @maniwani
* @robtfm
* @stepancheg
* @kshitijaucharmal
* @killercup
* @ricky26
* @mockersf
* @mattdm
* @softmoth
* @tbillington
* @skindstrom
* @CGMossa
* @ickk
* @Aceeri
* @Vrixyz
* @Feilkin
* @flisky
* @IceSentry
* @maxheyer
* @MalekiRe
* @torsteingrindvik
* @djeedai
* @rparrett
* @SIGSTACKFAULT
* @Zeenobit
* @ycysdf
* @nickrart
* @louis-le-cam
* @mnmaita
* @basilefff
* @mdickopp
* @gardengim
* @ManevilleF
* @Wcubed
* @PortalRising
* @JoJoJet
* @rj00a
* @jnhyatt
* @ryand67
* @alexmadeathing
* @floppyhammer
* @Pixelstormer
* @ItsDoot
* @SludgePhD
* @cBournhonesque
* @fgrust
* @sebosp
* @ndarilek
* @coreh
* @Selene-Amanita
* @aleksa2808
* @IDEDARY
* @kamirr
* @EmiOnGit
* @wpederzoli
* @Shatur
* @ClayenKitten
* @regnarock
* @hesiod
* @raffaeleragni
* @floreal
* @robojeb
* @konsolas
* @nxsaken
* @ameknite
* @66OJ66
* @Unarmed
* @MarkusTheOrt
* @alice-i-cecile
* @arsmilitaris
* @horazont
* @Elabajaba
* @BrandonDyer64
* @jimmcnulty41
* @SecretPocketCat
* @hymm
* @tadeohepperle
* @Dot32IsCool
* @waywardmonkeys
* @bushrat011899
* @devil-ira
* @rdrpenguin04
* @s-puig
* @denshika
* @FlippinBerger
* @TimJentzsch
* @sadikkuzu
* @paul-hansen
* @Neo-Zhixing
* @SkiFire13
* @wackbyte
* @JMS55
* @rlidwka
* @urben1680
* @BeastLe9enD
* @rafalh
* @ickshonpe
* @bravely-beep
* @Kanabenki
* @tormeh
* @opstic
* @iiYese
* @525c1e21-bd67-4735-ac99-b4b0e5262290
* @nakedible
* @Cactus-man
* @MJohnson459
* @rodolphito
* @MrGVSV
* @cyqsimon
* @DGriffin91
* @danchia
* @NoahShomette
* @hmeine
* @Testare
* @nicopap
* @soqb
* @cevans-uk
* @papow65
* @ptxmac
* @suravshresth
* @james-j-obrien
* @MinerSebas
* @ottah
* @doonv
* @pascualex
* @CleanCut
* @yrns
* @Quicksticks-oss
* @HaNaK0
* @james7132
* @awtterpip
* @aevyrie
* @ShadowMitia
* @tguichaoua
* @okwilkins
* @Braymatter
* @Cptn-Sherman
* @jakobhellermann
* @SpecificProtagonist
* @jfaz1
* @tsujp
* @Serverator
* @lewiszlw
* @dmyyy
* @cart
* @teoxoy
* @StaffEngineer
* @MrGunflame
* @pablo-lua
* @100-TomatoJuice
* @OneFourth
* @anarelion
* @VitalyAnkh
* @st0rmbtw
* @fornwall
* @ZacHarroldC5
* @NiseVoid
* @Dworv
* @NiklasEi
* @arendjr
* @Malax

## Full Changelog

### A-ECS + A-Diagnostics

* [Cache parallel iteration spans][9950]

### A-ECS + A-Scenes

* [Make builder types take and return `Self`][10001]

### A-Scenes

* [Move scene spawner systems to SpawnScene schedule][9260]
* [Add `SceneInstanceReady`][9313]
* [Add `SpawnScene` to prelude][9451]
* [Finish documenting `bevy_scene`][9949]
* [Only attempt to copy resources that still exist from scenes][9984]
* [Correct Scene loader error description][10161]

### A-Tasks + A-Diagnostics

* [Fix doc warning in bevy_tasks][9348]

### A-Tasks

* [elaborate on TaskPool and bevy tasks][8750]
* [Remove Resource and add Debug to TaskPoolOptions][9485]
* [Fix clippy lint in single_threaded_task_pool][9851]
* [Remove dependecies from bevy_tasks' README][9881]
* [Allow using async_io::block_on in bevy_tasks][9626]
* [add test for nested scopes][10026]
* [Global TaskPool API improvements][10008]

### A-Audio + A-Windowing

* [Application lifetime events (suspend audio on Android)][10158]

### A-Animation + A-Transform

* [Add system parameter for computing up-to-date `GlobalTransform`s][8603]

### A-Transform

* [Update `GlobalTransform` on insertion][9081]
* [Add `Without<Parent>` filter to `sync_simple_transforms`' orphaned entities query][9518]
* [Fix ambiguities in transform example][9845]

### A-App

* [Add `track_caller` to `App::add_plugins`][9174]
* [Remove redundant check for `AppExit` events in `ScheduleRunnerPlugin`][9421]
* [fix typos in crates/bevy_app/src/app.rs][10173]
* [fix typos in crates/bevy_app/src/app.rs][10173]
* [fix run-once runners][10195]

### A-ECS + A-App

* [Add configure_schedules to App and Schedules to apply `ScheduleBuildSettings` to all schedules][9514]
* [Only run event systems if they have tangible work to do][7728]

### A-Rendering + A-Gizmos

* [Fix gizmo draw order in 2D][9129]
* [Fix gizmo line width issue when using perspective][9067]

### A-Rendering + A-Diagnostics

* [Include note of common profiling issue][9484]
* [Enhance many_cubes stress test use cases][9596]
* [GLTF loader: handle warning NODE_SKINNED_MESH_WITHOUT_SKIN][9360]

### A-Rendering + A-Reflection

* [Register `AlphaMode` type][9222]

### A-Windowing

* [Add option to toggle window control buttons][9083]
* [Fixed: Default window is now "App" instead of "Bevy App"][9301]
* [improve documentation relating to `WindowPlugin` and `Window`][9173]
* [Improve `bevy_winit` documentation][7609]
* [Change `WinitPlugin` defaults to limit game update rate when window is not visible][7611]
* [User controlled window visibility][9355]
* [Check cursor position for out of bounds of the window][8855]
* [Fix doc link in transparent_window example][9697]
* [Wait before making window visible][9692]
* [don't create windows on winit StartCause::Init event][9684]
* [Fix the doc warning attribute and document remaining items for `bevy_window`][9933]
* [Revert "macOS Sonoma (14.0) / Xcode 15.0 — Compatibility Fixes + Docs…][9991]
* [Revert "macOS Sonoma (14.0) / Xcode 15.0 — Compatibility Fixes + Docs…][9991]
* [Allow Bevy to start from non-main threads on supported platforms][10020]
* [Prevent black frames during startup][9826]
* [Slightly improve `CursorIcon` doc.][10289]
* [Fix typo in window.rs][10358]

### A-Gizmos

* [Replace AHash with a good sequence for entity AABB colors][9175]
* [gizmo plugin lag bugfix][9166]
* [Clarify immediate mode in `Gizmos` documentation][9183]
* [Fix crash when drawing line gizmo with less than 2 vertices][9101]
* [Document that gizmo `depth_bias` has no effect in 2D][10074]

### A-Utils

* [change 'collapse_type_name' to retain enum types][9587]
* [bevy_derive: Fix `#[deref]` breaking other attributes][9551]
* [Move default docs][9638]

### A-Rendering + A-Assets

* [Import the second UV map if present in glTF files.][9992]
* [fix custom shader imports][10030]
* [Add `ImageSamplerDescriptor` as an image loader setting][9982]

### A-ECS

* [Add the Has world query to bevy_ecs::prelude][9204]
* [Simplify parallel iteration methods][8854]
* [Fix safety invariants for `WorldQuery::fetch` and simplify cloning][8246]
* [Derive debug for ManualEventIterator][9293]
* [Add `EntityMap::clear`][9291]
* [Add a paragraph to the lifetimeless module doc][9312]
* [opt-out `multi-threaded` feature flag][9269]
* [Fix `ambiguous_with` breaking run conditions][9253]
* [Add `RunSystem`][9366]
* [Add `replace_if_neq` to `DetectChangesMut`][9418]
* [Adding `Copy, Clone, Debug` to derived traits of `ExecutorKind`][9385]
* [Fix incorrect documentation link in `DetectChangesMut`][9431]
* [Implement `Debug` for `UnsafeWorldCell`][9460]
* [Relax In/Out bounds on impl Debug for dyn System][9581]
* [Improve various `Debug` implementations][9588]
* [Make `run_if_inner` public and rename to `run_if_dyn`][9576]
* [Refactor build_schedule and related errors][9579]
* [Add `system.map(...)` for transforming the output of a system][8526]
* [Reorganize `Events` and `EventSequence` code][9306]
* [Replaced EntityMap with HashMap][9461]
* [clean up configure_set(s) erroring][9577]
* [Relax more `Sync` bounds on `Local`][9589]
* [Rename `ManualEventIterator`][9592]
* [Replaced `EntityCommand` Implementation for `FnOnce`][9604]
* [Add a variant of `Events::update` that returns the removed events][9542]
* [Move schedule name into `Schedule`][9600]
* [port old ambiguity tests over][9617]
* [Refactor `EventReader::iter` to `read`][9631]
* [fix ambiguity reporting][9648]
* [Fix anonymous set name stack overflow][9650]
* [Fix unsoundness in `QueryState::is_empty`][9463]
* [Add panicking helpers for getting components from `Query`][9659]
* [Replace `IntoSystemSetConfig` with `IntoSystemSetConfigs`][9247]
* [Moved `get_component(_unchecked_mut)` from `Query` to `QueryState`][9686]
* [Fix naming on "tick" Column and ComponentSparseSet methods][9744]
* [Clarify a comment in Option WorldQuery impl][9749]
* [Return a boolean from `set_if_neq`][9801]
* [Rename RemovedComponents::iter/iter_with_id to read/read_with_id][9778]
* [Remove some old references to CoreSet][9833]
* [Use single threaded executor for archetype benches][9835]
* [docs: Improve some `ComponentId` doc cross-linking.][9839]
* [One Shot Systems][8963]
* [Add mutual exclusion safety info on filter_fetch][9836]
* [add try_insert to entity commands][9844]
* [Improve codegen for world validation][9464]
* [docs: Use intradoc links for method references.][9958]
* [Remove States::variants and remove enum-only restriction its derive][9945]
* [`as_deref_mut()` method for Mut-like types][9912]
* [refactor: Change `Option<With<T>>` query params to `Has<T>`][9959]
* [Hide `UnsafeWorldCell::unsafe_world`][9741]
* [Add a public API to ArchetypeGeneration/Id][9825]
* [Ignore ambiguous components or resources][9895]
* [Use chain in breakout example][10124]
* [`ParamSet`s containing non-send parameters should also be non-send][10211]
* [Replace all labels with interned labels][7762]
* [Fix outdated comment referencing CoreSet][10294]

### A-Rendering + A-Math

* [derive Clone/Copy/Debug trio for shape::Cylinder][9705]

### A-UI

* [Fix for vertical text bounds and alignment][9133]
* [UI extraction order fix][9099]
* [Update text example using default font][9259]
* [bevy_ui: fix doc formatting for some Style fields][9295]
* [Remove the `With<Parent>` query filter from `bevy_ui::render::extract_uinode_borders`][9285]
* [Fix incorrent doc comment for the set method of `ContentSize`][9345]
* [Improved text widget doc comments][9344]
* [Change the default for the `measure_func` field of `ContentSize` to None.][9346]
* [Unnecessary line in game_menu example][9406]
* [Change `UiScale` to a tuple struct][9444]
* [Remove unnecessary doc string][9481]
* [Add some missing pub in ui_node][9529]
* [UI examples clean up][9479]
* [`round_ties_up` fix][9548]
* [fix incorrect docs for `JustifyItems` and `JustifySelf`][9539]
* [Added `Val::ZERO` Constant][9566]
* [Cleanup some bevy_text pipeline.rs][9111]
* [Make `GridPlacement`'s fields non-zero and add accessor functions.][9486]
* [Remove `Val`'s `try_*` arithmetic methods][9609]
* [UI node bundle comment fix][9404]
* [Do not panic on non-UI child of UI entity][9621]
* [Rename `Val` `evaluate` to `resolve` and implement viewport variant support][9568]
* [Change `Urect::width` & `Urect::height` to be const][9640]
* [`TextLayoutInfo::size` should hold the drawn size of the text, and not a scaled value.][7794]
* [`impl From<String>` and `From<&str>` for `TextSection`][8856]
* [Remove z-axis scaling in `extract_text2d_sprite`][9733]
* [Fix doc comments for align items][9739]
* [Add tests to `bevy_ui::Layout`][9781]
* [examples: Remove unused doc comments.][9795]
* [Add missing `bevy_text` feature attribute to `TextBundle` from impl][9785]
* [Move `Val` into `geometry`][9818]
* [Derive Serialize and Deserialize for UiRect][9820]
* [`ContentSize` replacement fix][9753]
* [Round UI coordinates after scaling][9784]
* [Have a separate implicit viewport node per root node + make viewport node `Display::Grid`][9637]
* [Rename `num_font_atlases`  to `len`.][9879]
* [Fix documentation for ui node Style][9935]
* [`text_wrap_debug` scale factor commandline args][9951]
* [Store both the rounded and unrounded node size in Node][9923]
* [Various accessibility API updates.][9989]
* [UI node outlines][9931]
* [Implement serialize and deserialize for some UI types][10044]
* [Tidy up UI node docs][10189]
* [Remove unused import warning when default_font feature is disabled][10230]
* [Fix crash with certain right-aligned text][10271]
* [Add some more docs for bevy_text.][9873]
* [Implement `Neg` for `Val`][10295]
* [`normalize` method for `Rect`][10297]
* [don't Implement `Display` for `Val`][10345]
* [[bevy_text] Document what happens when font is not specified][10252]
* [Update UI alignment docs][10303]
* [Add stack index to `Node`][9853]
* [don't Implement `Display` for `Val`][10345]

### A-Animation

* [Fix doc typo][9162]
* [Expose `animation_clip` paths][9392]
* [animations: convert skinning weights from unorm8x4 to float32x4][9338]
* [API updates to the AnimationPlayer][9002]
* [only take up to the max number of joints][9351]
* [check root node for animations][9407]
* [Fix morph interpolation][9927]

### A-Pointers

* [Put `#[repr(transparent)]` attr to bevy_ptr types][9068]

### A-Assets + A-Reflection

* [reflect: `TypePath` part 2][8768]

### A-Rendering + A-Hierarchy

* [default inherited visibility when parent has invalid components][10275]

### A-ECS + A-Tasks

* [Round up for the batch size to improve par_iter performance][9814]

### A-Reflection + A-Utils

* [Moved `fq_std` from `bevy_reflect_derive` to `bevy_macro_utils`][9956]

### A-Reflection + A-Math

* [Add reflect impls to IRect and URect][9191]
* [Implement reflect trait on new glam types (I64Vec and U64Vec)][9281]

### A-Hierarchy

* [Prevent setting parent as itself][8980]
* [Add as_slice to parent][9871]

### A-Input

* [input: allow multiple gamepad inputs to be registered for one button in one frame][9446]
* [Bevy Input Docs : lib.rs][9468]
* [Bevy Input Docs : gamepad.rs][9469]
* [Add `GamepadButtonInput` event][9008]
* [Bevy Input Docs : the modules][9467]
* [Finish documenting `bevy_gilrs`][10010]
* [Change `AxisSettings` livezone default][10090]
* [docs: Update input_toggle_active example][9913]

### A-Input + A-Windowing

* [Fix `Window::set_cursor_position`][9456]
* [Change `Window::physical_cursor_position` to use the physical size of the window][9657]
* [Fix check that cursor position is within window bounds][9662]

### A-ECS + A-Reflection

* [implement insert and remove reflected entity commands][8895]
* [Allow disjoint mutable world access via `EntityMut`][9419]
* [Implement `Reflect` for `State<S>` and `NextState<S>`][9742]
* [`#[derive(Clone)]` on `Component{Info,Descriptor}`][9812]

### A-Math

* [Rename bevy_math::rects conversion methods][9159]
* [Add glam swizzles traits to prelude][9387]
* [Rename `Bezier` to `CubicBezier` for clarity][9554]
* [Add a method to compute a bounding box enclosing a set of points][9630]
* [re-export `debug_glam_assert` feature][10206]
* [Add `Cubic` prefix to all cubic curve generators][10299]

### A-Build-System

* [only check for bans if the dependency tree changed][9252]
* [Slightly better message when contributor modifies examples template][9372]
* [switch CI jobs between windows and linux for example execution][9489]
* [Check for bevy_internal imports in CI][9612]
* [Fix running examples on linux in CI][9665]
* [Bump actions/checkout from 2 to 4][9759]
* [doc: Remove reference to `clippy::manual-strip`.][9794]
* [Only run some workflows on the bevy repo (not forks)][9872]
* [run mobile tests on more devices / OS versions][9936]
* [Allow `clippy::type_complexity` in more places.][9796]
* [hacks for running (and screenshotting) the examples in CI on a github runner][9220]
* [make CI less failing on cargo deny bans][10151]
* [add test on Android 14 / Pixel 8][10148]
* [Use `clippy::doc_markdown` more.][10286]

### A-Diagnostics

* [Cache System Tracing Spans][9390]

### A-Rendering + A-Animation

* [Use a seeded rng for custom_skinned_mesh example][9846]
* [Move skin code to a separate module][9899]

### A-Core

* [Change visibility of `bevy::core::update_frame_count` to `pub`][10111]

### A-Reflection

* [Fix typo in NamedTypePathDef][9102]
* [Refactor `path` module of `bevy_reflect`][8887]
* [Refactor parsing in bevy_reflect path module][9048]
* [bevy_reflect: Fix combined field attributes][9322]
* [bevy_reflect: Opt-out attribute for `TypePath`][9140]
* [Add reflect path parsing benchmark][9364]
* [Make it so `ParsedPath` can be passed to GetPath][9373]
* [Make the reflect path parser utf-8-unaware][9371]
* [bevy_scene: Add `ReflectBundle`][9165]
* [Fix comment in scene example `FromResources`][9743]
* [Remove TypeRegistry re-export rename][9807]
* [Provide getters for fields of ReflectFromPtr][9748]
* [Add TypePath to the prelude][9963]
* [Improve TypeUuid's derive macro error messages][9315]
* [Migrate `Quat` reflection strategy from "value" to "struct"][10068]
* [bevy_reflect: Fix dynamic type serialization][10103]
* [bevy_reflect: Fix ignored/skipped field order][7575]

### A-Rendering + A-Assets + A-Reflection

* [Implement `Reflect` for `Mesh`][9779]

### A-ECS + A-Time

* [add on_real_time_timer run condition][10179]

### A-ECS + A-Hierarchy

* [Added 'clear_children' and 'replace_children' methods to BuildWorldChildren to be consistent with BuildChildren.][10311]

### A-Audio

* [Added Pitch as an alternative sound source][9225]
* [update documentation on AudioSink][9332]
* [audio sinks don't need their custom drop anymore][9336]
* [Clarify what happens when setting the audio volume][9480]
* [More ergonomic spatial audio][9800]

### A-Rendering + A-UI

* [Remove out-of-date paragraph in `Style::border`][9103]
* [Revert "Fix UI corruption for AMD gpus with Vulkan (#9169)"][9237]
* [Revert "Fix UI corruption for AMD gpus with Vulkan (#9169)"][9237]
* [`many_buttons` enhancements][9712]
* [Fix UI borders][10078]
* [UI batching Fix][9610]
* [Add UI Materials][9506]

### A-ECS + A-Reflection + A-Pointers

* [add `MutUntyped::map_unchanged`][9194]

### No area label

* [Fix typos throughout the project][9090]
* [Bump Version after Release][9106]
* [fix `clippy::default_constructed_unit_structs` and trybuild errors][9144]
* [delete code deprecated in 0.11][9128]
* [Drain `ExtractedUiNodes` in `prepare_uinodes`][9142]
* [example showcase - pagination and can build for WebGL2][9168]
* [example showcase: switch default api to webgpu][9193]
* [Add some more helpful errors to BevyManifest when it doesn't find Cargo.toml][9207]
* [Fix path reference to contributors example][9219]
* [replace parens with square brackets when referencing _mut on `Query` docs #9200][9223]
* [use AutoNoVsync in stress tests][9229]
* [bevy_render: Remove direct dep on wgpu-hal.][9249]
* [Fixed typo in line 322][9276]
* [custom_material.vert: gl_InstanceIndex includes gl_BaseInstance][9326]
* [fix typo in a link - Mesh docs][9329]
* [Improve font size related docs][9320]
* [Fix gamepad viewer being marked as a non-wasm example][9399]
* [Rustdoc: Scrape examples][9154]
* [enable multithreading on benches][9388]
* [webgl feature renamed to webgl2][9370]
* [Example Comment Typo Fix][9427]
* [Fix shader_instancing example][9448]
* [Update tracy-client requirement from 0.15 to 0.16][9436]
* [fix bevy imports. windows_settings.rs example][9547]
* [Fix CI for Rust 1.72][9562]
* [Swap TransparentUi to use a stable sort][9598]
* [Replace uses of `entity.insert` with tuple bundles in `game_menu` example][9619]
* [Remove `IntoIterator` impl for `&mut EventReader`][9583]
* [remove VecSwizzles imports][9629]
* [Fix erronenous glam version][9653]
* [Fixing some doc comments][9646]
* [Explicitly make instance_index vertex output @interpolate(flat)][9675]
* [Fix some nightly warnings][9672]
* [Use default resolution for viewport_debug example][9666]
* [Refer to "macOS", not "macOS X".][9704]
* [Remove useless single tuples and trailing commas][9720]
* [Fix some warnings shown in nightly][10012]
* [Fix animate_scale scaling z value in text2d example][9769]
* ["serialize" feature no longer enables the optional "bevy_scene" feature if it's not enabled from elsewhere][9803]
* [fix deprecation warning in bench][9823]
* [don't enable filesystem_watcher when building for WebGPU][9829]
* [Improve doc formatting.][9840]
* [Fix the `clippy::explicit_iter_loop` lint][9834]
* [Wslg docs][9842]
* [skybox.wgsl: Fix precision issues][9909]
* [Fix typos.][9922]
* [Add link to `Text2dBundle` in `TextBundle` docs.][9900]
* [Fix some typos][9934]
* [Fix typos][9965]
* [Replaced `parking_lot` with `std::sync`][9545]
* [Add inline(never) to bench systems][9824]
* [Android: handle suspend / resume][9937]
* [Fix some warnings shown in nightly][10012]
* [Updates for rust 1.73][10035]
* [Improve selection of iOS device in mobile example][9282]
* [Update toml_edit requirement from 0.19 to 0.20][10058]
* [foxes shouldn't march in sync][10070]
* [Fix tonemapping test patten][10092]
* [Removed `once_cell`][10079]
* [Improve WebGPU unstable flags docs][10163]
* [shadow_biases: Support different PCF methods][10184]
* [shadow_biases: Support moving the light position and resetting biases][10185]
* [Update async-io requirement from 1.13.0 to 2.0.0][10238]
* [few fmt tweaks][10264]
* [Derive Error for more error types][10240]
* [Allow AccessKit to react to WindowEvents before they reach the engine][10356]

### A-Rendering + A-Build-System

* [Improve execution of examples in CI][9331]
* [make deferred_rendering simpler to render for CI][10150]

### A-Meta

* [Remove the bevy_dylib feature][9516]
* [add and fix shields in Readmes][9993]
* [Added section for contributing and links for issues and PRs][10171]
* [Fix orphaned contributing paragraph][10174]

### A-Assets + A-Animation

* [Handle empty morph weights when loading gltf][9867]
* [Finish documenting `bevy_gltf`][9998]

### A-Editor + A-Diagnostics

* [Add `DiagnosticsStore::iter_mut`][9679]

### A-Time

* [Fix timers.rs documentation][9290]
* [Add missing documentation to `bevy_time`][9428]
* [Clarify behaviour of `Timer::finished()` for repeating timers][9939]
* [ignore time channel error][9981]
* [Unify `FixedTime` and `Time` while fixing several problems][8964]
* [Time: demote delta time clamping warning to debug][10145]
* [fix typo in time.rs example][10152]
* [Example time api][10204]

### A-Rendering + A-ECS

* [Update `Camera`'s `Frustum` only when its `GlobalTransform` or `CameraProjection` changed][9092]

### A-UI + A-Reflection

* [bevy_ui: reflect missing types][9677]
* [register `TextLayoutInfo` and `TextFlags` type.][9919]

### A-Build-System + A-Assets

* [Increase iteration count for asset tests][9737]

### A-Rendering

* [Clarify that wgpu is based on the webGPU API][9093]
* [Return URect instead of (UVec2, UVec2) in Camera::physical_viewport_rect][9085]
* [fix module name for AssetPath shaders][9186]
* [Add GpuArrayBuffer and BatchedUniformBuffer][8204]
* [Update `bevy_window::PresentMode` to mirror `wgpu::PresentMode`][9230]
* [Stop using unwrap in the pipelined rendering thread][9052]
* [Fix panic whilst loading UASTC encoded ktx2 textures][9158]
* [Document `ClearColorConfig`][9288]
* [Use GpuArrayBuffer for MeshUniform][9254]
* [Update docs for scaling_mode field of Orthographic projection][9297]
* [Fix shader_material_glsl example after #9254][9311]
* [Improve `Mesh` documentation][9061]
* [Include tone_mapping fn in tonemapping_test_patterns][9084]
* [Extend the default render range of 2D camera][9310]
* [Document when Camera::viewport_to_world and related methods return None][8841]
* [include toplevel shader-associated defs][9343]
* [Fix post_processing example on webgl2][9361]
* [use ViewNodeRunner in the post_processing example][9127]
* [Work around naga/wgpu WGSL instance_index -> GLSL gl_InstanceID bug on WebGL2][9383]
* [Fix non-visible motion vector text in shader prepass example][9155]
* [Use bevy crates imports instead of bevy internal. post_processing example][9396]
* [Make Anchor Copy][9327]
* [Move window.rs to window/mod.rs in bevy_render][9394]
* [Reduce the size of MeshUniform to improve performance][9416]
* [Fix temporal jitter bug][9462]
* [Fix gizmo lines deforming or disappearing when partially behind the camera][9470]
* [Make WgpuSettings::default() check WGPU_POWER_PREF][9482]
* [fix wireframe after MeshUniform size reduction][9505]
* [fix shader_material_glsl example][9513]
* [[RAINBOW EFFECT] Added methods to get HSL components from Color][9201]
* [ktx2: Fix Rgb8 -> Rgba8Unorm conversion][9555]
* [Reorder render sets, refactor bevy_sprite to take advantage][9236]
* [Improve documentation relating to `Frustum` and `HalfSpace`][9136]
* [Revert "Update defaults for OrthographicProjection (#9537)"][9878]
* [Remove unused regex dep from bevy_render][9613]
* [Split `ComputedVisibility` into two components to allow for accurate change detection and speed up visibility propagation][9497]
* [Use instancing for sprites][9597]
* [Enhance bevymark][9674]
* [Remove redundant math in tonemapping.][9669]
* [Improve `SpatialBundle` docs][9673]
* [Cache depth texture based on usage][9565]
* [warn and min for different vertex count][9699]
* [default 16bit rgb/rgba textures to unorm instead of uint][9611]
* [Fix TextureAtlasBuilder padding][10031]
* [Add example for `Camera::viewport_to_world`][7179]
* [Fix wireframe for skinned/morphed meshes][9734]
* [generate indices for Mikktspace][8862]
* [invert face culling for negatively scaled gltf nodes][8859]
* [renderer init: create a detached task only on wasm, block otherwise][9830]
* [Cleanup `visibility` module][9850]
* [Use a single line for of large binding lists][9849]
* [Fix a typo in `DirectionalLightBundle`][9861]
* [Revert "Update defaults for OrthographicProjection (#9537)"][9878]
* [Refactor rendering systems to use `let-else`][9870]
* [Use radsort for Transparent2d PhaseItem sorting][9882]
* [Automatic batching/instancing of draw commands][9685]
* [Directly copy data into uniform buffers][9865]
* [Allow other plugins to create renderer resources][9925]
* [Use EntityHashMap<Entity, T> for render world entity storage for better performance][9903]
* [Parallelize extract_meshes][9966]
* [Fix comment grammar][9990]
* [Allow overriding global wireframe setting.][7328]
* [wireframes: workaround for DX12][10022]
* [Alternate wireframe override api][10023]
* [Fix TextureAtlasBuilder padding][10031]
* [fix example mesh2d_manual][9941]
* [PCF For DirectionalLight/SpotLight Shadows][8006]
* [Refactor the render instance logic in #9903 so that it's easier for other components to adopt.][10002]
* [Fix 2d_shapes and general 2D mesh instancing][10051]
* [fix webgl2 crash][10053]
* [fix orthographic cluster aabb for spotlight culling][9614]
* [Add consuming builder methods for more ergonomic `Mesh` creation][10056]
* [wgpu 0.17][9302]
* [use `Material` for wireframes][5314]
* [Extract common wireframe filters in type alias][10080]
* [Deferred Renderer][9258]
* [Configurable colors for wireframe][5303]
* [chore: Renamed RenderInstance trait to ExtractInstance][10065]
* [pbr shader cleanup][10105]
* [Fix text2d view-visibility][10100]
* [Allow optional extraction of resources from the main world][10109]
* [ssao use unlit_color instead of white][10117]
* [Fix missing explicit lifetime name for copy_deferred_lighting_id name][10128]
* [Fixed mod.rs in rendering to support Radeon Cards][10132]
* [Explain usage of prepass shaders in docs for `Material` trait][9025]
* [Better link for prepare_windows docs][10142]
* [Improve linking within `RenderSet` docs.][10143]
* [Fix unlit missing parameters][10144]
* [`*_PREPASS` Shader Def Cleanup][10136]
* [check for any prepass phase][10160]
* [allow extensions to StandardMaterial][7820]
* [array_texture example: use new name of pbr function][10168]
* [chore: use ExtractComponent derive macro for EnvironmentMapLight and FogSettings][10191]
* [Variable `MeshPipeline` View Bind Group Layout][10156]
* [update shader imports][10180]
* [Bind group entries][9694]
* [Detect cubemap for dds textures][10222]
* [Fix alignment on ios simulator][10178]
* [Add convenient methods for Image][10221]
* [Use “specular occlusion” term to consistently extinguish fresnel on Ambient and Environment Map lights][10182]
* [Fix fog color being inaccurate][10226]
* [Replace all usages of texture_descritor.size.* with the helper methods][10227]
* [View Transformations][9726]
* [fix deferred example fog values][10249]
* [WebGL2: fix import path for unpack_unorm3x4_plus_unorm_20_][10251]
* [Use wildcard imports in bevy_pbr][9847]
* [Make mesh attr vertex count mismatch warn more readable][10259]
* [Image Sampler Improvements][10254]
* [Fix sampling of diffuse env map texture with non-uniform control flow][10276]
* [Log a warning when the `tonemapping_luts` feature is disabled but required for the selected tonemapper.][10253]
* [Smaller TAA fixes][10200]
* [Truncate attribute buffer data rather than attribute buffers][10270]
* [Fix deferred lighting pass values not all working on M1 in WebGL2][10304]
* [Add frustum to shader View][10306]
* [Fix handling of `double_sided` for normal maps][10326]
* [Add helper function to determine if color is transparent][10310]
* [`StandardMaterial` Light Transmission][8015]
* [double sided normals: fix apply_normal_mapping calls][10330]
* [Combine visibility queries in check_visibility_system][10196]
* [Make VERTEX_COLORS usable in prepass shader, if available][10341]
* [allow DeferredPrepass to work without other prepass markers][10223]
* [Increase default normal bias to avoid common artifacts][10346]
* [Make `DirectionalLight` `Cascades` computation generic over `CameraProjection`][9226]
* [Update default `ClearColor` to better match Bevy's branding][10339]
* [Fix gizmo crash when prepass enabled][10360]

### A-Build-System + A-Meta

* [Fixed: README.md][9994]

### A-Assets

* [doc(asset): fix asset trait example][9105]
* [Add `GltfLoader::new`.][9120]
* [impl `From<&AssetPath>` for `HandleId`][9132]
* [allow asset loader pre-registration][9429]
* [fix asset loader preregistration for multiple assets][9453]
* [Fix point light radius][9493]
* [Add support for KHR_materials_emissive_strength][9553]
* [Fix panic when using `.load_folder()` with absolute paths][9490]
* [Bevy Asset V2][8624]
* [create imported asset directory if needed][9716]
* [Copy on Write AssetPaths][9729]
* [Asset v2: Asset path serialization fix][9756]
* [don't ignore some EventKind::Modify][9767]
* [Manual "Reflect Value" AssetPath impl to fix dynamic linking][9752]
* [Fix unused variable warning for simple AssetV2 derives][9961]
* [Remove monkey.gltf][9974]
* [Update notify-debouncer-full requirement from 0.2.0 to 0.3.1][9757]
* [Removed `anyhow`][10003]
* [Multiple Asset Sources][9885]
* [Make loading warning for no file ext more descriptive][10119]
* [Fix load_folder for non-default Asset Sources][10121]
* [only set up processed source if asset plugin is not unprocessed][10123]
* [Hot reload labeled assets whose source asset is not loaded][9736]
* [Return an error when loading non-existent labels][9751]
* [remove unused import on android][10197]
* [Log an error when registering an AssetSource after AssetPlugin has been built][10202]
* [Add note about asset source register order][10186]
* [Add `asset_processor` feature and remove AssetMode::ProcessedDev][10194]
* [Implement source into Display for AssetPath][10217]
* [assets: use blake3 instead of md5][10208]
* [Reduce noise in asset processing example][10262]
* [Adding AssetPath::resolve() method.][9528]
* [Assets: fix first hot reloading][9804]
* [Non-blocking load_untyped using a wrapper asset][10198]
* [Reuse and hot reload folder handles][10210]
* [Additional AssetPath unit tests.][10279]
* [Corrected incorrect doc comment on read_asset_bytes][10352]
* [support file operations in single threaded context][10312]

[5303]: https://github.com/bevyengine/bevy/pull/5303
[5314]: https://github.com/bevyengine/bevy/pull/5314
[7179]: https://github.com/bevyengine/bevy/pull/7179
[7328]: https://github.com/bevyengine/bevy/pull/7328
[7575]: https://github.com/bevyengine/bevy/pull/7575
[7609]: https://github.com/bevyengine/bevy/pull/7609
[7611]: https://github.com/bevyengine/bevy/pull/7611
[7728]: https://github.com/bevyengine/bevy/pull/7728
[7762]: https://github.com/bevyengine/bevy/pull/7762
[7794]: https://github.com/bevyengine/bevy/pull/7794
[7820]: https://github.com/bevyengine/bevy/pull/7820
[8006]: https://github.com/bevyengine/bevy/pull/8006
[8015]: https://github.com/bevyengine/bevy/pull/8015
[8204]: https://github.com/bevyengine/bevy/pull/8204
[8246]: https://github.com/bevyengine/bevy/pull/8246
[8526]: https://github.com/bevyengine/bevy/pull/8526
[8603]: https://github.com/bevyengine/bevy/pull/8603
[8624]: https://github.com/bevyengine/bevy/pull/8624
[8750]: https://github.com/bevyengine/bevy/pull/8750
[8768]: https://github.com/bevyengine/bevy/pull/8768
[8841]: https://github.com/bevyengine/bevy/pull/8841
[8854]: https://github.com/bevyengine/bevy/pull/8854
[8855]: https://github.com/bevyengine/bevy/pull/8855
[8856]: https://github.com/bevyengine/bevy/pull/8856
[8859]: https://github.com/bevyengine/bevy/pull/8859
[8862]: https://github.com/bevyengine/bevy/pull/8862
[8887]: https://github.com/bevyengine/bevy/pull/8887
[8895]: https://github.com/bevyengine/bevy/pull/8895
[8963]: https://github.com/bevyengine/bevy/pull/8963
[8964]: https://github.com/bevyengine/bevy/pull/8964
[8980]: https://github.com/bevyengine/bevy/pull/8980
[9002]: https://github.com/bevyengine/bevy/pull/9002
[9008]: https://github.com/bevyengine/bevy/pull/9008
[9025]: https://github.com/bevyengine/bevy/pull/9025
[9048]: https://github.com/bevyengine/bevy/pull/9048
[9052]: https://github.com/bevyengine/bevy/pull/9052
[9061]: https://github.com/bevyengine/bevy/pull/9061
[9067]: https://github.com/bevyengine/bevy/pull/9067
[9068]: https://github.com/bevyengine/bevy/pull/9068
[9081]: https://github.com/bevyengine/bevy/pull/9081
[9083]: https://github.com/bevyengine/bevy/pull/9083
[9084]: https://github.com/bevyengine/bevy/pull/9084
[9085]: https://github.com/bevyengine/bevy/pull/9085
[9090]: https://github.com/bevyengine/bevy/pull/9090
[9092]: https://github.com/bevyengine/bevy/pull/9092
[9093]: https://github.com/bevyengine/bevy/pull/9093
[9099]: https://github.com/bevyengine/bevy/pull/9099
[9101]: https://github.com/bevyengine/bevy/pull/9101
[9102]: https://github.com/bevyengine/bevy/pull/9102
[9103]: https://github.com/bevyengine/bevy/pull/9103
[9105]: https://github.com/bevyengine/bevy/pull/9105
[9106]: https://github.com/bevyengine/bevy/pull/9106
[9111]: https://github.com/bevyengine/bevy/pull/9111
[9120]: https://github.com/bevyengine/bevy/pull/9120
[9127]: https://github.com/bevyengine/bevy/pull/9127
[9128]: https://github.com/bevyengine/bevy/pull/9128
[9129]: https://github.com/bevyengine/bevy/pull/9129
[9132]: https://github.com/bevyengine/bevy/pull/9132
[9133]: https://github.com/bevyengine/bevy/pull/9133
[9136]: https://github.com/bevyengine/bevy/pull/9136
[9140]: https://github.com/bevyengine/bevy/pull/9140
[9142]: https://github.com/bevyengine/bevy/pull/9142
[9144]: https://github.com/bevyengine/bevy/pull/9144
[9154]: https://github.com/bevyengine/bevy/pull/9154
[9155]: https://github.com/bevyengine/bevy/pull/9155
[9158]: https://github.com/bevyengine/bevy/pull/9158
[9159]: https://github.com/bevyengine/bevy/pull/9159
[9162]: https://github.com/bevyengine/bevy/pull/9162
[9165]: https://github.com/bevyengine/bevy/pull/9165
[9166]: https://github.com/bevyengine/bevy/pull/9166
[9168]: https://github.com/bevyengine/bevy/pull/9168
[9173]: https://github.com/bevyengine/bevy/pull/9173
[9174]: https://github.com/bevyengine/bevy/pull/9174
[9175]: https://github.com/bevyengine/bevy/pull/9175
[9183]: https://github.com/bevyengine/bevy/pull/9183
[9186]: https://github.com/bevyengine/bevy/pull/9186
[9191]: https://github.com/bevyengine/bevy/pull/9191
[9193]: https://github.com/bevyengine/bevy/pull/9193
[9194]: https://github.com/bevyengine/bevy/pull/9194
[9201]: https://github.com/bevyengine/bevy/pull/9201
[9204]: https://github.com/bevyengine/bevy/pull/9204
[9207]: https://github.com/bevyengine/bevy/pull/9207
[9219]: https://github.com/bevyengine/bevy/pull/9219
[9220]: https://github.com/bevyengine/bevy/pull/9220
[9222]: https://github.com/bevyengine/bevy/pull/9222
[9223]: https://github.com/bevyengine/bevy/pull/9223
[9225]: https://github.com/bevyengine/bevy/pull/9225
[9226]: https://github.com/bevyengine/bevy/pull/9226
[9229]: https://github.com/bevyengine/bevy/pull/9229
[9230]: https://github.com/bevyengine/bevy/pull/9230
[9236]: https://github.com/bevyengine/bevy/pull/9236
[9237]: https://github.com/bevyengine/bevy/pull/9237
[9247]: https://github.com/bevyengine/bevy/pull/9247
[9249]: https://github.com/bevyengine/bevy/pull/9249
[9252]: https://github.com/bevyengine/bevy/pull/9252
[9253]: https://github.com/bevyengine/bevy/pull/9253
[9254]: https://github.com/bevyengine/bevy/pull/9254
[9258]: https://github.com/bevyengine/bevy/pull/9258
[9259]: https://github.com/bevyengine/bevy/pull/9259
[9260]: https://github.com/bevyengine/bevy/pull/9260
[9269]: https://github.com/bevyengine/bevy/pull/9269
[9276]: https://github.com/bevyengine/bevy/pull/9276
[9281]: https://github.com/bevyengine/bevy/pull/9281
[9282]: https://github.com/bevyengine/bevy/pull/9282
[9285]: https://github.com/bevyengine/bevy/pull/9285
[9288]: https://github.com/bevyengine/bevy/pull/9288
[9290]: https://github.com/bevyengine/bevy/pull/9290
[9291]: https://github.com/bevyengine/bevy/pull/9291
[9293]: https://github.com/bevyengine/bevy/pull/9293
[9295]: https://github.com/bevyengine/bevy/pull/9295
[9297]: https://github.com/bevyengine/bevy/pull/9297
[9301]: https://github.com/bevyengine/bevy/pull/9301
[9302]: https://github.com/bevyengine/bevy/pull/9302
[9306]: https://github.com/bevyengine/bevy/pull/9306
[9310]: https://github.com/bevyengine/bevy/pull/9310
[9311]: https://github.com/bevyengine/bevy/pull/9311
[9312]: https://github.com/bevyengine/bevy/pull/9312
[9313]: https://github.com/bevyengine/bevy/pull/9313
[9315]: https://github.com/bevyengine/bevy/pull/9315
[9320]: https://github.com/bevyengine/bevy/pull/9320
[9322]: https://github.com/bevyengine/bevy/pull/9322
[9326]: https://github.com/bevyengine/bevy/pull/9326
[9327]: https://github.com/bevyengine/bevy/pull/9327
[9329]: https://github.com/bevyengine/bevy/pull/9329
[9331]: https://github.com/bevyengine/bevy/pull/9331
[9332]: https://github.com/bevyengine/bevy/pull/9332
[9336]: https://github.com/bevyengine/bevy/pull/9336
[9338]: https://github.com/bevyengine/bevy/pull/9338
[9343]: https://github.com/bevyengine/bevy/pull/9343
[9344]: https://github.com/bevyengine/bevy/pull/9344
[9345]: https://github.com/bevyengine/bevy/pull/9345
[9346]: https://github.com/bevyengine/bevy/pull/9346
[9348]: https://github.com/bevyengine/bevy/pull/9348
[9351]: https://github.com/bevyengine/bevy/pull/9351
[9355]: https://github.com/bevyengine/bevy/pull/9355
[9360]: https://github.com/bevyengine/bevy/pull/9360
[9361]: https://github.com/bevyengine/bevy/pull/9361
[9364]: https://github.com/bevyengine/bevy/pull/9364
[9366]: https://github.com/bevyengine/bevy/pull/9366
[9370]: https://github.com/bevyengine/bevy/pull/9370
[9371]: https://github.com/bevyengine/bevy/pull/9371
[9372]: https://github.com/bevyengine/bevy/pull/9372
[9373]: https://github.com/bevyengine/bevy/pull/9373
[9383]: https://github.com/bevyengine/bevy/pull/9383
[9385]: https://github.com/bevyengine/bevy/pull/9385
[9387]: https://github.com/bevyengine/bevy/pull/9387
[9388]: https://github.com/bevyengine/bevy/pull/9388
[9390]: https://github.com/bevyengine/bevy/pull/9390
[9392]: https://github.com/bevyengine/bevy/pull/9392
[9394]: https://github.com/bevyengine/bevy/pull/9394
[9396]: https://github.com/bevyengine/bevy/pull/9396
[9399]: https://github.com/bevyengine/bevy/pull/9399
[9404]: https://github.com/bevyengine/bevy/pull/9404
[9406]: https://github.com/bevyengine/bevy/pull/9406
[9407]: https://github.com/bevyengine/bevy/pull/9407
[9416]: https://github.com/bevyengine/bevy/pull/9416
[9418]: https://github.com/bevyengine/bevy/pull/9418
[9419]: https://github.com/bevyengine/bevy/pull/9419
[9421]: https://github.com/bevyengine/bevy/pull/9421
[9427]: https://github.com/bevyengine/bevy/pull/9427
[9428]: https://github.com/bevyengine/bevy/pull/9428
[9429]: https://github.com/bevyengine/bevy/pull/9429
[9431]: https://github.com/bevyengine/bevy/pull/9431
[9436]: https://github.com/bevyengine/bevy/pull/9436
[9444]: https://github.com/bevyengine/bevy/pull/9444
[9446]: https://github.com/bevyengine/bevy/pull/9446
[9448]: https://github.com/bevyengine/bevy/pull/9448
[9451]: https://github.com/bevyengine/bevy/pull/9451
[9453]: https://github.com/bevyengine/bevy/pull/9453
[9456]: https://github.com/bevyengine/bevy/pull/9456
[9460]: https://github.com/bevyengine/bevy/pull/9460
[9461]: https://github.com/bevyengine/bevy/pull/9461
[9462]: https://github.com/bevyengine/bevy/pull/9462
[9463]: https://github.com/bevyengine/bevy/pull/9463
[9464]: https://github.com/bevyengine/bevy/pull/9464
[9467]: https://github.com/bevyengine/bevy/pull/9467
[9468]: https://github.com/bevyengine/bevy/pull/9468
[9469]: https://github.com/bevyengine/bevy/pull/9469
[9470]: https://github.com/bevyengine/bevy/pull/9470
[9479]: https://github.com/bevyengine/bevy/pull/9479
[9480]: https://github.com/bevyengine/bevy/pull/9480
[9481]: https://github.com/bevyengine/bevy/pull/9481
[9482]: https://github.com/bevyengine/bevy/pull/9482
[9484]: https://github.com/bevyengine/bevy/pull/9484
[9485]: https://github.com/bevyengine/bevy/pull/9485
[9486]: https://github.com/bevyengine/bevy/pull/9486
[9489]: https://github.com/bevyengine/bevy/pull/9489
[9490]: https://github.com/bevyengine/bevy/pull/9490
[9493]: https://github.com/bevyengine/bevy/pull/9493
[9497]: https://github.com/bevyengine/bevy/pull/9497
[9505]: https://github.com/bevyengine/bevy/pull/9505
[9506]: https://github.com/bevyengine/bevy/pull/9506
[9513]: https://github.com/bevyengine/bevy/pull/9513
[9514]: https://github.com/bevyengine/bevy/pull/9514
[9516]: https://github.com/bevyengine/bevy/pull/9516
[9518]: https://github.com/bevyengine/bevy/pull/9518
[9528]: https://github.com/bevyengine/bevy/pull/9528
[9529]: https://github.com/bevyengine/bevy/pull/9529
[9539]: https://github.com/bevyengine/bevy/pull/9539
[9542]: https://github.com/bevyengine/bevy/pull/9542
[9545]: https://github.com/bevyengine/bevy/pull/9545
[9547]: https://github.com/bevyengine/bevy/pull/9547
[9548]: https://github.com/bevyengine/bevy/pull/9548
[9551]: https://github.com/bevyengine/bevy/pull/9551
[9553]: https://github.com/bevyengine/bevy/pull/9553
[9554]: https://github.com/bevyengine/bevy/pull/9554
[9555]: https://github.com/bevyengine/bevy/pull/9555
[9562]: https://github.com/bevyengine/bevy/pull/9562
[9565]: https://github.com/bevyengine/bevy/pull/9565
[9566]: https://github.com/bevyengine/bevy/pull/9566
[9568]: https://github.com/bevyengine/bevy/pull/9568
[9576]: https://github.com/bevyengine/bevy/pull/9576
[9577]: https://github.com/bevyengine/bevy/pull/9577
[9579]: https://github.com/bevyengine/bevy/pull/9579
[9581]: https://github.com/bevyengine/bevy/pull/9581
[9583]: https://github.com/bevyengine/bevy/pull/9583
[9587]: https://github.com/bevyengine/bevy/pull/9587
[9588]: https://github.com/bevyengine/bevy/pull/9588
[9589]: https://github.com/bevyengine/bevy/pull/9589
[9592]: https://github.com/bevyengine/bevy/pull/9592
[9596]: https://github.com/bevyengine/bevy/pull/9596
[9597]: https://github.com/bevyengine/bevy/pull/9597
[9598]: https://github.com/bevyengine/bevy/pull/9598
[9600]: https://github.com/bevyengine/bevy/pull/9600
[9604]: https://github.com/bevyengine/bevy/pull/9604
[9609]: https://github.com/bevyengine/bevy/pull/9609
[9610]: https://github.com/bevyengine/bevy/pull/9610
[9611]: https://github.com/bevyengine/bevy/pull/9611
[9612]: https://github.com/bevyengine/bevy/pull/9612
[9613]: https://github.com/bevyengine/bevy/pull/9613
[9614]: https://github.com/bevyengine/bevy/pull/9614
[9617]: https://github.com/bevyengine/bevy/pull/9617
[9619]: https://github.com/bevyengine/bevy/pull/9619
[9621]: https://github.com/bevyengine/bevy/pull/9621
[9626]: https://github.com/bevyengine/bevy/pull/9626
[9629]: https://github.com/bevyengine/bevy/pull/9629
[9630]: https://github.com/bevyengine/bevy/pull/9630
[9631]: https://github.com/bevyengine/bevy/pull/9631
[9637]: https://github.com/bevyengine/bevy/pull/9637
[9638]: https://github.com/bevyengine/bevy/pull/9638
[9640]: https://github.com/bevyengine/bevy/pull/9640
[9646]: https://github.com/bevyengine/bevy/pull/9646
[9648]: https://github.com/bevyengine/bevy/pull/9648
[9650]: https://github.com/bevyengine/bevy/pull/9650
[9653]: https://github.com/bevyengine/bevy/pull/9653
[9657]: https://github.com/bevyengine/bevy/pull/9657
[9659]: https://github.com/bevyengine/bevy/pull/9659
[9662]: https://github.com/bevyengine/bevy/pull/9662
[9665]: https://github.com/bevyengine/bevy/pull/9665
[9666]: https://github.com/bevyengine/bevy/pull/9666
[9669]: https://github.com/bevyengine/bevy/pull/9669
[9672]: https://github.com/bevyengine/bevy/pull/9672
[9673]: https://github.com/bevyengine/bevy/pull/9673
[9674]: https://github.com/bevyengine/bevy/pull/9674
[9675]: https://github.com/bevyengine/bevy/pull/9675
[9677]: https://github.com/bevyengine/bevy/pull/9677
[9679]: https://github.com/bevyengine/bevy/pull/9679
[9684]: https://github.com/bevyengine/bevy/pull/9684
[9685]: https://github.com/bevyengine/bevy/pull/9685
[9686]: https://github.com/bevyengine/bevy/pull/9686
[9692]: https://github.com/bevyengine/bevy/pull/9692
[9694]: https://github.com/bevyengine/bevy/pull/9694
[9697]: https://github.com/bevyengine/bevy/pull/9697
[9699]: https://github.com/bevyengine/bevy/pull/9699
[9704]: https://github.com/bevyengine/bevy/pull/9704
[9705]: https://github.com/bevyengine/bevy/pull/9705
[9712]: https://github.com/bevyengine/bevy/pull/9712
[9716]: https://github.com/bevyengine/bevy/pull/9716
[9720]: https://github.com/bevyengine/bevy/pull/9720
[9726]: https://github.com/bevyengine/bevy/pull/9726
[9729]: https://github.com/bevyengine/bevy/pull/9729
[9733]: https://github.com/bevyengine/bevy/pull/9733
[9734]: https://github.com/bevyengine/bevy/pull/9734
[9736]: https://github.com/bevyengine/bevy/pull/9736
[9737]: https://github.com/bevyengine/bevy/pull/9737
[9739]: https://github.com/bevyengine/bevy/pull/9739
[9741]: https://github.com/bevyengine/bevy/pull/9741
[9742]: https://github.com/bevyengine/bevy/pull/9742
[9743]: https://github.com/bevyengine/bevy/pull/9743
[9744]: https://github.com/bevyengine/bevy/pull/9744
[9748]: https://github.com/bevyengine/bevy/pull/9748
[9749]: https://github.com/bevyengine/bevy/pull/9749
[9751]: https://github.com/bevyengine/bevy/pull/9751
[9752]: https://github.com/bevyengine/bevy/pull/9752
[9753]: https://github.com/bevyengine/bevy/pull/9753
[9756]: https://github.com/bevyengine/bevy/pull/9756
[9757]: https://github.com/bevyengine/bevy/pull/9757
[9759]: https://github.com/bevyengine/bevy/pull/9759
[9767]: https://github.com/bevyengine/bevy/pull/9767
[9769]: https://github.com/bevyengine/bevy/pull/9769
[9778]: https://github.com/bevyengine/bevy/pull/9778
[9779]: https://github.com/bevyengine/bevy/pull/9779
[9781]: https://github.com/bevyengine/bevy/pull/9781
[9784]: https://github.com/bevyengine/bevy/pull/9784
[9785]: https://github.com/bevyengine/bevy/pull/9785
[9794]: https://github.com/bevyengine/bevy/pull/9794
[9795]: https://github.com/bevyengine/bevy/pull/9795
[9796]: https://github.com/bevyengine/bevy/pull/9796
[9800]: https://github.com/bevyengine/bevy/pull/9800
[9801]: https://github.com/bevyengine/bevy/pull/9801
[9803]: https://github.com/bevyengine/bevy/pull/9803
[9804]: https://github.com/bevyengine/bevy/pull/9804
[9807]: https://github.com/bevyengine/bevy/pull/9807
[9812]: https://github.com/bevyengine/bevy/pull/9812
[9814]: https://github.com/bevyengine/bevy/pull/9814
[9818]: https://github.com/bevyengine/bevy/pull/9818
[9820]: https://github.com/bevyengine/bevy/pull/9820
[9823]: https://github.com/bevyengine/bevy/pull/9823
[9824]: https://github.com/bevyengine/bevy/pull/9824
[9825]: https://github.com/bevyengine/bevy/pull/9825
[9826]: https://github.com/bevyengine/bevy/pull/9826
[9829]: https://github.com/bevyengine/bevy/pull/9829
[9830]: https://github.com/bevyengine/bevy/pull/9830
[9833]: https://github.com/bevyengine/bevy/pull/9833
[9834]: https://github.com/bevyengine/bevy/pull/9834
[9835]: https://github.com/bevyengine/bevy/pull/9835
[9836]: https://github.com/bevyengine/bevy/pull/9836
[9839]: https://github.com/bevyengine/bevy/pull/9839
[9840]: https://github.com/bevyengine/bevy/pull/9840
[9842]: https://github.com/bevyengine/bevy/pull/9842
[9844]: https://github.com/bevyengine/bevy/pull/9844
[9845]: https://github.com/bevyengine/bevy/pull/9845
[9846]: https://github.com/bevyengine/bevy/pull/9846
[9847]: https://github.com/bevyengine/bevy/pull/9847
[9849]: https://github.com/bevyengine/bevy/pull/9849
[9850]: https://github.com/bevyengine/bevy/pull/9850
[9851]: https://github.com/bevyengine/bevy/pull/9851
[9853]: https://github.com/bevyengine/bevy/pull/9853
[9861]: https://github.com/bevyengine/bevy/pull/9861
[9865]: https://github.com/bevyengine/bevy/pull/9865
[9867]: https://github.com/bevyengine/bevy/pull/9867
[9870]: https://github.com/bevyengine/bevy/pull/9870
[9871]: https://github.com/bevyengine/bevy/pull/9871
[9872]: https://github.com/bevyengine/bevy/pull/9872
[9873]: https://github.com/bevyengine/bevy/pull/9873
[9878]: https://github.com/bevyengine/bevy/pull/9878
[9879]: https://github.com/bevyengine/bevy/pull/9879
[9881]: https://github.com/bevyengine/bevy/pull/9881
[9882]: https://github.com/bevyengine/bevy/pull/9882
[9885]: https://github.com/bevyengine/bevy/pull/9885
[9895]: https://github.com/bevyengine/bevy/pull/9895
[9899]: https://github.com/bevyengine/bevy/pull/9899
[9900]: https://github.com/bevyengine/bevy/pull/9900
[9903]: https://github.com/bevyengine/bevy/pull/9903
[9909]: https://github.com/bevyengine/bevy/pull/9909
[9912]: https://github.com/bevyengine/bevy/pull/9912
[9913]: https://github.com/bevyengine/bevy/pull/9913
[9919]: https://github.com/bevyengine/bevy/pull/9919
[9922]: https://github.com/bevyengine/bevy/pull/9922
[9923]: https://github.com/bevyengine/bevy/pull/9923
[9925]: https://github.com/bevyengine/bevy/pull/9925
[9927]: https://github.com/bevyengine/bevy/pull/9927
[9931]: https://github.com/bevyengine/bevy/pull/9931
[9933]: https://github.com/bevyengine/bevy/pull/9933
[9934]: https://github.com/bevyengine/bevy/pull/9934
[9935]: https://github.com/bevyengine/bevy/pull/9935
[9936]: https://github.com/bevyengine/bevy/pull/9936
[9937]: https://github.com/bevyengine/bevy/pull/9937
[9939]: https://github.com/bevyengine/bevy/pull/9939
[9941]: https://github.com/bevyengine/bevy/pull/9941
[9945]: https://github.com/bevyengine/bevy/pull/9945
[9949]: https://github.com/bevyengine/bevy/pull/9949
[9950]: https://github.com/bevyengine/bevy/pull/9950
[9951]: https://github.com/bevyengine/bevy/pull/9951
[9956]: https://github.com/bevyengine/bevy/pull/9956
[9958]: https://github.com/bevyengine/bevy/pull/9958
[9959]: https://github.com/bevyengine/bevy/pull/9959
[9961]: https://github.com/bevyengine/bevy/pull/9961
[9963]: https://github.com/bevyengine/bevy/pull/9963
[9965]: https://github.com/bevyengine/bevy/pull/9965
[9966]: https://github.com/bevyengine/bevy/pull/9966
[9974]: https://github.com/bevyengine/bevy/pull/9974
[9981]: https://github.com/bevyengine/bevy/pull/9981
[9982]: https://github.com/bevyengine/bevy/pull/9982
[9984]: https://github.com/bevyengine/bevy/pull/9984
[9989]: https://github.com/bevyengine/bevy/pull/9989
[9990]: https://github.com/bevyengine/bevy/pull/9990
[9991]: https://github.com/bevyengine/bevy/pull/9991
[9992]: https://github.com/bevyengine/bevy/pull/9992
[9993]: https://github.com/bevyengine/bevy/pull/9993
[9994]: https://github.com/bevyengine/bevy/pull/9994
[9998]: https://github.com/bevyengine/bevy/pull/9998
[10001]: https://github.com/bevyengine/bevy/pull/10001
[10002]: https://github.com/bevyengine/bevy/pull/10002
[10003]: https://github.com/bevyengine/bevy/pull/10003
[10008]: https://github.com/bevyengine/bevy/pull/10008
[10010]: https://github.com/bevyengine/bevy/pull/10010
[10012]: https://github.com/bevyengine/bevy/pull/10012
[10020]: https://github.com/bevyengine/bevy/pull/10020
[10022]: https://github.com/bevyengine/bevy/pull/10022
[10023]: https://github.com/bevyengine/bevy/pull/10023
[10026]: https://github.com/bevyengine/bevy/pull/10026
[10030]: https://github.com/bevyengine/bevy/pull/10030
[10031]: https://github.com/bevyengine/bevy/pull/10031
[10035]: https://github.com/bevyengine/bevy/pull/10035
[10044]: https://github.com/bevyengine/bevy/pull/10044
[10051]: https://github.com/bevyengine/bevy/pull/10051
[10053]: https://github.com/bevyengine/bevy/pull/10053
[10056]: https://github.com/bevyengine/bevy/pull/10056
[10058]: https://github.com/bevyengine/bevy/pull/10058
[10065]: https://github.com/bevyengine/bevy/pull/10065
[10068]: https://github.com/bevyengine/bevy/pull/10068
[10070]: https://github.com/bevyengine/bevy/pull/10070
[10074]: https://github.com/bevyengine/bevy/pull/10074
[10078]: https://github.com/bevyengine/bevy/pull/10078
[10079]: https://github.com/bevyengine/bevy/pull/10079
[10080]: https://github.com/bevyengine/bevy/pull/10080
[10090]: https://github.com/bevyengine/bevy/pull/10090
[10092]: https://github.com/bevyengine/bevy/pull/10092
[10100]: https://github.com/bevyengine/bevy/pull/10100
[10103]: https://github.com/bevyengine/bevy/pull/10103
[10105]: https://github.com/bevyengine/bevy/pull/10105
[10109]: https://github.com/bevyengine/bevy/pull/10109
[10111]: https://github.com/bevyengine/bevy/pull/10111
[10117]: https://github.com/bevyengine/bevy/pull/10117
[10119]: https://github.com/bevyengine/bevy/pull/10119
[10121]: https://github.com/bevyengine/bevy/pull/10121
[10123]: https://github.com/bevyengine/bevy/pull/10123
[10124]: https://github.com/bevyengine/bevy/pull/10124
[10128]: https://github.com/bevyengine/bevy/pull/10128
[10132]: https://github.com/bevyengine/bevy/pull/10132
[10136]: https://github.com/bevyengine/bevy/pull/10136
[10142]: https://github.com/bevyengine/bevy/pull/10142
[10143]: https://github.com/bevyengine/bevy/pull/10143
[10144]: https://github.com/bevyengine/bevy/pull/10144
[10145]: https://github.com/bevyengine/bevy/pull/10145
[10148]: https://github.com/bevyengine/bevy/pull/10148
[10150]: https://github.com/bevyengine/bevy/pull/10150
[10151]: https://github.com/bevyengine/bevy/pull/10151
[10152]: https://github.com/bevyengine/bevy/pull/10152
[10156]: https://github.com/bevyengine/bevy/pull/10156
[10158]: https://github.com/bevyengine/bevy/pull/10158
[10160]: https://github.com/bevyengine/bevy/pull/10160
[10161]: https://github.com/bevyengine/bevy/pull/10161
[10163]: https://github.com/bevyengine/bevy/pull/10163
[10168]: https://github.com/bevyengine/bevy/pull/10168
[10171]: https://github.com/bevyengine/bevy/pull/10171
[10173]: https://github.com/bevyengine/bevy/pull/10173
[10174]: https://github.com/bevyengine/bevy/pull/10174
[10178]: https://github.com/bevyengine/bevy/pull/10178
[10179]: https://github.com/bevyengine/bevy/pull/10179
[10180]: https://github.com/bevyengine/bevy/pull/10180
[10182]: https://github.com/bevyengine/bevy/pull/10182
[10184]: https://github.com/bevyengine/bevy/pull/10184
[10185]: https://github.com/bevyengine/bevy/pull/10185
[10186]: https://github.com/bevyengine/bevy/pull/10186
[10189]: https://github.com/bevyengine/bevy/pull/10189
[10191]: https://github.com/bevyengine/bevy/pull/10191
[10194]: https://github.com/bevyengine/bevy/pull/10194
[10195]: https://github.com/bevyengine/bevy/pull/10195
[10196]: https://github.com/bevyengine/bevy/pull/10196
[10197]: https://github.com/bevyengine/bevy/pull/10197
[10198]: https://github.com/bevyengine/bevy/pull/10198
[10200]: https://github.com/bevyengine/bevy/pull/10200
[10202]: https://github.com/bevyengine/bevy/pull/10202
[10204]: https://github.com/bevyengine/bevy/pull/10204
[10206]: https://github.com/bevyengine/bevy/pull/10206
[10208]: https://github.com/bevyengine/bevy/pull/10208
[10210]: https://github.com/bevyengine/bevy/pull/10210
[10211]: https://github.com/bevyengine/bevy/pull/10211
[10217]: https://github.com/bevyengine/bevy/pull/10217
[10221]: https://github.com/bevyengine/bevy/pull/10221
[10222]: https://github.com/bevyengine/bevy/pull/10222
[10223]: https://github.com/bevyengine/bevy/pull/10223
[10226]: https://github.com/bevyengine/bevy/pull/10226
[10227]: https://github.com/bevyengine/bevy/pull/10227
[10230]: https://github.com/bevyengine/bevy/pull/10230
[10238]: https://github.com/bevyengine/bevy/pull/10238
[10240]: https://github.com/bevyengine/bevy/pull/10240
[10249]: https://github.com/bevyengine/bevy/pull/10249
[10251]: https://github.com/bevyengine/bevy/pull/10251
[10252]: https://github.com/bevyengine/bevy/pull/10252
[10253]: https://github.com/bevyengine/bevy/pull/10253
[10254]: https://github.com/bevyengine/bevy/pull/10254
[10259]: https://github.com/bevyengine/bevy/pull/10259
[10262]: https://github.com/bevyengine/bevy/pull/10262
[10264]: https://github.com/bevyengine/bevy/pull/10264
[10270]: https://github.com/bevyengine/bevy/pull/10270
[10271]: https://github.com/bevyengine/bevy/pull/10271
[10275]: https://github.com/bevyengine/bevy/pull/10275
[10276]: https://github.com/bevyengine/bevy/pull/10276
[10279]: https://github.com/bevyengine/bevy/pull/10279
[10286]: https://github.com/bevyengine/bevy/pull/10286
[10289]: https://github.com/bevyengine/bevy/pull/10289
[10294]: https://github.com/bevyengine/bevy/pull/10294
[10295]: https://github.com/bevyengine/bevy/pull/10295
[10297]: https://github.com/bevyengine/bevy/pull/10297
[10299]: https://github.com/bevyengine/bevy/pull/10299
[10303]: https://github.com/bevyengine/bevy/pull/10303
[10304]: https://github.com/bevyengine/bevy/pull/10304
[10306]: https://github.com/bevyengine/bevy/pull/10306
[10310]: https://github.com/bevyengine/bevy/pull/10310
[10311]: https://github.com/bevyengine/bevy/pull/10311
[10312]: https://github.com/bevyengine/bevy/pull/10312
[10326]: https://github.com/bevyengine/bevy/pull/10326
[10330]: https://github.com/bevyengine/bevy/pull/10330
[10339]: https://github.com/bevyengine/bevy/pull/10339
[10341]: https://github.com/bevyengine/bevy/pull/10341
[10345]: https://github.com/bevyengine/bevy/pull/10345
[10346]: https://github.com/bevyengine/bevy/pull/10346
[10352]: https://github.com/bevyengine/bevy/pull/10352
[10356]: https://github.com/bevyengine/bevy/pull/10356
[10358]: https://github.com/bevyengine/bevy/pull/10358
[10360]: https://github.com/bevyengine/bevy/pull/10360
