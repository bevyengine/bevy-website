+++
title = "Hot Reloading Code and Assets"
insert_anchor_links = "right"
[extra]
weight = 2
+++

It's common while running your game to notice some mistake, or to want to try out some small adjustment. Unfortunately, compiling your game requires you to close the game, losing your state (where you are in the world, what items you have, etc). What would be great is being able to make changes and have them apply to your running game.

Enter hot-reloading! This is exactly making changes to your game's code or your game's assets and having the game automatically load these changes into the running instance.

While hot-reloading is useful during development time, it's _usually_ not something you want to ship in a production game. Disabling these options when publishing is recommended!

## Hot Reloading Code

<!-- TBW -->

## Hot Reloading Assets

The asset system is always able to handle changes to asset files. Asset sources can define an `AssetWatcher` which receives a channel, that it must send any detected changes to.

For the default asset source, this watcher just needs to be enabled, which can be done by enabling the `file_watcher` feature flag:

```sh
cargo run --features bevy/file_watcher
```

When enabled, assets loaded in your game will automatically reload when their asset file changes - specifically the `Assets<T>` resource will be updated to include the new asset data.

{% callout(type="note") %}
If you are also using embedded assets (through the `load_embedded_asset!` macro), it can be useful to also enable the `embedded_watcher` feature.
{% end %}
