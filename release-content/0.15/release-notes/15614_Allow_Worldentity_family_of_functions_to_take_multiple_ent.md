<!-- Allow `World::entity` family of functions to take multiple entities and get multiple references back -->
<!-- https://github.com/bevyengine/bevy/pull/15614 -->

When using some of the more advanced features of Bevy's ECS, like hooks or exclusive systems, it's common to want to fetch entities straight out of a `World`:

```rust
#[derive(Component)]
#[component(on_add = on_foo_added)]
struct Foo;

fn on_foo_added(world: DeferredWorld, entity: Entity, _: ComponentId) {
    let has_foo = world.entity(entity);
    println!("{:?} has a Foo", has_foo.id());
}
```

In previous versions of Bevy, you could grab multiple entities from a `World` using a variety of different functions:

- `World::many_entities<N>(&self, [Entity; N]) -> [EntityRef; N]`
- `World::many_entities_mut<N>(&mut self, [Entity; N]) -> [EntityMut; N]`
- `World::get_many_entities<N>(&self, [Entity; N]) -> Result<[EntityRef; N], Entity>`
- `World::get_many_entities_dynamic(&self, &[Entity]) -> Result<Vec<EntityRef>, Entity>`
- `World::get_many_entities_mut<N>(&mut self, [Entity; N]) -> Result<[EntityMut; N], QueryEntityError>`
- `World::get_many_entities_dynamic_mut(&self, &[Entity]) -> Result<Vec<EntityMut>, QueryEntityError>`
- `World::get_many_entities_from_set_mut(&mut self, &EntityHashSet) -> Result<Vec<EntityMut>, QueryEntityError>`

As you can see, that's a lot of functions with very long names! But the gist of them is that we want to support the ability to
give a bunch of entity IDs, and receive a bunch of entity references. Surely there's a better way!

In `0.15`, all of those functions have been deprecated and now all you need is
the panicking `World::entity`/`World::entity_mut` or the non-panicking `World::get_entity`/`World::get_entity_mut`:

```rust
let e1: Entity = world.spawn_empty().id();
let e2: Entity = world.spawn_empty().id();

// Note: use World::get_entity or World::get_entity_mut instead to receive a Result

// You can still pass a single ID as normal:
let eref = world.entity(e1);  
let emut = world.entity_mut(e1);

// But you can also pass in an array of IDs (any amount N supported!):
let [eref1, eref2]: [EntityRef; 2] = world.entity([e1, e2]);
let [emut1, emut2]: [EntityMut; 2] = world.entity_mut([e1, e2]);

// Or a slice of IDs:
let ids = vec![e1, e2];
let eref_vec: Vec<EntityRef> = world.entity(&ids);
let emut_vec: Vec<EntityMut> = world.entity_mut(&ids);

// Or even a set of IDs:
let ids = EntityHashSet::from_iter([e1, e2]);
let eref_map: EntityHashMap<EntityRef> = world.entity(&ids);
let emut_map: EntityHashMap<EntityMut> = world.entity_mut(&ids);
```

It might *feel* like magic, but it's all standard Rust code! The `Entity` id
parameter that the `World::entity` family of functions accept was changed to
instead accept anything that implements a newly introduced trait:
[`WorldEntityFetch`][WorldEntityFetch]. Check out the trait and
[`World::entity`][World::entity] to learn more about how it was accomplished.

[WorldEntityFetch]: https://docs.rs/bevy/0.15/bevy/ecs/world/trait.WorldEntityFetch.html
[World::entity]: https://docs.rs/bevy/0.15/bevy/ecs/prelude/struct.World.html#method.entity