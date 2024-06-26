Currently, query iteration order is not guaranteed. If we wish to work with our query items in a certain order, we need sorting.
We might want sort our queries to f.e. display a list of things, or fold over our query with some order-dependent math operation.
In 0.13 a sort could look like this:

```rust
#[derive(Component, Copy, Clone, Deref)]
pub struct Attack(pub usize)

fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense)>) {
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
    for enemy in enemies.iter().sort::<&Attack>() {
        work_with(enemy)
    }
}
```

To sort our query with the `Attack` component, we specify it as the generic parameter to [`sort`].
This parameter can be thought of as being a [lens](https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.Query.html#method.transmute_lens) or "subset" of the original query, on which the underlying sort is actually performed. The result is then internally used to return a new sorted query iterator over the original query items.
With the default [`sort`], the lens has to be fully [`Ord`], like with [`slice::sort`].
If this is not enough, then the rest of sort methods from [`slice`] also have their counterpart!

The generic [`lens`] argument works the same way as in [`Query::transmute_lens`]. We do not use filters, they are inherited from the original query.
The [`transmute_lens`] infrastructure has some nice additional features, which allows for this:

```rust
fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense, &Rarity)>) {
    for enemy in enemies.iter().sort_unstable::<Entity>() {
        work_with(enemy)
    }
}
```

Because we can add [`Entity`] to any lens, we can sort by it without including it in the original query!

These sort methods are fully generic over mutability, so can be used on both [`Query::iter`] and [`Query::iter_mut`]! The rest of the of the iterator methods on [`Query`] do not support sorting.
The sorts return [`QuerySortedIter`], itself an iterator, enabling the use of further iterator adapters on it.
Further, [`QuerySortedIter`] implements [`DoubleEndedIterator`] while the initial [`QueryIter`] does not. As a consequence, an empty sort can be used to get a more powerful iterator type:

```rust
fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense, &Rarity)>) {
    // A reversible query iterator!
    enemies.iter_mut().sort::<()>().rev();
}
```

Additionally, these query iterator sorts offer a workaround for a restriction on slice sorts:

```rust
// Some `Component` that holds data in an `Arc`.
// `StatisticsData` does not implement `Copy`.
#[derive(Component, Clone, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub struct Statistics(pub Arc<StatisticData>)

// Does not compile.
fn show_stats(users: Query<(&User, &Statistics)>) {
   let users: users.iter().collect();
   users.sort_by_key(|(_, stats)| *stats)
   show(users)
}

// Compiles.
fn show_stats_2(users: Query<(&User, &Statistics)>) {
   let users: users.iter().sort::<&Statistics>();
   show(users)
}
```

In current Rust, we can not return references from the key extraction closure in [`slice::sort_by_key`]/[`slice::sort_by_key`].
This can become a headache when using non-`Copy` [`Component`]s.
The new `sort`/`sort_by` methods can work around this by having the lensing be the "key extraction", removing the need for such a closure.

Keep in mind that the lensing does add some overhead, so these query iterator sorts do not perform equally to a manual sort on average. However, this *strongly* depends on workload, so best test it yourself if relevant!
The sorts themselves are not yet cached between system runs, and these methods only sort iterators, not underlying query storage.

Note that query iteration might happen in a deterministic order for now, but that may change anytime over future releases.
