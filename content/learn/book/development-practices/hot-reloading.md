+++
title = "Hot Reloading Assets"
insert_anchor_links = "right"
[extra]
weight = 2
+++

While running your game, it's common to notice some mistake or want to try out some small adjustment.
Unfortunately, recompiling can be slow, and requires you to close the game, losing any game state.

To make this easier, Bevy supports **hot reloading**, which allows you to modify your game's assets and automatically load these changes into a running instance of the game.

While hot-reloading is useful during development time, it's usually not something you want to ship in a production game.
Unless you're deliberately using this for modding support, you should turn these settings off when [releasing projects](../releasing-projects/_index.md).

## Hot Reloading Assets

The asset system is always able to handle changes to asset files. Asset sources can define an [`AssetWatcher`] which receives a channel to which it sends any detected changes to.

For the default asset source, you just need to enable this watcher by using the `file_watcher` feature flag:

```sh
cargo run --features bevy/file_watcher
```

When enabled, assets loaded in your game will automatically reload when their asset file changes - specifically the `Assets<T>` resource will be updated to include the new asset data.

{% callout(type="note") %}
If you are also using embedded assets (through the [`load_embedded_asset!`] macro), it can be useful to also enable the `embedded_watcher` feature.
{% end %}

[`AssetWatcher`]: https://docs.rs/bevy/latest/bevy/asset/struct.AssetWatcher.html
[`Assets<T>`]: https://docs.rs/bevy/latest/bevy/asset/struct.Assets.html
[`load_embedded_asset!` macro]: https://docs.rs/bevy/latest/bevy_asset_macro/macro.load_embedded_asset.html

## Asset-driven gameplay logic

Asset hot-reloading is so useful and powerful that some projects choose to lean into it,
and deliberately architect their games to drive important gameplay parameters via assets to ease development and modding.

While assets are commonly thought of as being the "art" assets of a game (meshes, images, sounds...),
there's nothing that fundamentally links the pattern or infrastructure to those assets.
If you define our gameplay data in terms of human-readable structured text files (like `.ron` or `.json`),
you can hot reload those too:

```ron
(
    items: {
        (
            value: 814380520,
        ): (
            name: "sword",
            description: "A sharp sword",
            value: 10,
            weight: 2.0,
            max_stack: 1,
        ),
        (
            value: 113295724,
        ): (
            name: "shield",
            description: "A sturdy shield",
            value: 5,
            weight: 5.0,
            max_stack: 1,
        ),
    },
)
```

Each of these objects corresponds to a Rust struct,
which you can serialize (write to disk) and deserialize (load from disk)
with the help of the [`serde`] crate:

```rust
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    description: String,
    value: i32,
    weight: f32,
    max_stack: u8,
}
```

You can read these files to populate a "manifest" of objects, storing it in a resource:

```rust
use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// A strongly typed identifier for an object that can be stored in a manifest
#[derive(Component, Reflect, Serialize, Deserialize)]
pub struct Id<T> {
    value: u64,
    #[reflect(ignore)]
    #[serde(skip)]
    _phantom: core::marker::PhantomData<T>,
}


/// Contains the canonical data for all the items in the game.
#[derive(Debug, Resource, Asset, TypePath, Serialize, Deserialize, PartialEq)]
struct ItemManifest {
    items: HashMap<Id<Item>, Item>,
}
```

These IDs are stored in your components and then looked up by systems when their values need to be referenced.
Because our manifests are asset files, you can take advantage of hot-reloading to adjust or rebalance on the fly by simply editing the corresponding file directly.

As your data becomes more complex, your approach should become more sophisticated: break the process into multiple steps to resolve cross-object references, add error handling, and so on.

While this workflow can be powerful and convenient, this pattern is not right for every project.
There's non-trivial setup work, significant indirection, and you cannot capture arbitrarily complex gameplay logic in data.
This pattern is best suited to games that have a large amount of structured gameplay data that needs tuning:
it would work well for something like an ARPG, but poorly for a walking simulator.

{% callout(type="note") %}

Within the games industry, games that use this pattern are sometimes called "data-driven". 
This is not to be confused with data-driven in the sense of using data to make decisions, or data-oriented, where your game maps well to the underlying hardware of the machine to run faster.
As a result, Bevy uses the less confusing term "asset-driven" when discussing this pattern.

{% end %}

[`serde`]: https://docs.rs/serde/latest/serde/
