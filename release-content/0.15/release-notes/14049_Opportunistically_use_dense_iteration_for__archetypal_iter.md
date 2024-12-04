<!-- Opportunistically use dense iteration for  archetypal iteration -->
<!-- https://github.com/bevyengine/bevy/pull/14049 -->

In Bevy, components can be [stored] using one of two different mechanisms,
according to the [`StorageType`] set when implementing the [`Component`] trait.

Table storage is the traditional archetypal ECS storage, where component data is densely packed into tables of raw data with other entities who share the same set of components.
By contrast, sparse set storage keeps the component information out of the table, separating entities by archetype (the set of components they have) without fragmenting otherwise shared tables.

As a result of the map-like storage strategy used by sparse set components, they have faster insertion and removal speed, at the cost of slower random-access iteration.
This is a reasonable tradeoff, but historically, one that Bevy developers were unlikely to use.

That's because a long-standing bug caused iteration to use the slower, fallback sparse-style iteration if even one of the components in the query or its filters were sparse sets, regardless of whether or not this was necessary.
The fix has resulted in query iteration speeds that are between 1.8 and 3.5 times faster (when using parallel iteration) for these scenarios!

Iterating over the data in sparse set components is still relatively slow,
but they should finally be a good default choice for any repeatedly inserted or dataless components.

[stored]: https://docs.rs/bevy/0.15/bevy/ecs/component/trait.Component.html#associatedconstant.STORAGE_TYPE
[`StorageType`]: https://docs.rs/bevy/0.15/bevy/ecs/component/enum.StorageType.html
[`Component`]: https://docs.rs/bevy/0.15/bevy/ecs/component/trait.Component.html#associatedconstant.STORAGE_TYPE
