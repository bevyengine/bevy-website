Bevy does not make any guarantees about the order of items. So if we wish to work with our query items in a certain order, we need to sort them!
We might want to display the scores of the players in order, or ensure a consistent iteration order for the sake of networking stability.
In 0.13 a sort could look like this:

```rust
#[derive(Component, Copy, Clone, Deref)]
pub struct Attack(pub usize)

fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense)>) {
    // An allocation!
    let mut enemies: Vec<_> = enemies.iter().collect();
    enemies.sort_by_key(|(_, atk, ..)| *atk)
    for enemy in enemies {
        work_with(enemy)
    }
}
```

This can get especially unwieldy and repetitive when sorting within multiple systems.
Even if we always want the same sort, different [`Query`] types make it unreasonably difficult to abstract away as a user!
To solve this, we implemented new sort methods on the [`QueryIter`] type, turning the example into:

```rust
// To be used as a sort key, `Attack` now implements Ord.
#[derive(Component, Copy, Clone, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attack(pub usize)

fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense)>) {
    // Still an allocation, but undercover.
    for enemy in enemies.iter().sort::<&Attack>() {
        work_with(enemy)
    }
}
```

To sort our query with the `Attack` component, we specify it as the generic parameter to [`sort`].
To sort by more than one [`Component`], we can do so, independent of [`Component`] order in the original [`Query`] type: `enemies.iter().sort::<(&Defense, &Attack)>()`

The generic parameter can be thought of as being a [lens] or "subset" of the original query, on which the underlying sort is actually performed. The result is then internally used to return a new sorted query iterator over the original query items.
With the default [`sort`], the lens has to be fully [`Ord`], like with [`slice::sort`].
If this is not enough, we also have the counterparts to the remaining 6 sort methods from [`slice`]!

The generic lens argument works the same way as with [`Query::transmute_lens`]. We do not use filters, they are inherited from the original query.
The [`transmute_lens`] infrastructure has some nice additional features, which allows for this:

```rust
fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense, &Rarity)>) {
    for enemy in enemies.iter().sort_unstable::<Entity>() {
        work_with(enemy)
    }
}
```

Because we can add [`Entity`] to any lens, we can sort by it without including it in the original query!

These sort methods work with both [`Query::iter`] and [`Query::iter_mut`]! The rest of the of the iterator methods on [`Query`] do not currently support sorting.
The sorts return [`QuerySortedIter`], itself an iterator, enabling the use of further iterator adapters on it.

Keep in mind that the lensing does add some overhead, so these query iterator sorts do not perform the same as a manual sort on average. However, this *strongly* depends on workload, so best test it yourself if relevant!

[`Query`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Query.html
[`QueryIter`]: https://docs.rs/bevy/0.14/bevy/ecs/query/struct.QueryIter.html
[`sort`]: https://docs.rs/bevy/0.14/bevy/ecs/query/struct.QueryIter.html?search=Component#method.sort
[`Component`]: https://docs.rs/bevy/0.14/bevy/ecs/component/trait.Component.html
[lens]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Query.html#method.transmute_lens
[`Ord`]: https://doc.rust-lang.org/stable/std/cmp/trait.Ord.html
[`slice::sort`]: https://doc.rust-lang.org/nightly/std/primitive.slice.html#method.sort
[`slice`]: https://doc.rust-lang.org/nightly/std/primitive.slice.html
[`Query::transmute_lens`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Query.html#method.transmute_lens
[`transmute_lens`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Query.html#method.transmute_lens
[`Entity`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Entity.html
[`Query::iter`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Query.html#method.iter
[`Query::iter_mut`]: https://docs.rs/bevy/0.14/bevy/ecs/prelude/struct.Query.html#method.iter_mut
[`QuerySortedIter`]: https://docs.rs/bevy/0.14/bevy/ecs/query/struct.QuerySortedIter.html
