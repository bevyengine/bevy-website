+++
title = "Systems access data through queries"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
+++

As we saw in the example given on the last page, systems are used to run the logic of our game and use queries to access data stored in the ECS.
Let's break down how the basics of systems and queries in more detail here.

**Systems** are functions that automatically receive data from the `World` from the scheduler according to their **system parameters**, and can mutate that data to change the world.
Any type which implements the [`SystemParam`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html) trait can be used as an argument in your system: this trait tells the scheduler how to pass out access to the `World` in a safe and efficient way.

Most commonly, you'll be using:

- [`Query`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html), to access entity-component database
- [`Res`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Res.html) and [`ResMut`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.ResMut.html), to access the global singleton data stored in resources
- [`Commands`](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Commands.html), to queue up complex changes to the world like spawning entities
- [`EventWriter`](https://docs.rs/bevy/latest/bevy/app/struct.EventWriter.html) and [`EventReader`](https://docs.rs/bevy/latest/bevy/app/struct.EventReader.html), to work with events in an ergonomic fashion

You can see the full list by checking the [API docs for `SystemParam`](https://docs.rs/bevy/latest/bevy/ecs/system/trait.SystemParam.html#implementors).

You can add systems to your app using [`AppBuilder::add_system`](https://docs.rs/bevy/latest/bevy/app/struct.AppBuilder.html#method.add_system), which will cause them to run once on every pass through the game loop.

## Startup systems

In many cases, you don't *want* your systems to run constantly: instead, you may only want to have them run a single time at the beginning to perform some setup.
To do this, use a **startup system**.

Startup systems run exactly once, before any ordinary systems, and can add them using [`AppBuilder::add_startup_system](https://docs.rs/bevy/latest/bevy/app/struct.AppBuilder.html#method.add_startup_system).

## Query basics

The `[Query](https://docs.rs/bevy/latest/bevy/ecs/system/struct.Query.html)` type is your primary access into the data in your Bevy game: allowing you to carefully access exactly the data you need from the entity-component data storage.
The `Query` type has two type parameters: the first describes which data should be requested, and the second optional type parameter describes how it should be filtered.

We can request access to a single component (`Life`, in this case), by adding `Query<&Life>` to our system parameters.
This will request (a reference to) the `Life` component on every entity that has that component.
This reference is read-only: we cannot mutate the values contained within.

If we want to be able to change these values, we need to use `Query<&mut Life>` instead, and remember to set our `query` function parameter to be mutable as well.

```rust
// A system that has read-access to the Life component of each entity
fn read_life(query: Query<&Life>){}

// A system that has write-access to the Life component of each entity
fn write_life(mut query: Query<&mut Life>){}
```

In order to access multiple components at once, we need to use a tuple type as our first type parameter.
`Query<(&Life, &Attack)>` gives us read access to the `Life` and `Attack` components to all entities that have *both* of those components.
Be mindful of this fact when designing queries: queries operate using "AND" logic (unless you use an `Option<&C>` query parameter): adding more components will always strictly reduce the number of entities returned by your query.
We can get mutable access to `Life` and `Attack` separately by changing our `&` (reference) to `&mut` (mutable references) on the components that we want mutable access to.

## Iterating over queries

Once we have a query, the most common thing we're likely to want to do with it is perform some logic on every entity returned.
To do so, we can use straightforward for-loops:

```rust
#[derive(Debug)]
struct Life{
	val: u8
}

struct IncomingDamge{
	val: u8
}


/// Prints the current life total of every entity with the Life component
fn report_life(query: Query<&Life>){
	for life in query.iter(){
		dbg!(life);
	}
}

struct Age(u64);

fn increment_age(query: Query<&mut Age>){
	// We need to use mut age and .iter_mut() here because we need mutable access
	for mut age in query.iter_mut(){
		// age.0 refers to the first (only) field on our tuple type
		// We could make this more ergonomic by implementing the Add<Age, u64> trait
		// or the AddAssign<Age> trait on our Age component type
		age.0 =  age.0 + 1;
	}
}

fn take_damage(query: Query<(&mut Life, &mut IncomingDamage)>){
	// Typically you want to unpack this iterator into several variables 
	// that you can use in your loop
	for (mut life, mut incoming_damage) in query.iter_mut(){
		life.val -= incoming_damage.val;
		incoming_damage.val = 0;
	}
}
```

For those more experienced with Rust, you will be unsurprised to discover that you can also use iterator constructs like `.for_each`, `.map`, and `.filter` to work with your queries.

If you find yourself needing to iterate over all pairs (or triples or...) of a query (perhaps for collision detection), turn to the `iter_combinations` function demonstrated in the [corresponding example](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/iter_combinations.rs) to avoid borrow-checker headaches.

## Queries that return one entity

When we have a query that we *know* will always return a single entity, iterating over the query tends to result in unclear code.
To get around this, we can use [`Query::single()`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.Query.html#method.single) and [`Query::single_mut()`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.Query.html#method.single_mut), depending on whether or not we need to mutate the returned data.

Note that these functions return a `Result`: if you expect this could fail in real scenarios (in case the query does not contain exactly one entity), handle the result properly.
Otherwise, just call `let (component_a, component_b) = query.single().unwrap()` to make use of the data quickly.

## Looking up specific entities

Each entity in our ECS data storages has a unique identifier, given by its [`Entity`](https://docs.rs/bevy/0.5.0/bevy/ecs/entity/struct.Entity.html), which defines the entity in terms of a `u32` [`id`](https://docs.rs/bevy/0.5.0/bevy/ecs/entity/struct.Entity.html#method.id) and a `u32` [`generation`](https://docs.rs/bevy/0.5.0/bevy/ecs/entity/struct.Entity.html#method.generation).
We can fetch the `Entity` of each entity returned by our queries by including it in the first type parameter of `Query`:

```rust
// This system reports the Entity of every entity in your World
fn all_entities(query: Query<Entity>){
	for entity in query.iter(){
		dbg!(entity);
	}
}

struct Marker;
struct MyEntities{
	entities: Vec<Entity>,
}
// Typically you'll combine this pattern with query filters 
// to extract the entities of a relevant subset, 
// and then store it somewhere where you can access it later
fn identify_yourself(query: Query<Entity, With<Marker>>, my_entities: ResMut<MyEntities>){
	for entity in query.iter(){
		my_entities.push(entity);
	}
}

```

Once we have a particular entity in mind, we can grab its data using [`Query::get()`](https://docs.rs/bevy/0.5.0/bevy/ecs/system/struct.Query.html#method.get) and the related methods.
This is fallible, and so it returns a `Result` that you must unwrap or handle.

## Optional components in queries

If we want a query to include a component's data if it exists, we can use an `Option<&MyComponent>` query parameter in the first type parameter of `Query`.
This can be a powerful tool for branching logic (use `match` on the `Option` returned), especially when combined with marker components.
