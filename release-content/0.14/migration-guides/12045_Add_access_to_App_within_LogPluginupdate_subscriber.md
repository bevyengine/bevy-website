`LogPlugin::update_subscriber` now has a `&mut App` parameter. If you don’t need access to `App`, you can ignore the parameter with an underscore (`_`).

```diff,rust
- fn update_subscriber(subscriber: BoxedSubscriber) -> BoxedSubscriber {
+ fn update_subscriber(_: &mut App, subscriber: BoxedSubscriber) -> BoxedSubscriber {
      Box::new(subscriber.with(CustomLayer))
  }
```
