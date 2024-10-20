This API hasn’t shipped yet, so I didn’t bother with a deprecation. However, for any crates tracking main the changes are as follows: 

Previous api:

```rs
commands.insert(PointerBundle::new(PointerId::Mouse));
commands.insert(PointerBundle::new(PointerId::Mouse).with_location(location));
```

New api:

```rs
commands.insert(PointerId::Mouse);
commands.insert((PointerId::Mouse, PointerLocation::new(location)));
```
