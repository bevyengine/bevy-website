<!-- Add AsyncSeek trait to Reader to be able to seek inside asset loaders -->
<!-- https://github.com/bevyengine/bevy/pull/12547 -->

Assets can be huge, and you don't always need all of the data contained in a single file.

Bevy allows you to add your [own asset loaders].
Starting in Bevy 0.14,  you can now seek to an offset of your choice, reading partway through the file.

Perhaps you have the `.celestial` file format which encodes the universe, but you want to only look at lil' asteroids which always appear at some offset:

```rust
#[derive(Default)]
struct UniverseLoader;

#[derive(Asset, TypePath, Debug)]
struct JustALilAsteroid([u8; 128]); // Each lil' asteroid uses this much data

impl AssetLoader for UniverseLoader {
    type Asset = JustALilAsteroid;
    type Settings = ();
    type Error = std::io::Error;
    async fn load<'a>(
        &'a self,
        reader: &'a mut Reader<'_>,
        _settings: &'a (),
        _load_context: &'a mut LoadContext<'_>,
    ) -> Result<JustALilAsteroid, Self::Error> {
        // The universe is big, and our lil' asteroids don't appear until this offset
        // in the celestial file format!
        let offset_of_lil_asteroids = 5_000_000_000_000;

        // Skip vast parts of the universe with the new async seek trait!
        reader
            .seek(SeekFrom::Start(offset_of_lil_asteroids))
            .await?;

        let mut asteroid_buf = [0; 128];
        reader.read_exact(&mut asteroid_buf).await?;

        Ok(JustALilAsteroid(asteroid_buf))
    }

    fn extensions(&self) -> &[&str] {
        &["celestial"]
    }
}
```

This works because Bevy's [`reader`] type passed into the asset loader's `load` function now implements [`AsyncSeek`].

Real world use cases might for example be:

- You have packed several assets in an archive and you wish to skip to an asset within and read that
- You are dealing with big datasets such as map data and you know where to extract some locations of interest

[`own asset loaders`]: https://github.com/bevyengine/bevy/blob/release-0.14.0/examples/asset/processing/asset_processing.rs 
[`reader`]: http://dev-docs.bevyengine.org/bevy/asset/io/type.Reader.html
[`AsyncSeek`]: https://docs.rs/futures-io/latest/futures_io/trait.AsyncSeek.html
