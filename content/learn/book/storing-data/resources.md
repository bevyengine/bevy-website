+++
title = "Resources"
insert_anchor_links = "right"
[extra]
weight = 5
status = 'hidden'
+++

Resources are used to represent global, shared data that exists independently of any specific entity
and which can be accessed by any system in your application. Unlike components, which are attached
to individual entities, resources provide a way to store singleton data such as game settings, asset
handles, time information, or any other state that needs to be accessible across multiple systems.

To create a new resource type, simply create a Rust `struct` or `enum`, and derive the [`Resource`]
trait:

```rs
#[derive(Resource)]
struct AudioSettings {
    music_volume: f32,
    speech_volume: f32,
    effects_volume: f32,
}
```

For each unique resource type `T`, there can only be one instance of that type in the Bevy [`World`].
If you might need multiple instances, then consider using [entities and components] instead.

Bevy uses resources for many of the built-in features of the engine. For example, Bevy's
[`AssetServer`] is a resource.

## Accessing Resources

To access the value of a resource in a system, use [`Res`] for immutable access, and [`ResMut`] for mutable access:

```rs
fn audio_settings_system(settings: Res<AudioSettings>) {
    println!("Music volume: {}", settings.music_volume);
    println!("Speech volume: {}", settings.speech_volume);
    println!("Effects volume: {}", settings.effects_volume);
}

fn update_music_volume(mut settings: ResMut<AudioSettings>) {
    // Adjust the music volume
    settings.music_volume = 0.8;
}
```

## Initializing Resources

Typically resources are created during app initialization, and exist as long as the [`App`] does.
To insert a resource with a specific value, call `.insert_resource()`:

```rs
App::new()
    .add_plugins(DefaultPlugins)
    .insert_resource(AudioSettings {
        music_volume: 0.7,
        speech_volume: 0.7,
        effects_volume: 0.6,
    });
```

Alternatively, you can insert the default value of a resource with `.init_resource<T>`:

```rs
App::new()
    .add_plugins(DefaultPlugins)
    .init_resource::<AudioSettings>();
```

This requires that the resource implement either `Default` or [`FromWorld`]. The latter is a trait
that lets you construct a new instance of the type using data already in the Bevy world.

## Dynamic Resources

It is also possible to insert and remove resources at runtime:

```rs
fn setup_audio_settings(mut commands: Commands) {
    // add (or overwrite if existing) a resource, with the given value
    commands.insert_resource(AudioSettings {
        music_volume: 0.7,
        speech_volume: 0.7,
        effects_volume: 0.6,
    });
    // ensure resource exists (create it with its default value if necessary)
    commands.init_resource::<AudioSettings>();
    // remove a resource (if it exists)
    commands.remove_resource::<AudioSettings>();
}
```

You can also do the same via direct world access:

```rs
fn setup_audio_settings(world: &mut World) {
    // The same methods as used with Commands are also available here, but
    // we can do more elaborate things:

    // Check if resource exists:
    if !world.contains_resource::<AudioSettings>() {
        // Get access to a resource, inserting if unavailable
        let music_volume = world.get_resource_or_insert_with(
            || AudioSettings {
                music_volume: 0.7,
                speech_volume: 0.7,
                effects_volume: 0.6,
            }).music_volume;
    }
}
```

{% callout(type="warn") %}
**Caution**: Use care when accessing resources which may not exist. Attempting to inject a
non-existent resource using [`Res`] or [`ResMut`] will cause a panic. You can avoid this by
wrapping the resource in `Option`:

```rs
fn audio_settings_system(settings_res: Option<Res<AudioSettings>>) {
    if let Some(settings) = settings_res {
        // Do stuff with audio settings
    }
}
```

{% end %}

## Resources vs Singleton Entities

Not every singleton object has to be stored as a resource, and in some cases you may not want to.
Any data that is storable in a resource could instead be stored as a component on a singleton
entity; which approach is better is a design question that depends on a number of tradeoffs.

The advantages of resources is simplicity and ease of use: it requires very little code to read or
write data stored in a resource. Accessing data within a component is more involved, and entails a
multi-step process where you first have to get access to the entity, and then its components. Often
this will involve writing a query.

Resources make sense when the data is truly singular and always will be. For example, there can
only be one master audio volume on a device, so it doesn't make sense to allow for multiple
instances.

However, sometimes things that appear at first glance to be singletons...aren't.

For example, there might only be one instance of a type now, but perhaps there will be more in the
future. Your single-player game only supports one game controller, but what if you decide later to
make it a split-screen co-op game? Then you'll need more than one gamepad.

Also, an object which is unique might be a member of a larger class: in a single-player game, there
is only one player avatar, but that avatar is going to be part of a larger collection of characters
or ships which are rendered and simulated using common ECS systems. If at any point your object
is going to be part of a collection, it can't be a resource.

Finally, a resource can only be a single `struct` or `enum`. If the object is a composite of
multiple structs, then using components is the way to go.

[`Resource`]: https://docs.rs/bevy/latest/bevy/prelude/trait.Resource.html
[`Res`]: https://docs.rs/bevy/latest/bevy/prelude/struct.Res.html
[`ResMut`]: https://docs.rs/bevy/latest/bevy/prelude/struct.ResMut.html
[`App`]: https://docs.rs/bevy/latest/bevy/prelude/struct.App.html
[`FromWorld`]: https://docs.rs/bevy/latest/bevy/prelude/trait.FromWorld.html
