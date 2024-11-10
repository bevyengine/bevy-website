Certain type info structs now only return their item types as `Type` instead of exposing direct methods on them.

The following methods have been removed:

- `ArrayInfo::item_type_path_table`
- `ArrayInfo::item_type_id`
- `ArrayInfo::item_is`
- `ListInfo::item_type_path_table`
- `ListInfo::item_type_id`
- `ListInfo::item_is`
- `SetInfo::value_type_path_table`
- `SetInfo::value_type_id`
- `SetInfo::value_is`
- `MapInfo::key_type_path_table`
- `MapInfo::key_type_id`
- `MapInfo::key_is`
- `MapInfo::value_type_path_table`
- `MapInfo::value_type_id`
- `MapInfo::value_is`

Instead, access the `Type` directly using one of the new methods:

- `ArrayInfo::item_ty`
- `ListInfo::item_ty`
- `SetInfo::value_ty`
- `MapInfo::key_ty`
- `MapInfo::value_ty`

For example:

```rust
// BEFORE
let type_id = array_info.item_type_id();

// AFTER
let type_id = array_info.item_ty().id();
```
