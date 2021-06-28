+++
title = "Filtering queries"
weight = 5
template = "book-section.html"
page_template = "book-section.html"
+++

When components are fetched in queries, the data of *every* component in the first type argument passed will be fetched and made available to the system.
However, this isn't always what you want!
In many cases, you just want to filter the query based on the presence (or absence) of a component, and don't want to deal with unpacking data you're never going to use.
This is particularly true when working with **marker components**: data-less structs designed to convey the identity or current state of an entity.

Fortunately, `Query` has two type parameters: the first describes what data to fetch, while the second describes how the entities that would be returned by the first type parameter are then filtered down.
The two most important query filter types are `With<C>` and `Without<C>`, which filter based on whether or not an entity has a component of the type `C`.

Let's demonstrate how these filters are used.

FIXME: add code to this page.
```rust

```

### `Or` Queries

By default, query filters operate on a "and" basis: if you have a filter for `With<A>` and another filter for `With<B>`, only entities with both the `A` and `B` components will be fetched.
We can change this behavior by using the `Or` type, nesting primitive query filters like `With`, `Without` and `Changed` inside of it to return entities that meet any of the criteria inside.
If we wanted to fetch the `Life` component on entities that had either the `A` or `B` components, we would use `Query<&Life, (Or<With<A>, With<B>>)>` as the type of our query.

Note that the `Or` type supports ??? number of type arguments and can be nested indefinitely, allowing you to construct very complex logic if needed.

### Multiple queries in a single system

As the logic in your systems become more complex, you may find that you want to access data from two different queries at once.
In most cases, simply adding a second query as another system parameter works perfectly fine:

```rust

```

But as you use this pattern more, you may encounter an error that looks something like:

```

```

What went wrong? It worked just fine before!

Well, it turns out that Rust, in its infinite wisdom,
does not like it when you access the same data in multiple places at once,
if at least one of those accesses is mutable.
That's a result of its [ownership rules](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html): we could mutate data in the first query while the second query is trying to read the data, resulting in undefined behavior.
Which is bad.

Of course, you already knew that, and have carefully thought about the architecture of your system, designing something like:

```rust

```

You know that there's never going to be an entity that has both `Player` and `Camera` on it, so there's no way that you're ever accessing the same `Transform` component twice.
Unfortunately, Rust *doesn't* know that.
We can fix this by making *sure* our queries our disjoint, no matter what bizarre entities might exist, through the judicious application of `Without` queries.

```rust

```

The other way to get around this issue is to use a `QuerySet`, which permits multiple conflicting queries to exist in a single system.
The catch is that you can only access one query at a time.
Query sets can be useful when you need to access genuinely conflicting data, such as if we truly had an entity with both `Player` and `Camera` that we wanted to operate on in both loops of our system.
Let's rewrite our broken system again, using a `QuerySet` instead.

```rust

```
