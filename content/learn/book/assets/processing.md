+++
title = "Asset Processing"
insert_anchor_links = "right"
[extra]
weight = 4
+++

So far, we've conceptualized assets as just "stuff on disk you want to put in memory". In practice
though there are two "states" of assets: the "raw" files you acquire during development (from
the internet, from your artists, etc.), and the "processed", game-ready assets. For some assets,
these are the same thing - you just ship whatever asset files you get. However, some assets need to
be converted into a form that is more appropriate for use in a game.

As an example, using an ultrahigh resolution texture in your game is (probably) not what you want.
This increases download sizes, slows down loading speeds, and likely reduces rendering performance.
This would be your "raw" file. Usually, these high-quality versions are compressed, downscaled, etc.,
before being used in a game. This final form is our game-ready version.

We can enable **asset processing** to perform this conversion. When enabled, assets in your `assets`
folder are automatically processed by the registered processors to produce the game-ready versions
of assets. Your game will automatically use the processed assets (without needing to change anything
else).

{% callout(type="important") %}

While it is possible to enable asset processing just before publishing to do steps like compression,
we **strongly** recommend users choose at the beginning of their project whether to use processing
or not. In general, processing is **not** guaranteed to be transparent. For example, the "game-ready
version" of a `.gltf` file could actually be a `Scene` asset, which is not the same as the `Gltf`
asset - these have entirely different structures! There is no guarantee (intentionally) that the
processed asset has the same type as the original asset. Another way to think of asset processing is
as an **import process**: importing a file could import it as another type entirely.

{% end %}

The first step to enable asset processing is to set the `AssetPlugin::mode` accordingly:

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(AssetPlugin {
            mode: AssetMode::Processed,
            ..Default::default()
        }))
        .run();
}
```

This tells Bevy to use the processed versions of assets. To process assets during development, set
the `asset_processor` feature on `bevy`/`bevy_asset` (this should be **unset** when publishing your
game).

Now all that's left to do is to add processors to start processing assets. For example, enabling the
`compressed_image_saver` feature will automatically add an asset processor for `.png` files to
compress and write them (internally) as the `.basisu` file format.

{% callout(type="warning") %}

By default, processed assets are written to `imported_assets/` (as opposed to the "regular" assets
directory of `assets/`). **Do not** check in the processed assets as to your version control. The
`imported_assets/` directory _should be totally ephemeral_. You should be able to delete it and Bevy
will automatically reprocess the assets. In other words, the "source of truth" for your assets
during development are those in the `assets/` directory.

In contrast, when publishing, you should be publishing your `imported_assets/` directory, not the
`assets/` directory. `imported_assets/` contains the game-ready assets which is what your published
game should use!

{% end %}

## Writing Your Own Asset Processors

Bevy provides some common processors, but there are plenty of game-specific (or even asset-specific)
processors that may be needed by users. If you need a processor that Bevy doesn't provide, you can
make one!

The [`Process`] trait is the lowest-level trait for asset processing. The [`Process`]
implementation is given a `&mut dyn Reader` and must write the processed asset to a `&mut dyn Writer`.
This is the most flexible (if cumbersome) version of this interface. Implementing [`Process`]
directly can allow you to optimize for all sorts of use cases (for example, processing small chunks
of the reader to avoid needing to load the entire file into memory).

In practice though, most users just need the simplest sort of processing: load the file, change the
asset in some way (even maybe changing its type), then save it back out. For this, we have the
[`LoadTransformAndSave`] type. You specify the [`AssetLoader`] to load with, the [`AssetTransformer`]
to apply, and the [`AssetSaver`] to save with, and you will have a new [`Process`] implementation.

For now, we will assume we have these three: `MyLoader`, `MyTransformer`, and `MySaver` (we'll
describe these more after). To register the processor, use `register_asset_processor`:

```rust
// This type alias is **not** required. However it makes using the processor a little easier!
type MyProcessor = LoadTransformAndSave<MyLoader, MyTransformer, MySaver>;
// It's up to you to create the transformer and saver. In this case, we assume they both have a
// `new` function. The loader must be separately registered (see below).
app.register_asset_processor(MyProcessor::new(MyTransformer::new(), MySaver::new()));
```

At this point, the processor can be used through meta files (see more below). However, most
processors are just meant to be applied to all assets with a certain extension. This can be done
quite easily:

```rust
// That type alias came in handy! We can reuse it here to set which extension it should process.
app.set_default_processor::<MyProcessor>("my_file_extension");
// We can also register more extensions to process.
app.set_default_processor::<MyProcessor>("smolext");
```

Now any asset like `blah.my_file_extension` or `cute.smolext` will automatically be processed by our
processor.

### AssetLoader

This is exactly the same asset loader as discussed in [Custom Assets](/learn/book/assets/custom-assets). Note that
`LoadTransformAndSave` can only use your asset loader if it is registered.

### AssetTransformer

This trait takes an [`AssetInput`] type and converts it into an [`AssetOutput`] type. How you do
this conversion is totally up to you! In the example below, we just call the `NewAssetType::new`
method with the original asset value:

```rust
#[derive(TypePath)]
struct MyTransformer;

impl AssetTransformer for MyTransformer {
    type AssetInput = MyAssetType;
    type AssetOutput = NewAssetType;
    type Settings = ();
    type Error = BevyError;

    async fn transform<'a>(
        &'a self,
        mut asset: TransformedAsset<Self::AssetInput>,
        _settings: &Self::Settings,
    ) -> Result<TransformedAsset<Self::AssetOutput>, Self::Error> {
        // Do something to produce the output type. Here we imagine our asset type has a `new`
        // function that takes `&MyAssetType`.
        let new_asset = NewAssetType::new(asset.get());

        // Using `replace_asset` keeps all the "subassets" from the initial asset load. In practice,
        // this means all the handles in the original `asset` will remain valid.
        Ok(asset.replace_asset(new_asset))
    }
}
```

Of course, [`AssetInput`] must match the [`AssetLoader::Asset`] type from our loader.

### AssetSaver

This trait takes a [`TransformedAsset`] and writes it as bytes to the provided [`Writer`]. How it
writes these bytes is up to you! This depends heavily on the file format. Most often this is done
using the [`serde`] crate to generate serialization and deserialization implementations, followed by
a format crate like [`ron`].

In addition, the [`AssetSaver`] must specify what loader the output is meant to be loaded as. This
ensures that processed assets are loaded with a loader that can actually read the format that was
written.

```rust
#[derive(TypePath)]
struct MySaver;

impl AssetSaver for MySaver {
    type Asset = NewAssetType;
    type Settings = ();
    type Error = BevyError;
    type OutputLoader = NewAssetLoader;

    async fn save(
        &self,
        writer: &mut Writer,
        asset: SavedAsset<'_, '_, Self::Asset>,
        _settings; &Self::Settings,
        _asset_path: AssetPath<'_>,
    ) -> Result<NewAssetLoader::Settings, BevyError> {
        // Note: this is a simplified example where we assume we have no "subassets". If we did,
        // we'd likely need to "encode" those subassets in some way in our output data.
        let ron_string = ron::to_string(asset.get())?;
        writer.write_all(&ron_string).await?;
        Ok(NewAssetLoader::Settings::default())
    }
}
```

Just as with [`AssetLoader`]s, your type needs to be "encoded" somehow, whether through [`serde`] or
whatever else. It may be necessary to create a "serializable" version of your asset. This is
described in more detail in [Custom Assets](/learn/book/assets/custom-assets).

## Meta Files

Throughout the description of our [`LoadTransformAndSave`] implementation, we've skipped over the
various `Settings` associated types. These allow your processor to expose settings that can change
how an individual asset is processed. With [`LoadTransformAndSave`], you can change the settings of
each individual stage (as in, load, transform, and save). But how do you configure these settings
for an asset?

Meta files! Meta files allow you to define whether an asset should be processed/loaded, with what
processor/loader, and the settings for that processor/loader. The default meta file for an asset can
be created using [`AssetProcessor::write_default_meta_file_for_path`]. For an asset at
`path/to/my/asset.ext`, the meta file is written to `path/to/my/asset.ext.meta`. Below is an example
of such a meta file:

```ron
(
    meta_format_version: "1.0",
    asset: Process(
        processor: "LoadTransformAndSave<MyLoader, MyTransformer, MySaver>",
        settings: (
            loader_settings: (),
            transformer_settings: (),
            saver_settings: (),
        ),
    ),
)
```

This meta file will process its asset with the [`LoadTransformAndSave`] processor we created in the
previous section. It will also do so with the given settings for all three of our stages. In our
example, the settings were just `()`, so we don't have anything interesting to configure here. But
we could!

As mentioned earlier, we can use **any** processor that has been registered, not just ones that have
been registered as a default processor for some file extension. This allows you to create processors
that target individual assets, rather than all assets of a particular extension.

### "Load" Meta Files

Meta files can also be used to configure settings of [`AssetLoader`]s during loading rather than
processing. This also adds a way to "opt out" of processing a particular asset, by specifying that
it should be loaded instead. Below is such a meta file.

```ron
(
    meta_format_version: "1.0",
    asset: Load(
        loader: "MyLoader",
        settings: (),
    ),
)
```

This ensures that the asset this meta file is associated with will be loaded using `MyLoader`
instead of processed with our processor. We can also specify the settings (though just like our
processor, its settings are boring since they are just `()`).

"Load" meta files can be used **even without asset processing**. They are very powerful when
combined with configurable [`AssetLoader`]s.

{% callout(type="info") %}

A common use case for these "load" meta files is to set the [`RenderAssetUsages`] for an asset.
Setting render assets like meshes or textures to use [`RenderAssetUsages::RENDER_WORLD`] only allows
mesh and texture data to only exist in the GPU, freeing RAM on the CPU to do whatever else. **This
can be a very worthwhile optimization**. Of course this comes with caveats: since the data is no
longer present on the CPU, it can't be used by regular systems. Most meshes and textures aren't used
in this way anyway though.

{% end %}

[`Process`]: https://docs.rs/bevy/latest/bevy/asset/processor/trait.Process.html
[`LoadTransformAndSave`]: https://docs.rs/bevy/latest/bevy/asset/processor/struct.LoadTransformAndSave.html
[`AssetLoader`]: https://docs.rs/bevy/latest/bevy/asset/trait.AssetLoader.html
[`AssetTransformer`]: https://docs.rs/bevy/latest/bevy/asset/transformer/trait.AssetTransformer.html
[`AssetSaver`]: https://docs.rs/bevy/latest/bevy/asset/saver/trait.AssetSaver.html
[`AssetInput`]: https://docs.rs/bevy/latest/bevy/asset/transformer/trait.AssetTransformer.html#associatedtype.AssetInput
[`AssetOutput`]: https://docs.rs/bevy/latest/bevy/asset/transformer/trait.AssetTransformer.html#associatedtype.AssetOutput
[`AssetLoader::Asset`]: https://docs.rs/bevy/latest/bevy/asset/trait.AssetLoader.html#associatedtype.Asset
[`TransformedAsset`]: https://docs.rs/bevy/latest/bevy/asset/transformer/struct.TransformedAsset.html
[`Writer`]: https://docs.rs/bevy/latest/bevy/asset/io/type.Writer.html
[`serde`]: https://docs.rs/serde/latest/serde/
[`ron`]: https://docs.rs/ron/latest/ron/
[`AssetProcessor::write_default_meta_file_for_path`]: https://docs.rs/bevy/latest/bevy/asset/processor/struct.AssetProcessor.html#method.write_default_meta_file_for_path
[`RenderAssetUsages`]: https://docs.rs/bevy/latest/bevy/asset/struct.RenderAssetUsages.html
[`RenderAssetUsages::RENDER_WORLD`]: https://docs.rs/bevy/latest/bevy/asset/struct.RenderAssetUsages.html#associatedconstant.RENDER_WORLD
