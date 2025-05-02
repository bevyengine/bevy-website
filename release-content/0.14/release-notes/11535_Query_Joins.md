ECS Queries can now be combined, returning the data for entities that are contained in both queries.

```rust
fn helper_function(a: &mut Query<&A>, b: &mut Query<&B>){    
    let a_and_b: QueryLens<(Entity, &A, &B)> = a.join(b);
    assert!(a_and_b.iter().len() <= a.len());
    assert!(a_and_b.iter().len() <= b.len());
}
```

In most cases, you should continue to simply add more parameters to your original query. `Query<&A, &B>` will generally be clearer than joining them later.
But when a complex system or helper function backs you into a corner, query joins are there if you need them.

If you're familiar with database terminology, this is an ["inner join"](https://www.w3schools.com/sql/sql_join.asp).
Other types of query joins are being considered. Maybe you could take a crack at the [follow-up issue](https://github.com/bevyengine/bevy/issues/13633)?
