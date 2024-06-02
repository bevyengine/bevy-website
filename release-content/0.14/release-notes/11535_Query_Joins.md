Queries can now be combined, returning the data for entities that are contained in both queries.

```rust
fn socratic_deduction_the_hard_way(mut all_men: Query<&Man>, mut all_mortals: Query<&Mortal>){
    let n_men = all_men.iter().len();
    let n_mortals = all_mortals.iter().len();

    // This check is necessary but not sufficient
    assert_eq!(n_men, n_mortals);

    // Only entities found in *both* queries will be found in this query
    let men_and_mortals: QueryLens<&Man, &Mortal> = all_men.join(all_mortals);
    let n_mortal_men = men_and_mortals.iter().len();

    // By contrast, this check is both necessary and sufficient!
    assert_eq!(n_men, n_mortal_men);
}
```

If you're familiar with database terminology, this is an ["inner join"](https://www.w3schools.com/sql/sql_join.asp).
Other types of query joins are on the metaphorical table: please consider taking a crack at the [follow-up issue](https://github.com/bevyengine/bevy/issues/13633).
