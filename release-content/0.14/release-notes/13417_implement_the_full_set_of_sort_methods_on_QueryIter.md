Currently, query iteration order is not guaranteed. If a user wishes to handle their query items in a certain order, sorting is required.
In 0.13 this could look like this:
```rust
#[derive(Component, Copy, Clone, Deref)]
pub struct Attack(pub usize)

fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense, &Rarity)>) {
    let mut enemies: Vec<_> = enemies.iter().collect();
    enemies.sort_by_key(|(_, atk, ..)| *atk)
    for enemy in enemies {
        work_with(enemy)
    }
}
```
This can get rather unwieldy and repetitive when sorting within multiple systems.
Even when the sort itself is always the same, abstracting it away is unreasonably difficult when used with differing [`Query`] types!
As a solution, the sort methods were implemented as an adapter on the [`QueryIter`] types, which turns the prior example into this:
```rust
// To be used as a sort key, `Attack` now implements Ord.
#[derive(Component, Copy, Clone, Deref, PartialEq, Eq, PartialOrd, Ord)]
pub struct Attack(pub usize)

fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense, &Rarity)>) {
    for enemy in enemies.iter().sort::<&Attack>() {
        work_with(enemy)
    }
}
```
Here, the user specifies the [`Component`] from their query they wish to sort with as a generic parameter to [`sort`].
This call can be thought of as constructing a [lens](https://dev-docs.bevyengine.org/bevy/ecs/prelude/struct.Query.html#method.transmute_lens) of the original query, on which the sort is actually performed. The result is then internally used to return the original query items in sorted order. 
In this case, [`sort`] requires its argument to be fully [`Ord`], like its [`slice::sort`] counterpart. 
If the functionality of [`sort`] does not suffice, all 5 other sort methods from the std library on [`slice`] are now available.

The generic [`lens`] argument is specified the same way as in [`Query::transmute_lens`]. Any filters are inherited from the source query, and do not need to be specified by the user.
This has some nice additional effects, such as allowing this use case:
```rust
fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense, &Rarity)>) {
    for enemy in enemies.iter().sort::<Entity>() {
        work_with(enemy)
    }
}
```
We can sort by [`Entity`] without including it in the original query!

These sort methods are fully generic over mutability, so can be used on both [`Query::iter`] and [`Query::iter_mut`]! The sorts return iterators themselves, so can be used with further iterator adapter if desired!

The sorted iterator type [`QuerySortedIter`] implements [`DoubleEndedIterator`] while the initial [`QueryIter`] does not. As a consequence, an empty sort can be used to get a more powerful iterator type:
```rust
fn handle_enemies(enemies: Query<(&Health, &Attack, &Defense, &Rarity)>) {
    // A reversible query iterator!
    enemies.iter_mut().sort::<()>().rev();
}
```
Furthermore, these query iterator sorts offer a workaround for a restriction on slice sorts:
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
In current Rust, one can not return references/non-`Copy` data from the key extraction closure in [`slice::sort_by_key`]/[`slice::sort_by_key`]. 
This can become a headache when using non-`Copy` [`Component`]s.
The `sort`/`sort_by` methods can work around this by having the lensing be the "key extraction", removing the need for such a closure.

Keep in mind that the lensing does add some overhead, so on average does not perform equally to a manual sort. However, this *strongly* depends on workload, so best test it yourself if relevant!
The sorts themselves are not yet cached between system runs, and these methods only sort iterators, not underlying query storage.

Note that query iteration might happen in a deterministic order for now, but that may change any time over future releases.