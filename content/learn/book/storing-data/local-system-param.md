+++
title = "Local System Parameters"
insert_anchor_links = "right"
[extra]
weight = 99
status = 'hidden'
+++

Sometimes you may need to preserve some state between executions of a [system].
For this, we have the [`Local<T>`] system parameter.

Any type that implements `Default` can be stored in a local. `Default` is required so that the value can be initialized before the system runs. If your type does not implement `Default`, you can use `Option<T>` instead.

[system]: /learn/book/control-flow/systems
[`Local<T>`]: https://docs.rs/bevy/0.16.0/bevy/ecs/system/struct.Local.html
