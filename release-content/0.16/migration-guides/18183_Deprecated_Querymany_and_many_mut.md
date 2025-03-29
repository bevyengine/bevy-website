`Query::many` and `Query::many_mut` have been deprecated to reduce panics and API duplication. Use `Query::get_many` and `Query::get_many_mut` instead, and handle the `Result`.
