`bevy_ui` now supports basic text shadows out of the box.
Simply add the [`TextShadow`] component to any [`Text`] entity
and we'll draw a nice simple offset version of your glyph in the color of your choice.

TODO: generate and add an image of text shadows in action

While this is a simple first implementation (no blurring), and only supports `bevy_ui` text (sorry [`Text2d`]),
we hope that this helps make Bevy a better fit for stylized game UIs.

[`TextShadow`]: https://docs.rs/bevy/0.16/bevy/prelude/struct.TextShadow.html
[`Text`]: https://docs.rs/bevy/0.16/bevy/prelude/struct.Text.html
[`Text2d`]: https://docs.rs/bevy/0.16/bevy/prelude/struct.Text2d.html
