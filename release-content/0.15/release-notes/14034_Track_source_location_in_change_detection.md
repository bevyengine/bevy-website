<!-- Track source location in change detection -->
<!-- https://github.com/bevyengine/bevy/pull/14034 -->

Keeping track of when and where values are changed can be tricky in any complex program, and Bevy applications are no exception.
Thankfully, our unified ECS-backed data model makes it easy for us to add debugging tools that work right out of the box, with no user configuration required.

When you turn on the `track_change_detection` feature flag, Bevy will record the exact line of code that mutated your component or resource side-by-side with the value.
While this is obviously too expensive for ordinary use, it's a godsend for debugging tricky issues, as the value can be logged or read directly via the debugger of your choice.

As shown in the [`change_detection` example],
simply turn on the feature and call `my_component.changed_by()` on any [`Ref`], [`Mut`], [`Res`] or [`ResMut`] smart pointer to get a helpful string pointing you straight to the last line of code that mutated your data!

[`change_detection` example]: https://github.com/bevyengine/bevy/blob/main/examples/ecs/change_detection.rs
[`Ref`]: https://docs.rs/bevy/0.15.0/bevy/ecs/change_detection/struct.Ref.html
[`Mut`]: https://docs.rs/bevy/0.15.0/bevy/ecs/change_detection/struct.Mut.html
[`Res`]: https://docs.rs/bevy/0.15.0/bevy/ecs/change_detection/struct.Res.html
[`ResMut`]: https://docs.rs/bevy/0.15.0/bevy/ecs/change_detection/struct.ResMut.html
