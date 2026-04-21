+++
title = "Asset Lifetimes and Preloading"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Assets are automatically removed whenever all their handles have been dropped (with the exception of
UUID assets). This is done through reference-counting: creating/cloning a handle increments the
asset's reference count, dropping a handle decrements the asset's reference count, and when the
reference count hits zero, the asset is removed (aka unloaded).

While this strategy is straightforward, it can lead to some issues if not considered. Imagine the
following part of a game:

```rust
#[derive(Component)]
struct EnemySpawner(Timer);

fn enemy_spawner(
    time: Res<Time>,
    mut spawner: Query<&mut EnemySpawner>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for mut spawner in spawner.iter_mut() {
        // Tick every spawner. We only want to spawn the enemy if the timer has finished
        // this frame. We are assuming that the Timer uses [`TimerMode::Repeating`].
        if !spawner.tick(time.delta()).just_finished() {
            continue;
        }
        commands.spawn((
            Enemy,
            Sprite::from_image(asset_server.load("enemy.png")),
        ));
    }
}
```

Put into words, this system spawns an enemy whenever the spawner's timer finishes. If the timer is
[`TimerMode::Repeating`], this would spawn an enemy at a regular interval.

Now imagine that the player goes and kills all the enemies, despawning them. Since all the enemies
have been despawned, their [`Sprite`] components will be dropped, and therefore the `"enemy.png"`
handles will be dropped. Since there are no more handles to this asset, the image corresponding to
`"enemy.png"` will be removed due to being unused. When the spawner then tries to spawn the enemy
**it will trigger a reload of the asset**. This can lead to the enemy being invisible for a while
(thanks to the rendering system gracefully handling these missing asset cases). For less flexible
systems (e.g., third-party plugins), missing data like this may not have nice handling (some systems
may **panic** if the asset data isn't available). In addition, you incur the cost of needing to
reload the data from disk.

Now, this is not **necessarily** wrong. Your game may be able to tolerate this, and being able to
dynamically load and unload data as it's needed can reduce your memory footprint!

However, for many use cases, having asset data take a few frames to load can degrade the experience.
In the case of our enemy spawner, maybe we want to play a nice animation when the enemy is spawned.
If we need to load the data _after_ the enemy is spawned, we may miss part of the animation, or the
animation will be delayed while the enemy is still loading. In either case this introduces differing
gameplay behavior for when the asset is loaded or when it isn't.

## Preloading

Preloading is the strategy of loading data that you expect to need ahead of time, and then caching
that handle. Since the handle is being held, we won't unload the data, so the enemy spawner will be
able to spawn the enemy and have its data already available!

Updating our example from above:

```rust
#[derive(Resource)]
struct EnemyAssets {
    sprite: Handle<Image>,
}

fn load_enemy_assets_in_startup(asset_server: Res<AssetServer>, mut commands: AssetCommands) {
    commands.insert_resource(EnemyAssets {
        sprite: asset_server.load("enemy.png"),
    });
}

#[derive(Component)]
struct EnemySpawner(Timer);

fn enemy_spawner(
    time: Res<Time>,
    mut spawner: Query<&mut EnemySpawner>,
    enemy_assets: Res<EnemyAssets>,
    mut commands: Commands,
) {
    for mut spawner in spawner.iter_mut() {
        if !spawner.tick(time.delta()).just_finished() {
            continue;
        }
        commands.spawn((
            Enemy,
            Sprite::from_image(enemy_assets.sprite.clone()),
        ));
    }
}
```

In a startup system, we do the initial [`AssetServer::load`], and insert that handle into a
resource. Our `enemy_spawner` code is basically the same, except now we just use the `EnemyAssets`
resource, instead of loading directly.

How does this handle the previous case where the player kills all the enemies? Well, since the
handle is still stored in `EnemyAssets`, the enemy sprite doesn't unload! The next time we spawn, it
clones the handle again, and the asset data is immediately ready.

The disadvantage of course is that while there are no enemies, that sprite remains loaded. This can
use up memory if the sprite turns out to not be needed at all (for example, there are no enemy
spawners). Despite this, **our standard recommendation is to do this preloading**. This makes
gameplay logic simpler and more reliable.

## Waiting for Asset Loading

Loading assets is not instantaneous, and it's also not blocking. That means in our
`load_enemy_assets_in_startup` system, although we called [`AssetServer::load`], the asset may not
have been loaded yet. So despite the term "preloading", the first few frames of our game may still
have missing assets - which is not ideal!

To address this, we need to wait for our assets to load **before** starting our game. In essence, we
need a loading screen! There are all sorts of ways to implement a loading screen (even some that
may be controversial to call loading screens, like the infamous "crawl through a narrow gap"
animation). We will be ignoring all that and focusing on how to actually tell when the assets have
been loaded.

The fundamental tools for this are [`AssetServer::is_loaded`],
[`AssetServer::is_loaded_with_dependencies`], and
[`AssetServer::is_loaded_with_direct_dependencies`]. These allow you to query the current state of
loads for a handle. For example, in our previous example, we could implement the following:

```rust
impl EnemyAssets {
    fn is_loaded(&self, asset_server: &AssetServer) -> bool {
        // In general, we recommend using `is_loaded_with_dependencies`, since other
        // asset types may have more complex requirements to be usable.
        asset_server.is_loaded_with_dependencies(&self.sprite)
        // If we had more handles here, we could just `&&` them all together.
    }
}
```

Now, all we need is to prevent our `enemy_spawner` system from running until `EnemyAssets` is
loaded. There's a lot of ways to do this, but the most out-of-the-box solution is to use states.

```rust
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_state::<AssetsState>()
        .add_systems(Startup, load_enemy_assets_in_startup)
        .add_systems(Update,
            wait_until_enemy_assets_loaded
                .run_if(in_state(AssetsState::Loading))
        )
        .add_systems(FixedUpdate,
            enemy_spawner
                .run_if(in_state(AssetsState::Loaded))
        )
        .run()
}

#[derive(States, Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
enum AssetsState {
    Loading,
    Loaded,
}

fn wait_until_enemy_assets_loaded(
    enemy_assets: Res<EnemyAssets>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<AssetsState>>) {
    if !enemy_assets.is_loaded(&asset_server) {
        return;
    }
    next_state.set(AssetsState::Loaded);
}
```

### Automated Load Checking

Writing our `EnemyAssets::is_loaded` is cumbersome and error-prone. If you have many handles, you
may forget to check one! Thankfully, we have tools to deal with this. In particular,
[`VisitAssetDependencies`] is a trait (and derive macro) that allows defining types that report all
referenced assets. For example, we can do this for our `EnemyAssets`:

```rust
#[derive(Resource, VisitAssetDependencies)]
struct EnemyAssets {
    #[dependency]
    sprite: Handle<Image>,
}
```

How does this help us? Well now we entirely replace our `enemy_assets.is_loaded()` method, with just
`asset_server.are_dependencies_loaded(&enemy_assets)`. As long as we annotate all handles (and
fields containing handles) with `#[dependency]`, this call will immediately tell us whether all our
assets are loaded or not!

[`TimerMode::Repeating`]: https://docs.rs/bevy/latest/bevy/prelude/enum.TimerMode.html#variant.Repeating
[`Sprite`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Sprite.html
[`AssetServer::load`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.load
[`AssetServer::is_loaded`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded
[`AssetServer::is_loaded_with_dependencies`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded_with_dependencies
[`AssetServer::is_loaded_with_direct_dependencies`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded_with_direct_dependencies
[`VisitAssetDependencies`]: https://dev-docs.bevy.org/bevy/asset/trait.VisitAssetDependencies.html
