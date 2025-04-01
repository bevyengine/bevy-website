`Handle::weak_from_u128()` has been deprecated in favor of the new `weak_handle!` macro, which takes a UUID as a string instead of a `u128`. `weak_handle!` is preferred because it both makes the string form of the UUID visible and it verifies that the UUID is compliant with UUIDv4.

```rust
// 0.15
const SHADER: Handle<Shader> = Handle::weak_from_u128(314685653797097581405914117016993910609);

// 0.16
const SHADER: Handle<Shader> = weak_handle!("1347c9b7-c46a-48e7-b7b8-023a354b7cac");
```
