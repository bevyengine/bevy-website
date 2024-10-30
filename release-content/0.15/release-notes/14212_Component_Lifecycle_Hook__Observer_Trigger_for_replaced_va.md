<!-- Component Lifecycle Hook & Observer Trigger for replaced values -->
<!-- https://github.com/bevyengine/bevy/pull/14212 -->

Bevy 0.14 introduced [Component Lifecycle Hooks and Observers](https://bevyengine.org/news/bevy-0-14/#ecs-hooks-and-observers),
and included several built-in observer triggers for each way that components could be added to or removed from entities: `OnAdd`, `OnInsert` and `OnRemove`.
However, there was a hole in this API. While `OnRemove` is a counterpart to `OnAdd`, `OnInsert` had no such counterpart,
meaning certain operations had no corresponding lifecycle hook or observer trigger:

```rust
use bevy::{
    ecs::component::{ComponentHooks, StorageType},
    prelude::{Commands, Component, Deref, DerefMut, Entity, Query, Resource},
    utils::HashMap,
};

#[derive(Hash, PartialEq, Eq, Clone, Copy)]
struct SomeId(u32);

#[derive(Resource, Deref, DerefMut)]
struct EntityLookupById(HashMap<SomeId, Entity>);

impl Component for SomeId {
    const STORAGE_TYPE: StorageType = StorageType::Table;

    fn register_component_hooks(hooks: &mut ComponentHooks) {
        hooks
            .on_insert(|mut world, entity, _| {
                let this = *world.entity(entity).get::<Self>().unwrap();
                world
                    .resource_mut::<EntityLookupById>()
                    .insert(this, entity);
            })
            .on_remove(|mut world, entity, _| {
                let this = *world.entity(entity).get::<Self>().unwrap();
                world.resource_mut::<EntityLookupById>().remove(&this);
            });
    }
}

fn some_system(mut commands: Commands, query: Query<(Entity, &SomeId)>) {
    let mut iter = query.iter();

    let Some((a_entity, _)) = iter.next() else {
        return;
    };

    let Some((_, &b_id)) = iter.next() else {
        return;
    };

    commands.entity(a_entity).insert(b_id);
}
```

In this example, the system inserts a new component value onto an entity that already has one,
which overwrites the previous component value.
This causes the `on_insert` lifecycle hook to run for the new value,
but the `on_remove` hook doesn't run for the previous value.
As a result, the hashmap entry for the previous ID value is still present, even though it has been replaced.

Bevy 0.15 introduces a new component lifecycle hook and observer trigger for this scenario: `on_replace`/`OnReplace`.
This hook runs just before the `on_remove` hook in all cases,
and additionally runs in the aforementioned scenario where a component value is entirely replaced.
The hook runs just before the replacement occurs,
letting you access the soon-to-be-dropped value to perform bookkeeping and cleanup.

The above example would be fixed by simply replacing the `on_remove` hook with the new `on_replace` hook:

```diff
21                     .resource_mut::<EntityLookupById>()                                          
22                     .insert(this, entity);       
23             })                                   
-24             .on_remove(|mut world, entity, _| {  
+24             .on_replace(|mut world, entity, _| {
25                 let this = *world.entity(entity).get::<Self>().unwrap();                         
26                 world.resource_mut::<EntityLookupById>().remove(&this);                          
27             });                                  
```


Note that it *does not* run if a component value is merely *mutated* - in those cases you want to use change detection instead.

