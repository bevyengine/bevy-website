`Table` now uses `ThinColumn` instead of `Column`. That means that methods that previously returned `Column`, will now return `ThinColumn` instead.

`ThinColumn` has a much more limited and low-level API, but you can still achieve the same things in `ThinColumn` as you did in `Column`. For example, instead of calling `Column::get_added_tick`, you’d call `ThinColumn::get_added_ticks_slice` and index it to get the specific added tick.
