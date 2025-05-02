The order of hooks and observers for `on_replace()` and `on_remove()` has been swapped, so now observers are run before hooks. As hooks are more primitive, they are designated as the first and last thing run when a component is added and removed. The total order for component removal can now be seen in the following table:

|0.15|0.16|
|-|-|
|`on_replace()` hook|`on_replace()` observer|
|`on_replace()` observer|`on_replace()` hook|
|`on_remove()` hook|`on_remove()` observer|
|`on_remove()` observer|`on_remove()` hook|
