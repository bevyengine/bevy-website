+++
title = "Fetching data with queries"
weight = 2
template = "book-section.html"
page_template = "book-section.html"
+++

Once we have data stored on our entities in the form of components, we need to be able to get the data back out in a principled way.
**Queries** are precisely that tool; allowing us to carefully request sets of entities from the world that meet the criteria we care about and then retrieve the data we need to operate on.

The {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" no_mod = "true")}} is your primary access into the data in your Bevy game: allowing you to carefully access exactly the data you need from the entity-component data storage.
The {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" no_mod = "true")}} type has two type parameters: the first describes which data should be requested, and the second optional type parameter describes how it should be filtered.

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

In order to access multiple components at once, we need to replace our single `&Life` component with a tuple type that bundles many types into one.
Concretely, `Query<(&Life, &Attack)>` gives us read access to the `Life` and `Attack` components to all entities that have *both* of those components.
Be mindful of this fact when designing queries: queries operate using "AND" logic (unless you use an `Option<&C>` query parameter): adding more components will always strictly reduce the number of entities returned by your query.
We can get mutable access to `Life` and `Attack` separately by changing our `&` (reference) to `&mut` (mutable references) on the components that we want mutable access to.

## Iterating over queries

Once we have a query, the most common thing we're likely to want to do with it is perform some logic on every entity returned.
To do so, we can use straightforward for-loops:

```rust
#[derive(Component, Debug)]
struct Life{
	val: u8
}

#[derive(Component)]
struct IncomingDamge{
	val: u8
}


/// Prints the current life total of every entity with the Life component
fn report_life(query: Query<&Life>){
	for life in query.iter(){
		dbg!(life);
	}
}

#[derive(Component)]
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

For those more experienced with Rust, you will be unsurprised to discover that you can also use [iterator constructs](https://doc.rust-lang.org/std/iter/index.html) like `.for_each`, `.map`, and `.filter` to work with your queries.

If you find yourself needing to iterate over all pairs (or triples or...) of a query (perhaps for collision detection), turn to the {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" method = "iter_combinations")}} method demonstrated in the [corresponding example](https://github.com/bevyengine/bevy/blob/latest/examples/ecs/iter_combinations.rs) to avoid borrow-checker headaches.

## Queries that return one entity

When we have a query that we *know* will always return a single entity, iterating over the query tends to result in unclear code.
To get around this, we can use {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" method = "single")}} and {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" method = "single_mut")}}, depending on whether or not we need to mutate the returned data.

Note that these functions return a {{rust_type(type="enum" crate="std" mod="rest" name="Result" no_mod=true)}}: if you expect this could fail in real scenarios (in case the query does not contain exactly one entity), handle the returned value properly.
Otherwise, just call `let (component_a, component_b) = query.single().unwrap()` to make use of the data quickly.

## Looking up specific entities

Each entity in our ECS data storages has a unique identifier, given by its {{rust_type(type="trait" crate="bevy_ecs" mod = "entity" name="Entity" no_mod = "true")}}, which defines the entity in terms of a `u32` {{rust_type(type="trait" crate="bevy_ecs" mod = "entity" name="Entity" method = "id" no_mod = "true")}} and a `u32` {{rust_type(type="trait" crate="bevy_ecs" mod = "entity" name="Entity" method = "generation" no_mod = "true")}}.
We can fetch the {{rust_type(type="trait" crate="bevy_ecs" mod = "entity" name="Entity" no_mod = "true")}} of each entity returned by our queries by including it as part of the first type parameter of {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" no_mod = "true")}} as if it were a component (although no `&` is used):

```rust
// This system reports the Entity of every entity in your World
fn all_entities(query: Query<Entity>){
	for entity in query.iter(){
		dbg!(entity);
	}
}

#[derive(Component)]
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

Once we have a particular entity in mind, we can grab its data using {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" method = "get" no_mod = "true")}} and the related methods on {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" no_mod = "true")}}.
This is fallible, and so it returns a {{rust_type(type="enum" crate="std" mod="result" name="Result" no_mod=true)}} that you must unwrap or handle.

## Optional components in queries

If we want a query to include a component's data if it exists, we can use an `Option<&MyComponent>` query parameter in the first type parameter of {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" no_mod = "true")}}.
This can be a powerful tool for branching logic (use {{rust_type(type="keyword" crate="std" name="match" no_mod=true)}} on the {{rust_type(type="enum" crate="std" mod="option" name="Option" no_mod=true)}} returned), especially when combined with marker components.

## Query filtering

When components are fetched in queries, the data of *every* component in the first type argument passed will be fetched and made available to the system.
However, this isn't always what you want!
In many cases, you just want to filter the query based on the presence (or absence) of a component, and don't want to deal with unpacking data you're never going to use.
This is particularly true when working with **marker components**: data-less structs designed to convey the identity or current state of an entity.

Fortunately, {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="Query" no_mod = "true")}} has two type parameters: the first describes what data to fetch, while the second describes how the entities that would be returned by the first type parameter are then filtered down.
The two most important query filter types are {{rust_type(type="struct" crate="bevy_ecs" mod = "query" name="With")}} and {{rust_type(type="struct" crate="bevy_ecs" mod = "query" name="Without")}}, which filter based on whether or not an entity has a component of the type in their generic type argument.

Let's demonstrate how these filters are used by filtering some fruit.

```rust
#[derive(Component)]
enum Fruit{
	Apple,
	Orange,
	Durian,
	Kiwi
}

#[derive(Component)]
struct Delicious;

#[derive(Component)]
struct Rotten;

#[derive(Default)]
struct FruitInventory{
	map: HashMap<Fruit, u8>
}

// The second filtering type parameter of Query can be ommitted when no filter is used
fn take_inventory_system(fruit_query: Query<&Fruit>, fruit_inventory: FruitInventory){
	// Restart the count each time inventory is taken
	fruit_inventory = FruitInventory::default();
	for &fruit in fruit_inventory.iter(){
		let fruit_type_count = fruit_inventory.map.get_mut(fruit);
		fruit_type_count = match fruit_type_count {
			None => 1,
			Some(n) => n + 1,
		}
	}
}

// With restricts your query to only those entities
// who have components that match all of the data requested *and* all of the With filters
fn clean_inventory_system(rotten_food_query: Query<Entity, With<Rotten>>, mut commands: Commands){
	for entity in rotten_food_query.iter(){
		commands.despawn(entity);
	}
}

// We can combine query filters by passing them in as tuple
// Returning entities that meet *all* of the query filters criteria
fn eat_fruit_system(query: Query<&Fruit, (Without<Rotten>, With<Delicious>)>){
	// Perform complicated fruit-eating logic here!
}

```

You've probably noticed that `Query<(&Fruit, &Delicious)>` will always return the same set of entities as `Query<&Fruit, With<Delicious>>`.
So what's the difference?

By using `With` filters instead of requesting the data directly, we only check whether a component *has* that value, rather than fetching that component's value.
As a result, we don't need to unpack our query into `(fruit, _delicious)` when iterating over it, and as a bonus, we can allow other systems to change the value of our components that are only include in our `With` filters in parallel to the systems being run!

### `Or` Queries

By default, query filters (just like query data requests) operate on a "and" basis: if you have a filter for `With<A>` and another filter for `With<B>`, only entities with both the `A` and `B` components will be fetched.
We can change this behavior by using the {{rust_type(type="struct" crate="bevy_ecs" mod = "query" name="Or")}} type, nesting primitive query filters like {rust_type(type="struct" crate="bevy_ecs" mod = "query" name="With")}}, {{rust_type(type="struct" crate="bevy_ecs" mod = "query" name="Without")}} and {{rust_type(type="struct" crate="bevy_ecs" mod = "query" name="Changed")}} inside of it to return entities that meet any of the criteria inside.
If we wanted to purchase fruits that were either `Delicious` or `Cheap`, we would use `Query<&mut Owner, (Or<With<Deliciou>, With<Cheap>>)>` as the type of our query, allowing us to change the owner of any delicious and cheap fruit that we found.

Note that the {rust_type(type="struct" crate="bevy_ecs" mod = "query" name="Or")}} type (and other query tuples) can be nested indefinitely, allowing you to construct very complex logic if needed.

### Running multiple queries at once

As the logic in your systems become more complex, you may find that you want to access data from two different queries at once.
In most cases, simply adding a second query as another system parameter works perfectly fine:

```rust
fn defense_aura_system(aura_query: Query<&Transform, With<Aura>>, target_query: Query<(&mut Defense, &Transform), With<Creature>>){
	// Give all allies near an aura-generator a bonus to their defense
}
```

But as you use this pattern more, you may encounter an error that looks something like:

```
   Query<&mut Transform, With<Camera>> in system move_player accesses component(s) &mut Transform in a way that conflicts with a previous system parameter. Allowing this would break Rust's mutability rules. Consider using `Without<T>` to create disjoint Queries or merging conflicting Queries into a `QuerySet`.
```

What went wrong? It worked just fine before!

Well, it turns out that Rust, in its infinite wisdom,
does not like it when you access the same data in multiple places at once,
if at least one of those accesses is mutable.
That's a result of its [ownership rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html): we could mutate data in the first query while the second query is trying to read the data, resulting in undefined behavior.
Which is bad.

Of course, you already knew that, and have carefully thought about the architecture of your system, designing something like:

```rust
fn camera_follow_system(player_query: Query<&Transform, With<Player>>, camera_query: Query<&mut Transform, With<Camera>>){
	let player_transform = player_query.single().unwrap();
	let camera_query = camera_query.single_mut.unwrap();
	// Insert logic here
}
```

You know that there's never going to be an entity that has both `Player` and {{rust_type(type="struct" crate="bevy_render" mod = "camera" name="Camera")}} on it, so there's no way that you're ever accessing the same {{rust_type(type="struct" crate="bevy" mod = "transform/components" name="Transform" no_mod = "true")}} component twice.
Unfortunately, Rust *doesn't* know that.
We can fix this by making *sure* our queries our disjoint, no matter what bizarre entities might exist, through the judicious application of `Without` queries.

```rust
fn camera_follow_system(player_query: Query<&Transform, With<Player>>, camera_query: Query<&mut Transform, (With<Camera>, Without<Player>)>){
	let player_transform = player_query.single().unwrap();
	let camera_query = camera_query.single_mut.unwrap();
	// Insert logic here
}
```

The other way to get around this issue is to use a {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="QuerySet")}}, which permits multiple conflicting queries to exist in a single system.
The catch is that you can only access one query at a time.
Query sets can be useful when you need to access genuinely conflicting data, such as if we truly had an entity with both `Player` and {{rust_type(type="struct" crate="bevy_render" mod = "camera" name="Camera")}} that we wanted to operate on in both loops of our system.
Let's rewrite our broken system again, using a {{rust_type(type="struct" crate="bevy_ecs" mod = "system" name="QuerySet")}} instead.

```rust
fn camera_follow_system(queries: QuerySet<Query<&Transform, With<Player>>, Query<&mut Transform, With<Camera>>){
	let player_transform = queries.0.single().unwrap();
	let camera_query = queries.1.single_mut.unwrap();
	// Insert logic here
}
```

Bevy's systems automatically run in parallel by default, so long as the scheduler can guarantee that the same data is never accessed in another place while it is being mutated.

As a result, we can use the same query filtering techniques described  to allow our *systems* to safely run in parallel.
In addition to improving parallelism, this also reduces the false positives when checking for [system execution order ambiguities](https://docs.rs/bevy/latest/bevy/ecs/schedule/struct.ReportExecutionOrderAmbiguities.html), as we can guarantee that the relative order of two systems that do not share data never changes the final outcome.
