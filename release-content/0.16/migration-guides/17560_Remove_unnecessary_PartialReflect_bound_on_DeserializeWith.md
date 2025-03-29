`DeserializeWithRegistry` types are no longer guaranteed to be `PartialReflect` as well. If you were relying on this type bound, you should add it to your own bounds manually.

```diff
- impl<T: DeserializeWithRegistry> Foo for T { .. }
+ impl<T: DeserializeWithRegistry + PartialReflect> Foo for T { .. }
```
