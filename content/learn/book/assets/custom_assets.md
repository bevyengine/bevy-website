+++
title = "Custom Assets"
insert_anchor_links = "right"
[extra]
weight = 3
+++

By default, Bevy provides asset types for all sorts of common operations, from `Mesh`, to `Image`,
to `AudioClip`. However, some games may need their own asset types. As described previously, assets
allow data for your game to not be compiled into the actual app, allowing for faster iteration
loops. For example, if you're making a deck builder, you may want to define each card as an asset
including its stats and any special effects it has. Since these cards are not compiled into the app,
changing stats is as simple as changing the file.

## Defining an asset type.

To create an asset type, first define a type, with the `Asset` derive.

```rust
#[derive(Asset)]
struct Card {
    energy: u32,
    health: u32,
    effects: Vec<Effect>,
}
```

If your asset contains any `Handle`s or `UntypedHandle`s, make sure to annotate them with
`#[dependency]`.

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

This allows methods like `AssetServer::is_loaded_with_dependencies` to track not only your asset,
but also any assets it references.

{% callout(type="info") %}

This tracking is done through the `VisitAssetDependencies` trait, which can be derived independently
from `Asset`. So in the above example, we could have:

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

{% end %}

Finally, in your `main` function, call `init_asset` with your asset type to register it:

```rust
fn main() {
    App::new(DefaultPlugins)
        .init_asset::<Card>()
        .run()
}
```

Now this type can be used with things like `ResMut<Assets<Card>>`! These assets can be `add`ed just
like any other, and accessed just like any other.

## Creating an asset loader.

Our asset type **is** totally usable, but most of the time, you want to create a custom asset type
in order to load it from files. For this, we need to define an asset loader: we need to tell Bevy
how to take the bytes on disk, and turn them into your asset type.

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

This gives us the skeleton. Of course, every file format is different, so the specifics will depend
on your particular situation.

In our example, we will use the `ron` format to store our data. Normally, this is done using `serde`
and the `ron` crate - and we will use this strategy! However, there's a complication: `Card` doesn't
implement `serde::Deserialize`, and in fact it can't. `Handle` doesn't implement
`serde::Deserialize` either.

To resolve, we can define a type that **does** define `serde::Deserialize` (and `serde::Serialize`
for that matter) which we can then convert into our actual asset type.

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

That's all! Now we can load our `Card`s just like any other asset:

```rust
let exodia_leg: Handle<Card> = asset_server.load("exodia_s_leg.card.ron");
```
