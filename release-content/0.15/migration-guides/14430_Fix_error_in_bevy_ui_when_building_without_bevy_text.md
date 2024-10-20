**This is not a breaking change for users migrating from 0.14, since `MeasureArgs` did not exist then.**

When the `bevy_text` feature is disabled for `bevy_ui`, the type of the `MeasureArgs::font_system` field is now a `PhantomData` instead of being removed entirely. This is in order to keep the lifetime parameter, even though it is unused without text being enabled.
