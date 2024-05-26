Users of `WorldQuery::get_state` or `transmute`, `transmute_filtered`, `join` and `join_filtered` methods on `QueryState` now need to pass `&Components` instead of `&World`. 
`&Components` can be trivially obtained from either `components` method on `&World` or `UnsafeWorldCell`.
For implementors of `WorldQuery::get_state` that were accessing more than the `Components` inside `&World` and its methods, this is no longer allowed.
