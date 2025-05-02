Spawning hierarchies in Bevy has historically been a bit cumbersome:

```rust
commands
    .spawn(Player)
    .with_children(|p| {
        p.spawn(RightHand).with_children(|p| {
            p.spawn(Glove);
            p.spawn(Sword);
        });
        p.spawn(LeftHand).with_children(|p| {
            p.spawn(Glove);
            p.spawn(Shield);
        });
    });
```

We have big plans to improve Bevy's spawning experience with our [Next Generation Scene / UI System](https://github.com/bevyengine/bevy/discussions/14437) (BSN). An important stepping stone on that path is making it possible to express hierarchies directly via data, rather than using builder methods. The addition of Relationships further increases the value of building such a system, as _all_ relationships can benefit from it.

In **Bevy 0.16** we have vastly improved the ergonomics of spawning hierarchies:

```rust
commands.spawn((
    Player,
    children![
        (RightHand, children![Glove, Sword]),
        (LeftHand, children![Glove, Shield]),
    ],
));
```

This builds on the existing Bundle API by adding support for "bundle effects", which are applied immediately after a Bundle is inserted. Notably, this enables developers to define functions that return a hierarchy like this:

```rust
fn player(name: &str) -> impl Bundle {
    (
        Player,
        Name::new(name),
        children![
            (RightHand, children![Glove, Sword]),
            (LeftHand, children![Glove, Shield]),
        ]
    )
}

// later in your app
commands.spawn(player("Bob"));
```

In most cases the `children!` macro should be preferred for ergonomics reasons. It expands to the following API:

```rust
commands.spawn((
    Player,
    Children::spawn((
        Spawn((
            RightHand,
            Children::spawn((Spawn(Glove), Spawn(Sword))),
        )),
        Spawn((
            LeftHand,
            Children::spawn((Spawn(Glove), Spawn(Shield))),
        )),
    )),
));
```

There are a number of spawn wrapper variants, which provide additional flexibility:

```rust
world.spawn((
    Name::new("Root"),
    Children::spawn((
        Spawn(Name::new("Child1")),   
        SpawnIter(["Child2", "Child3"].into_iter().map(Name::new)),
        SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn(Name::new("Child4"));
            parent.spawn(Name::new("Child5"));
        })
    )),
))
```

Notably, this API works for _all_ relationship types. For example, you could spawn a `Likes` / `LikedBy` relationship hierarchy (as defined in the relationships section above) like this:

```rust
world.spawn((
    Name::new("Monica"),
    LikedBy::spawn((
        Spawn(Name::new("Naomi")),
        Spawn(Name::new("Dwight")),
    ))
))
```

There is also a `related!` macro, which does the same thing as `children!`, but for any relationship type:

```rust
world.spawn((
    Name::new("Monica"),
    related!(LikedBy[
        Name::new("Naomi"),
        Name::new("Dwight"),
    ]),
))
```

This API also allows us to optimize hierarchy construction time by cutting down on re-allocations, as we can generally (with the exception of cases like `SpawnWith`) statically determine how many related entities an entity will have and preallocate space for them in the `RelationshipTarget` component (ex: `Children`).
