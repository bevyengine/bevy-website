Bevy Remote Protocol's `bevy/query` request now skips missing or invalid components by default instead of returning an error. This can be configured with `BrpQueryParams`'s new `strict` boolean field.

If you wish `bevy/query` to return to its previous behavior of erroring on missing / invalid components, set `"strict": true`:

```json
{
    "method": "bevy/query",
    "id": 0,
    "params": {
        "data": {
            "components": ["foo::bar::MyComponent"]
        },
        // Error if `foo::bar::MyComponent` doesn't exist.
        "strict": true
    }
}
```
