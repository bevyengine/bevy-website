+++
title = "Bevy's Asset Framework"
insert_anchor_links = "right"
[extra]
weight = 1
status = 'hidden'
+++

**Assets** have two defining qualities that shape Bevy's architecture for working with them:

1. Their values are not compiled into the program. Typically, this means the file system on the user's hard drive.
2. They are often very large: multiple megabytes, when typical data-storing components are measured in bytes.

The first point means that we need tools to dynamically load (and unload) them at runtime.
This is handled by the [`AssetServer`], which handles the surprisingly complex process of turning a path to an asset we want to use into bytes in RAM that we can stick in a Rust struct.

The second point means that we really don't want to be storing multiple copies of the same asset: RAM and VRAM usage is often a critical limitation for game performance.
In Bevy, the canonical version of every asset of a type `A` is stored in a matching resource: [`Assets<A>`].
Components (like [`Mesh3d`] or [`Sprite`]) which want to reference this data store a [`Handle<A>`],
which point to this canonical version.

{% callout(type="info") %}

Loading an asset from the same path does not create a new copy of the data from disk by loading it again;
these calls are deduplicated by [`AssetPath`].

{% end %}

Various systems then read these component-storing handles,
look up the asset they're pointing to, and then use that information to 
do things like "render them" or "determine collisions".

While this explanation serves as an excellent mental model for the core data flow of assets in Bevy,
there are a few subtleties that are worth getting into in this chapter.

But before that: let's load our first assets!

[`AssetServer`]: https://docs.rs/bevy/latest/bevy/asset/struct.AssetServer.html
[`Assets<A>`]: https://docs.rs/bevy/latest/bevy/asset/struct.Assets.html
[`Mesh3d`]: https://docs.rs/bevy/latest/bevy/pbr/struct.Mesh3d.html
[`Sprite`]: https://docs.rs/bevy/latest/bevy/sprite/struct.Sprite.html
[`Handle<A>`]: https://docs.rs/bevy/latest/bevy/asset/struct.Handle.html
[`AssetPath`]: https://docs.rs/bevy/latest/bevy/asset/struct.AssetPath.html

## The basics of loading assets

The basic approach to loading an asset is pretty simple:
call `AssetServer::load("bevy_bird.png")`, it gives you a handle to the asset,
you put that handle inside of the component that needed asset data to control appearance, behavior or sounds.

```rust
// Breaking the steps down for clarity
fn spawn_bevy_bird_verbose(mut commands: Commands, asset_server: Res<AssetServer>) {
    let handle_to_bevy_bird_image: Handle<Image> = asset_server.load("branding/bevy_bird_dark.png");

	commands.spawn(Sprite {
		image: handle_to_bevy_bird_image,
		..default()
	});
}

// In practice, you'd use something more like this
fn spawn_bevy_bird_idiomatic(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Sprite::from_image(
        asset_server.load("branding/bevy_bird_dark.png"),
    ));
}
```

{% callout(type="info") %}

Bevy looks for assets inside of the `assets` folder.
During development, this should be located inside your project folder,
in the same folder as the `Cargo.toml` of your binary crate.
When shipping your game, this should be an `assets` folder in the same folder as the final binary.

This behavior is a reasonable choice for most projects.
However, this behavior can be overridden by setting the `BEVY_ASSET_ROOT` environment variable for your program. 
You should be careful to only modify this locally, rather than globally, to avoid interfering with other Bevy programs.

{% end %}

Asset loading is, by default, done asynchronously.
This means that, while `AssetServer::load("bevy_bird.png")` will give you a [`Handle<Image>`]
that you can use to set up your [`Sprite`], the program will not block and wait for the image to actually load.

If your asset is large enough (or you are loading many assets at once),
this can take long enough for it to be noticeable.
Objects will look like they "didn't spawn" or look "invisible",
only for them to suddenly "pop in".

You can check the progress of asset loading using [`AssetServer::load_state`],
looking up the asset identifier to provide by calling [`Handle::id`].
As your project grows, you will probably want to use this method to create a loading screen (or something more sophisticated);
waiting for all of your assets to load in before allowing the player to actually play the game.

{% callout(type="info") %}

While loading data as an asset is more complex than simply hardcoding it, doing so unlocks [asset hot reloading](../development-practices/hot-reloading.md).
This allows us to change the asset file during testing and see those changes reflected in real time.

{% end %}

[`Handle<Image>`]: https://docs.rs/bevy/latest/bevy/asset/struct.Handle.html
[`AssetServer::load_state`]: https://docs.rs/bevy/latest/bevy/asset/struct.AssetServer.html#method.load_state
[`Handle::id`]: https://docs.rs/bevy/latest/bevy/asset/struct.Handle.html#method.id

## Mutating handles vs mutating assets

Understanding the [`Handle<A>`] / [`Assets<A>`] distinction becomes quite important when you want to mutate assets.
Should you change the handle that your sprite holds, or the asset that the handle points to?

Both approaches will change what your sprite looks like, and are valid to do, but the effects differ in an important way:

- mutating the handle changes the [`Image`] asset your sprite entity is pointing to
  - this only affects the entity in question
  - this is analogous to replacing a `&` reference
- mutating the asset changes the underlying data that the handle is pointing to
  - this affects *all* entities that point to the same asset
  - this is analogous to mutating the data that your `&` reference is pointing to

To mutate a handle (90% of cases):

```rust
fn swap_player_image(
    mut sprite: Single<&mut Sprite, With<Player>>,
    asset_server: Res<AssetServer>,
) {
    // Only this entity's appearance changes;
	// other sprites are unaffected
    sprite.image = asset_server.load("new_image.png");
}
```

To mutate the underlying asset (10% of cases):

```rust
fn fade_enemy_image_asset(
    sprite: Single<&Sprite, With<EnemyToFade>>,
    mut images: ResMut<Assets<Image>>,
) {
    if let Some(image) = images.get_mut(&sprite.image) {
        // Every entity using this image will be affected!
        for y in 0..image.height() {
            for x in 0..image.width() {
                if let Ok(mut color) = image.get_color_at(x, y) {
                    color.set_alpha(color.alpha() / 2.0);
                    let _ = image.set_color_at(x, y, color);
                }
            }
        }
    }
}
```

[`Image`]: https://docs.rs/bevy/latest/bevy/render/texture/struct.Image.html

## Handles are reference-counted

The [`Handle`] type can be thought of as a [smart pointer] with two key properties:

1. It uses an [`AssetId`] to reference the asset it's pointing to, rather than a true pointer.
2. It is [reference-counted]: when all of the handles to a given asset are dropped, the asset will be unloaded.

Whenever a handle is created (or cloned), the counter of "how many handles reference this asset" goes up by one.
When it is dropped (via replacement, despawning or removing a component, or deleting a resource that stores the handle),
the counter decreases by one.
If this causes the number of handles referencing the asset to hit zero, the data is unloaded.

This behavior is quite useful, as it allows you to dynamically spawn and despawn entities
that rely on different asset data without holding onto all of their assets forever.
That would be, in effect, a memory leak.

However, this behavior can be frustrating when trying to pre-load assets.
Loading all of your assets ahead of time simply won't work if you immediately drop the handles.
Instead, you need to hold onto them somehow.
Resources can work well for this, as can "zoos" of loaded entities or scenes that you quickly clone into your game as needed.

[`Handle`]: https://docs.rs/bevy/latest/bevy/asset/struct.Handle.html
[`AssetId`]: https://docs.rs/bevy/latest/bevy/asset/enum.AssetId.html
[smart pointer]: https://doc.rust-lang.org/book/ch15-00-smart-pointers.html
[reference-counted]: https://doc.rust-lang.org/std/rc/index.html

{% callout(type="warning") %}

Rendering is one of the most important consumers of assets in Bevy: using the images and models that we load to make pretty pixels.
However, when rendering data, we need to load it into VRAM on the GPU;
not ordinary RAM on the CPU.
As a result, Bevy is configured to unload [`RenderAsset`] data from the CPU by default,
in order to save significant amounts of RAM.

This can make assets look like they have been dropped, when they've in fact been moved to the GPU,
leaving only the asset metadata behind.
This behavior can be configured by setting [`RenderAssetUsages`] when loading assets.

[`RenderAsset`]: https://docs.rs/bevy/latest/bevy/render/render_asset/trait.RenderAsset.html
[`RenderAssetUsages`]: https://docs.rs/bevy/latest/bevy/asset/struct.RenderAssetUsages.html

{% end %}
