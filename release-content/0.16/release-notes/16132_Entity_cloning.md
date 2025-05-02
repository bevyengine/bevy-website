<!-- Entity cloning -->
<!-- https://github.com/bevyengine/bevy/pull/16132 -->
Bevy now has first-class support for cloning entities. While it was possible to do this before using reflection and `ReflectComponent` functionality, the common implementation was slow and required registering all cloneable components. With **Bevy 0.16**, entity cloning is supported natively and is as simple as adding `#[derive(Clone)]` to a component to make it cloneable.

```rust
#[derive(Component, Clone)]
#[require(MagicalIngredient)]
struct Potion;

#[derive(Component, Default, Clone)]
struct MagicalIngredient {
    amount: f32,
}

fn process_potions(
    input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    mut potions: Query<(Entity, &mut MagicalIngredient), With<Potion>>,
) {
    // Create a new potion
    if input.just_pressed(KeyCode::KeyS) {
        commands.spawn(
            (Name::new("Simple Potion"), Potion)
        );
    }
    // Add as much magic as we want
    else if input.just_pressed(KeyCode::KeyM) {
        for (_, mut ingredient) in potions.iter_mut() {
            ingredient.amount += 1.0
        }
    }
    // And then duplicate all the potions!
    else if input.just_pressed(KeyCode::KeyD) {
        for (potion, _) in potions.iter() {
            commands.entity(potion).clone_and_spawn();
        }
    }
}

```

`clone_and_spawn` spawns a new entity with all cloneable components, skipping those that can't be cloned. If your use case requires different behavior, there are more specialized methods:

- `clone_components` clones components from the source entity to a specified target entity instead of spawning a new one.
- `move_components` removes components from the source entity after cloning them to the target entity.
- `clone_and_spawn_with` and `clone_with` allow customization of the cloning behavior by providing access to `EntityClonerBuilder` before performing the clone.

`EntityClonerBuilder` can be used to configure how cloning is performed - for example, by filtering which components should be cloned, modifying how `required` components are cloned, or controlling whether entities linked by relationships should be cloned recursively.

An important note: components with generic type parameters will not be cloneable by default. For these cases, you should add `#[derive(Reflect)]` and `#[reflect(Component)]` to the component and register it for the entity cloning functionality to work properly.

```rust
#[derive(Component, Reflect, Clone)]
#[reflect(Component)]
struct GenericComponent<T> {
    // ...
}

fn main(){
    // ...
    app.register_type::<GenericComponent<i32>>;
    // ...
}
```

See documentation for `EntityCloner` if you're planing on implementing custom clone behaviors for components or need further explanation on how this works.