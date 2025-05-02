Due to improvements in **Bevy 0.16**'s error handling capabilities, `Query::many()` and `Query::many_mut()` have been deprecated in favor of their non-panicking variants: `Query::get_many()` and `Query::get_many_mut()`.

```rust
#[derive(Resource)]
struct Player1(Entity);

#[derive(Resource)]
struct Player2(Entity);

// 0.15
fn my_system(player1: Res<Player1>, player2: Res<Player2>, query: Query<&Transform>) {
    let [transform1, transform2] = query.many([player1.0, player2.0]);

    // ...
}

// 0.16
// Make the system return a `Result`, which is automatically imported in Bevy's prelude.
fn my_system(player1: Res<Player1>, player2: Res<Player2>, query: Query<&Transform>) -> Result {
    // Use `get_many()` and the `?` operator to return early on an error.
    let [transform1, transform2] = query.get_many([player1.0, player2.0])?;

    // ...

    Ok(())
}
```

Please note that `Query::get_many()` is very similar to `Query::get()`. To increase the consistency between the two methods, the name `get_many()` was kept over plain `many()`. Although in 0.15 `Query::many()` seemed similar to `Query::single()` due to their naming, they are quite distinct. This change is meant to reinforce this distinction.
