<!-- Add mesh picking backend and `MeshRayCast` system parameter -->
<!-- https://github.com/bevyengine/bevy/pull/15800 -->

TODO: Laser beam image

Being able to click on objects to select them is a vital and seemingly simple task in any game.
Since 2020, doing this in Bevy has largely meant pulling in `@aevyrie`'s beloved ecosystem crate, [`bevy_mod_picking`] and it's simple raycasting companion [`bevy_mod_raycast`].

Over the years, this crate has been refined and battle-tested, both by [Foresight Spatial Labs] (a CAD-creating, Bevy-using company co-founded by Aevyrie) and
the broader open source community of game developers that have used it for everything from first-person-shooters to point-and-click adventures.
Bevy is thrilled to have had the chance to work with the team behind [`bevy_mod_picking`] and have adopted the project wholesale into Bevy itself.
Integrating a large project is a ton of work, and we're incredibly grateful to the contributors who have made `bevy_picking` a stable, first-class feature of the engine.

The new `bevy_picking` crate follows the existing modular architecture closely:

1. Inputs are gathered from mouse, touch and pen devices. Each pointing device (humans are equipped with 10 by default) gets a screen-space [`PointerLocation`].
2. Each modular [backend] performs the domain-specific work (like raycasting) of figuring out how these pointer locations map to [`PointerHits`] on objects that they're watching.
3. The hit information from each backend is combined and sorted to produce a coherent [`HoverMap`], which lists which entities each pointer is hovering over.
4. High level events (both ordinary events and observers!) are emitted for each hovered entity, capturing complex behavior such as clicking, dragging or releasing various objects.

In Bevy 0.15, we're shipping with UI, sprite and mesh backends. Each of these comes with its own caveats for now:

- UI: both the legacy [`Interaction`] and new [`PickingInteraction`] components exist [for now](https://github.com/bevyengine/bevy/issues/15550), with subtle behavioral differences.
- Sprites: picking always uses the full rectangle, and [alpha transparency is not taken into account](https://github.com/bevyengine/bevy/issues/14929).
- Mesh: this is a naive raycast against the full mesh, and can be painfully slow. You should be using simplified meshes and an acceleration data structure like a BVH to speed this up if you are using this functionality for games. As a result, this functionality is disabled by default.

We expect both [`bevy_rapier`] and [`avian`] (the two most popular ecosystem physics crates for Bevy) to update their accelerated mesh picking implementations to work with the newly upstreamed API. Unless you're debugging, building an editor or really care about the exact triangles of raw meshes, you should use one of those crates for efficient mesh picking.

## Usage

If you haven't used `bevy_picking`'s predecessor, there are two important and straightforward ways to get started with the API.

First, you might want to quickly update the state of your objects (be they UI or game objects) based on what is being done to them, typically highlighting them or changing their color. For that, simply match against the [`PickingInteraction`] component.

Secondly, you might want to respond dynamically to various pointer-powered events. For that, we recommend using observers (which replaced the existing `bevy_event_listener` solution during the upstreaming process).
Here, we're spawning a simple text node and responding to pointer events.

```rust
use bevy::prelude::*;

let app = App::new()
    .add_plugins(DefaultPlugins)
    .add_systems(Startup, spawn_buttons)
    .add_systems(Update, update_counter_display)
    // Observers added globally will watch for events sent to *any* entity
    .add_observer(change_text_color_on_hover)
    .add_observer(reset_text_color);

fn change_text_color_on_hover(out: Trigger<Pointer<Out>>, mut text_colors: Query<&mut TextColor>){
    if let Ok(mut text_color) = text_colors.get_mut(over.entity()){
        text_color.0 = Color::gray(0.8);
    }
}


fn reset_text_color(over: Trigger<Pointer<Out>>, mut text_colors: Query<&mut TextColor>){
    if let Ok(mut text_color) = text_colors.get_mut(out.entity()){
        text_color.0 = Color::gray(0.5);
    }
}

#[derive(Component)]
struct Counter(i32);

fn spawn_buttons(mut commands: Commands){
    commands.spawn((Text::new("Counter: 0"), Counter(0)));

    commands
        .spawn((
            Text::new("Count up :)"),
        )
        // Observers added to a single entity only watch for events to that specific entity
        .observe(|_click: Trigger<Pointer<Click>>, counter_query: Query<&mut Counter>| {
            if let Ok(mut counter) = counter_query.get_single_mut(){
                counter.0 += 1;
            }
        }));

    commands
        .spawn((
            Text::new("Count down :("),
        )
        .observe(|_click: Trigger<Pointer<Click>>, counter_query: Query<&mut Counter>| {
            if let Ok(mut counter) = counter_query.get_single_mut(){
                counter.0 -= 1;
            }
        }));
}

// Systems with the new `Single` system param are skipped if their query doesn't return exactly one elemnent
fn update_counter(singleton_query: Single<(&mut Text, &Counter)>){
    let (mut text, counter) = singleton_query;
    *text = Text::new(format!("Counter: {}", counter.0));
}
```

If you want to control how an entity interacts with picking, add the [`PickingBehavior`] component to them and configure it to meet your needs.

[`bevy_mod_picking`]: https://crates.io/crates/bevy_mod_picking/
[`bevy_mod_raycast`]: https://crates.io/crates/bevy_mod_raycast/
[Foresight Spatial Labs]: https://www.fslabs.ca/
[`PointerLocation`]: https://docs.rs/bevy/0.15.0/bevy/picking/backend/prelude/struct.PointerLocation.html
[backend]: https://docs.rs/bevy/0.15.0/bevy/picking/backend/index.html
[`PointerHits`]: https://docs.rs/bevy/0.15.0/bevy/picking/backend/struct.PointerHits.html
[`HoverMap`]: https://docs.rs/bevy/0.15.0/bevy/picking/focus/struct.HoverMap.html
[`Interaction`]: https://docs.rs/bevy/0.15.0/bevy/prelude/enum.Interaction.html
[`PickingInteraction`]: https://docs.rs/bevy/0.15.0/bevy/picking/focus/enum.PickingInteraction.html
[`bevy_rapier`]: https://crates.io/crates/bevy_rapier3d
[`avian`]: https://crates.io/crates/avian3d
[`PickingBehavior`]: https://docs.rs/bevy/0.15.0/bevy/picking/struct.PickingBehavior.html
