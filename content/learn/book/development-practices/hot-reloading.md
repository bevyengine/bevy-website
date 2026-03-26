+++
title = "Hot Reloading Assets"
insert_anchor_links = "right"
[extra]
weight = 2
+++

It's common while running your game to notice some mistake, or to want to try out some small adjustment.
Unfortunately, compiling can be slow, and requires you to close the game, losing any game state.

To make this easier, Bevy supports **hot reloading**, which allows you to modify your game's assets and automatically load these changes into a running instance of the game.

While hot-reloading is useful during development time, it's _usually_ not something you want to ship in a production game.
Unless you're deliberately using this for modding support, you should turn these settings off when [releasing projects](../releasing-projects/_index.md).

## Hot Reloading Assets

The asset system is always able to handle changes to asset files. Asset sources can define an [`AssetWatcher`] which receives a channel, that it must send any detected changes to.


For the default asset source, this watcher just needs to be enabled, which can be done by enabling the `file_watcher` feature flag:

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