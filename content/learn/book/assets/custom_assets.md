+++
title = "Custom Assets"
insert_anchor_links = "right"
[extra]
weight = 3
+++

By default, Bevy provides asset types for all sorts of common operations, from [`Mesh`], to [`Image`], to [`AudioSource`].
However, some games may need assets with their own custom type.
As described in previous pages, assets allow you to access data that doesn't have to be compiled into the actual app, allowing for faster iteration loops.
For example, if you're making a deck builder you may want to define each card as an asset, including its stats and any special effects it has.
Since these cards are not compiled into the app, changing stats is as simple as changing the file.

## Defining an Asset Type

Creating an asset type starts with defining the type and deriving the [`Asset`] trait:

```rust
#[derive(Asset)]
struct Card {
    energy: u32,
    health: u32,
    effects: Vec<Effect>,
}
```

If your asset contains any [`Handle`]s or [`UntypedHandle`]s, make sure to annotate them with `#[dependency]`.

```rust
#[derive(Asset)]
struct Card {
    energy: u32,
    health: u32,
    #[dependency]
    art: Handle<Image>,
    effects: Vec<Effect>,
}
```

This allows methods like [`AssetServer::is_loaded_with_dependencies`] to track not only your custom asset, but also any other assets it references.

{% callout(type="info") %}

This tracking is done through the [`VisitAssetDependencies`] trait, which can be derived independently from [`Asset`].
So in the above example, we could have:

```rust
#[derive(Asset)]
struct Card {
    energy: u32,
    health: u32,
    #[dependency]
    art: Handle<Image>,
    #[dependency]
    effects: Vec<Effect>,
}

#[derive(VisitAssetDependencies)]
struct Effect {
    #[dependency]
    particles: Handle<Image>,
}
```

[`Asset`]: https://docs.rs/bevy/latest/bevy/asset/trait.Asset.html
[`VisitAssetDependencies`]: https://docs.rs/bevy/latest/bevy/asset/trait.VisitAssetDependencies.html
{% end %}

Finally, in your `main` function, call `init_asset` with your asset type to register it:

```rust
fn main() {
    App::new(DefaultPlugins)
        .init_asset::<Card>()
        .run()
}
```

Now this type can be used with things like `ResMut<Assets<Card>>`!
These assets can be `add`ed just like any other, and accessed just like any other.

[`Mesh`]: https://docs.rs/bevy/latest/bevy/mesh/struct.Mesh.html
[`Image`]: https://docs.rs/bevy/latest/bevy/image/struct.Image.html
[`AudioSource`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AudioSource.html
[`Asset`]: https://docs.rs/bevy/latest/bevy/asset/trait.Asset.html
[`Handle`]: https://docs.rs/bevy/latest/bevy/asset/enum.Handle.html
[`UntypedHandle`]: https://docs.rs/bevy/latest/bevy/asset/enum.UntypedHandle.html
[`AssetServer::is_loaded_with_dependencies`]: https://docs.rs/bevy/latest/bevy/asset/struct.AssetServer.html#method.is_loaded_with_dependencies


## Creating an Asset Loader

At this point our custom asset type _is_ completely usable, however most of the time you'll want to create a custom asset type in order to load it from files.
For this, we need to define an asset loader: Bevy needs to know how it should read the bytes on disk and turn them into your asset type.

```rust
fn main() {
    App::new(DefaultPlugins)
        .init_asset::<Card>()
        .register_asset_loader(CardLoader)
        .run()
}

struct CardLoader;

impl AssetLoader for CardLoader {
    type Asset = Card;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _settings: &Self::Settings,
        load_context: &mut LoadContext<'_>,
    ) -> Result<Card, Self::Error> {
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        // We'll give our files the extension "card.ron".
        &["card.ron"]
    }
}
```

This gives us a basic framework to work with, but you can expand it further.
Every file format is different, so the specifics will depend on your particular situation.

In our example, we will use the `ron` format to store our data.
Normally, this is done using [`serde`] and the [`ron`] crate - and we will use this strategy!
However, there's a complication: `Card` doesn't implement [`serde::Deserialize`] (and in fact it can't).
[`Handle`] doesn't implement [`serde::Deserialize`] either.

To resolve this blocker, we can create a type that **does** define [`serde::Deserialize`] (along with [`serde::Serialize`]), which we can then convert into our actual asset type.

```rust
#[derive(Serialize, Deserialize)]
struct CardData {
    energy: u32,
    health: u32,
    // We've replaced our `art` handle with a string that we can load to get the actual handle.
    art_path: String,
    effects: Vec<EffectData>,
}

// This replaces our `todo` from before.
let mut buffer = vec![];
reader.read_to_end(&mut buffer).await?;
let card_data: CardData = ron::de::from_bytes(&buffer)?;

Ok(Card {
    energy: card_data.energy,
    health: card_data.health,
    // We perform a "nested load" to get the handle for our art from its path.
    art: load_context.load(card_data.art_path),
    effects: card_data.effects.map(EffectData::to_effect).collect(),
})
```

That's all we need!
Now we can load our `Card`s just like any other asset:

```rust
let exodia_leg: Handle<Card> = asset_server.load("exodia_s_leg.card.ron");
```

[`ron`]: https://crates.io/crates/ron
[`serde`]: https://crates.io/crates/serde
[`serde::Serialize`]: https://docs.rs/serde/1.0.228/serde/trait.Serialize.html
[`serde::Deserialize`]: https://docs.rs/serde/1.0.228/serde/trait.Deserialize.html
