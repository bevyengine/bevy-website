+++
title = "Loading Assets In Advance"
insert_anchor_links = "right"
[extra]
weight = 2
+++

Assets are automatically removed whenever all their handles have been dropped.
This is done by tracking every reference created for a specific asset.
Creating or cloning a handle increments the asset's reference count, while dropping a handle decrements the asset's reference count.
When the reference count hits zero, the asset is removed (a.k.a. unloaded).
While this process is straightforward, it can cause issues if handles are dropped before they are finished being used.

To see this, imagine we have an `enemy_spawner` system that will spawn an enemy whenever a timer finishes.
The timer is set to [`TimerMode::Repeating`], which means an enemy will spawn at a regular interval.
This could look something like this:

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
        // this frame. We are assuming that the Timer uses `TimerMode::Repeating`.
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

Now imagine that the player goes and kills all the enemies, despawning them.
Since all the enemies have been despawned, their [`Sprite`] components will be dropped, and therefore the `"enemy.png"` handles will be dropped.
With no existing `"enemy.png"` handles, the image corresponding to `"enemy.png"` will be removed due to being unused.

The next time the spawner tries to spawn a new enemy **it will trigger a reload of the asset**.
In this case, the enemy may temporarily become invisible while the asset is reloading since the rendering system will "gracefully" avoid rendering missing assets.
However, less flexible systems (e.g., third-party plugins) may not handle missing data as gracefully: some systems may even **panic** if the asset data isn't available.
In addition, you have to pay the cost of loading the asset from disk again.

Now, this is not necessarily wrong.
Your game may be able to tolerate this, and being able to dynamically load and unload data as it's needed can reduce your memory footprint!

However, for many use cases having asset data take a few frames to load can degrade the experience.
In the case of our enemy spawner, we might want to play a specific animation when the enemy is spawned.
If we need to load the data _after_ the enemy is spawned, we may miss part of the animation, or the animation will be delayed while the enemy is still loading.
In either case this introduces differing gameplay behavior for when the asset is loaded or when it isn't.

## Preloading

Preloading is the strategy of loading data that you expect to need ahead of time, and then caching those newly created handles.
Since the handle is being cached, the data won't be unloaded.
We can update our previous example to use preloading so that the enemy spawner can immediately access the image handle when spawning a new enemy:

```rust
#[derive(Resource)]
struct EnemyAssets {
    sprite: Handle<Image>,
}

fn load_enemy_assets_in_startup(
    asset_server: Res<AssetServer>,
    mut commands: AssetCommands
) {
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

Now we're calling the initial [`AssetServer::load`] and storing that handle in a resource in the `load_enemy_assets_in_startup` system.
Our `enemy_spawner` system essentially stays the same, except now we're using the handle stored in the `EnemyAssets` resource instead of calling the `load` directly.

How is the previous case where the player kills all the enemies affected?
The enemy sprite won't be unloaded since the handle is still stored in `EnemyAssets`.
The next time an enemy is spawned, it will simply clone the handle again from `EnemyAssets` and immediately access the asset data!

The disadvantage of course is that the sprite image will always be loaded.
This will take up memory even if the sprite image isn't being used (for example, if there are no enemy spawners).
Despite this, **our general recommendation is to use asset preloading**.
This makes gameplay logic simpler and more reliable.

## Waiting for Asset Loading

Despite the term "preloading", your game will not wait for assets to finish loading.
This means that systems will not pause and wait for assets to load.
Its entirely possible that the first few frames of your game may still have missing assets.
Even though we called [`AssetServer::load`] and stored the asset handle in a resource in our previous example, the asset is not guaranteed to be loaded.

To address this, we need to wait for our assets to load **before** starting our game.
In essence, we need a loading screen - or at least a way to know when our assets are loaded.

{% callout(type="info") %}

There are all sorts of ways to implement a loading screen (even some that may be controversial to call loading screens, like the infamous "crawl through a narrow gap" animation).
These are generally game specific, and it would be impossible to cover every possible situation.
Instead we'll focus on how to tell when the assets have been loaded instead.

{% end %}

The fundamental tools for this are [`AssetServer::is_loaded`], [`AssetServer::is_loaded_with_dependencies`], and [`AssetServer::is_loaded_with_direct_dependencies`].
These allow you to query the current load state of a handle.
In particular, [`AssetServer::is_loaded`] checks if the asset itself is loaded.
[`AssetServer::is_loaded_with_direct_dependencies`] checks if the asset is loaded and its direct dependencies are loaded.
[`AssetServer::is_loaded_with_dependencies`] checks if the asset is loaded and **all** its dependencies are loaded, including recursive ones.
To use this in our previous example, we could implement an `is_loaded` method for our `EnemyAssets` struct:

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

Now, all we need is to prevent our `enemy_spawner` system from running until `EnemyAssets` is loaded.
The most straight-forward approach is to use a [state](learn/book/control-flow/states/) to control for if our gameplay systems should run.

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
    mut next_state: ResMut<NextState<AssetsState>>
) {
    if !enemy_assets.is_loaded(&asset_server) {
        return;
    }
    next_state.set(AssetsState::Loaded);
}
```

### Automated Load Checking

Manually writing a `EnemyAssets::is_loaded` method can be cumbersome (and potentially error-prone).
For example, if we add another field to `EnemyAssets`, we may forget to update our `EnemyAssets::is_loaded` function.
Thankfully, Bevy provides some tools that can entirely replace our `EnemyAssets::is_loaded`.
In particular, [`VisitAssetDependencies`] is a trait (and derive macro) that allows defining types that report all referenced assets.
For example, we can do this for our `EnemyAssets`:

```rust
#[derive(Resource, VisitAssetDependencies)]
struct EnemyAssets {
    #[dependency]
    sprite: Handle<Image>,
    #[dependency]
    weapon: Handle<Image>,
}
```

Now we can delete `EnemyAssets::is_loaded` and instead call `asset_server.are_dependencies_loaded(&enemy_assets)`.
As long as we annotate all handles (and fields containing handles) with `#[dependency]`, this function will immediately tell us whether all our assets are loaded or not.

{% callout(type="warning") %}

The `#[dependency]` attribute supports any type that implements [`VisitAssetDependencies`], whether it's a single handle, or a whole struct.
This can be used to create complex structures.
For example:

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

[`VisitAssetDependencies`]: https://docs.rs/bevy/latest/bevy/asset/trait.VisitAssetDependencies.html
{% end %}

[`TimerMode::Repeating`]: https://docs.rs/bevy/latest/bevy/prelude/enum.TimerMode.html#variant.Repeating
[`Sprite`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Sprite.html
[`AssetServer::load`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.load
[`AssetServer::is_loaded`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded
[`AssetServer::is_loaded_with_dependencies`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded_with_dependencies
[`AssetServer::is_loaded_with_direct_dependencies`]: https://docs.rs/bevy/latest/bevy/prelude/struct.AssetServer.html#method.is_loaded_with_direct_dependencies
[`VisitAssetDependencies`]: https://docs.rs/bevy/latest/bevy/asset/trait.VisitAssetDependencies.html
