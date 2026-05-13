+++
title = "Loading assets in advance"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Assets are automatically removed whenever all their handles have been dropped. This is done by
tracking every reference created for a specific asset. Creating or cloning a handle increments the
asset's reference count, while dropping a handle decrements the asset's reference count. When the
reference count hits zero, the asset is removed (aka unloaded).

While this strategy is straightforward, it can cause issues if handles are dropped before they are
finished being used. Imagine the following part of a game:

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

The `enemy_spawner` system spawns an enemy whenever the spawner's timer finishes. If the timer is
[`TimerMode::Repeating`], this would spawn an enemy at a regular interval.

Now imagine that the player goes and kills all the enemies, despawning them. Since all the enemies
have been despawned, their [`Sprite`] components will be dropped, and therefore the `"enemy.png"`
handles will be dropped. Since there are no more handles to this asset, the image corresponding to
`"enemy.png"` will be removed due to being unused. When the spawner then tries to spawn the enemy
**it will trigger a reload of the asset**. In this case, the enemy may temporarily become invisible
while the asset is reloading. The rendering system will "gracefully" avoid rendering missing assets.
However, less flexible systems (e.g., third-party plugins) may not handle missing data as
gracefully: some systems may even **panic** if the asset data isn't available. In addition, you have
to pay the cost of loading the asset from disk again.

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

In the `load_enemy_assets_in_startup` system, we do the initial [`AssetServer::load`], and insert
that handle into a resource. Our `enemy_spawner` system is basically the same, except now we just
use the `EnemyAssets` resource, instead of loading directly.

How does this handle the previous case where the player kills all the enemies? Well, since the
handle is still stored in `EnemyAssets`, the enemy sprite doesn't unload! The next time we spawn, it
clones the handle again, and the asset data is immediately ready.

The disadvantage of course is that while there are no enemies, that sprite remains loaded. This can
use up memory if the sprite turns out to not be needed at all (for example, there are no enemy
spawners). Despite this, **our standard recommendation is to do this preloading**. This makes
gameplay logic simpler and more reliable.

## Waiting for Asset Loading

Loading assets is not instantaneous, and it's also not blocking. This means that systems will not
pause and wait for the assets to load. In our `load_enemy_assets_in_startup` system, we called
[`AssetServer::load`] and stored the asset handle in a resource, but the asset will not be loaded
yet. So despite the term "preloading", the first few frames of our game may still have missing
assets - which is not ideal!

To address this, we need to wait for our assets to load **before** starting our game. In essence, we
need a loading screen - or at least a way to know when our assets are loaded.

{% callout(type="info") %}

There are all sorts of ways to implement a loading screen (even some that may be controversial to
call loading screens, like the infamous "crawl through a narrow gap" animation). These are generally
game specific, so we will be ignoring all that and focusing on how to actually tell when the assets
have been loaded.

{% end %}

The fundamental tools for this are [`AssetServer::is_loaded`],
[`AssetServer::is_loaded_with_dependencies`], and
[`AssetServer::is_loaded_with_direct_dependencies`]. These allow you to query the current load state
of a handle. In particular, [`AssetServer::is_loaded`] checks if the asset itself is loaded.
[`AssetServer::is_loaded_with_direct_dependencies`] checks if the asset is loaded and its direct
dependencies are loaded. [`AssetServer::is_loaded_with_dependencies`] checks if the asset is loaded
and **all** its dependencies are loaded, including recursive ones. To use this in our previous
example, we could implement an `is_loaded` method for our `EnemyAssets` struct:

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
loaded. The most straight-forward approach is to use states.

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

Writing our `EnemyAssets::is_loaded` is cumbersome and error-prone. For example, if we add a
`weapon: Handle<Image>` to `EnemyAssets`, we may forget to update our `EnemyAssets::is_loaded`
function. Thankfully, we have tools to entirely replace our `EnemyAssets::is_loaded`. In
particular, [`VisitAssetDependencies`] is a trait (and derive macro) that allows defining types that
report all referenced assets. For example, we can do this for our `EnemyAssets`:

```rust
#[derive(Resource, VisitAssetDependencies)]
struct EnemyAssets {
    #[dependency]
    sprite: Handle<Image>,
    #[dependency]
    weapon: Handle<Image>,
}
```

Now we can delete `EnemyAssets::is_loaded` and instead just call
`asset_server.are_dependencies_loaded(&enemy_assets)`. As long as we annotate all handles (and
fields containing handles) with `#[dependency]`, this function will immediately tell us whether all
our assets are loaded or not.

{% callout(type="warning") %}

The `#[dependency]` attribute supports any type that implements [`VisitAssetDependencies`], whether
it's a single handle, or a whole struct. This can be used to create complex structures. For example:

```rust
#[derive(Resource, VisitAssetDependencies)]
struct EnemyAssets {
    #[dependency]
    sprite: Handle<Image>,
    #[dependency]
    weapon: Handle<Image>,
    #[dependency]
    sounds: CharacterSounds,
}

#[derive(VisitAssetDependencies)]
struct CharacterSounds {
    #[dependency]
    state_to_sound: HashMap<String, Handle<AudioSource>>,
}
```

{% end %}

[`TimerMode::Repeating`]: https://docs.rs/bevy/latest/bevy/prelude/enum.TimerMode.html#variant.Repeating
[`Sprite`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Sprite.html
[`AssetServer::load`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.load
[`AssetServer::is_loaded`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded
[`AssetServer::is_loaded_with_dependencies`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded_with_dependencies
[`AssetServer::is_loaded_with_direct_dependencies`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded_with_direct_dependencies
[`VisitAssetDependencies`]: https://dev-docs.bevy.org/bevy/asset/trait.VisitAssetDependencies.html
