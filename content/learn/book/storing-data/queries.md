+++
title = "Queries"
insert_anchor_links = "right"
[extra]
weight = 4
status = 'hidden'
+++

Queries are your primary tool for interacting with the Bevy world,
allowing you to carefully and efficiently read and write component data from matching entities.

## Anatomy of a query

To understand how queries work in a bit more detail, let's take a look at the anatomy of a [`Query`].
The [`Query`] type has two generics: `D`, which must implement the [`QueryData`] trait,
and `F`, which must implement the [`QueryFilter`] trait.
`D` describes "which data should I access", while `F` describes "how should I restrict the returned entities".
Only entities which match *all* of the terms in `D` *and* `F` will be included in your query.

When we write `Query<&Life, With<Player>>`, we're supplying these generics, setting `D` to `&Life` and `F` to `With<Player>`,
separated by a comma.
Bevy uses the information in the [`QueryData`] and [`QueryFilter`] traits,
along with the [`WorldQuery`] supertrait, in order to know how to lookup components of
the correct type in the world and supply them to your system via [dependency injection].

If we don't want to fetch any data, or perform any filtering,
we can use `()`, Rust's "unit type" in the place of `D` or `F`.
The `F: QueryFilter` argument defaults to `()`, allowing us to conveniently write
`Query<&Life>`, rather than spelling out that we don't want to filter via `Query<&Life, ()>`.

To access more than one component at once, or filter by multiple predicates at the same time,
we can combine [`QueryData`] or [`QueryFilter`] types by putting them inside of a [tuple],
wrapping them with parentheses.

If we have two components, `Life` and `Mana`, we can fetch all entities with the `Life` component
by requesting `Query<&Life>` (if we want the data), or `Query<(), With<Life>>` (if we don't need the data).
We can do the same thing for `Mana`, and we could add two queries in the same system, one for `Life`, and one for `Mana`,
if we wanted to examine these two lists side by side.

But to fetch entities with *both* the life and mana components,
we would need:

- `Query<(&Life, &Mana)>`, giving us access to the data of both components
- `Query<&Life, With<Mana>>` or `Query<&Mana, With<Life>>`, accessing the data of one and filtering on the presence of the other
- `Query<(), (With<Life>, With<Mana>)>` to filter on both components

These tuples can be extended to multiple elements: `Query<(&Life, &Mana, &Energy)>` and so on works just fine!
To swap from the default "and" logic, where all of the components must be present, to "or" logic,
where any of the components can be present, you can use:

- `Query<Option<&Life>>`, for an [`Option`] that contains the component value if present and nothing if it is absent
- `Query<AnyOf<(&Life, &Mana)>>` which acts as a wrapper for multiple `Option` `QueryData` types
- `Query<Has<Life>>`, which contains `true` if the component is present, and `false` if the component is absent
- `Query<(), Or<(With<Life>, With<Mana>)>>`, which combines query filters via an `Or` relationship

As you can see, Bevy's type-driven querying can be quite expressive and elaborate!
Don't worry though: most of your queries will be quite simple, requesting a few pieces of data with some filter.

[dependency injection]: https://en.wikipedia.org/wiki/Dependency_injection

## Mutable and immutable query data

## Query::get

ENTITY QUERY DATA

QUERY GET

## Working with singleton entities

QUERY::SINGLE

CONTRAST TO SINGLE SYSTEM PARAM

## Accessing multiple items from the same query

QUERY::GET_MANY

## Multiple queries in a single system

MUTABILITY RULES.
AVOID WITH WITHOUT
PARAMSET

## Disabling entities

DEFAULT QUERY FILTERS

DISABLED COMPONENT

## Working with complex queries

DERIVE QUERYDATA / QUERYFILTER

USE SYSTEMPARAM DERIVE INSTEAD

CONTRAST TO TYPE ALIASES

## Accessing all data from a single entity via queries

- change detection
- Mut and Ref
- EntityMut, EntityRef, FilteredEntityMut, FilteredEntityRef, EntityMutExcept, EntityRefExcept
