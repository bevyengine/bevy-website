- Reflection now will serialize and deserialize tuple struct with single field as newtype struct. Consider this code.

```rs
#[derive(Reflect, Serialize)]
struct Test(usize);
let reflect = Test(3);
let serializer = TypedReflectSerializer::new(reflect.as_partial_reflect(), &registry);
return serde_json::to_string(&serializer)
```

Old behavior will return `["3"]`. New behavior will return `"3"`. If you were relying on old behavior you need to update your logic. Especially with `serde_json`. `ron` doesn’t affect from this.
