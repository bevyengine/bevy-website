`PartialReflect::clone_value` is being deprecated. Instead, use `PartialReflect::to_dynamic` if wanting to create a new dynamic instance of the reflected value. Alternatively, use `PartialReflect::reflect_clone` to attempt to create a true clone of the underlying value.

Similarly, the following methods have been deprecated and should be replaced with these alternatives:

- `Array::clone_dynamic` → `Array::to_dynamic_array`
- `Enum::clone_dynamic` → `Enum::to_dynamic_enum`
- `List::clone_dynamic` → `List::to_dynamic_list`
- `Map::clone_dynamic` → `Map::to_dynamic_map`
- `Set::clone_dynamic` → `Set::to_dynamic_set`
- `Struct::clone_dynamic` → `Struct::to_dynamic_struct`
- `Tuple::clone_dynamic` → `Tuple::to_dynamic_tuple`
- `TupleStruct::clone_dynamic` → `TupleStruct::to_dynamic_tuple_struct`
