Style has a new field `OverflowClipMargin`.  It allows users to set the visible area for clipped content when using overflow-clip, -hidden, or -scroll and expand it with a margin.

There are three associated constructor functions `content_box`, `padding_box` and `border_box`:

- `content_box`: elements painted outside of the content box area (the innermost part of the node excluding the padding and border) of the node are clipped. This is the new default behaviour.
- `padding_box`: elements painted outside outside of the padding area of the node are clipped. 
- `border_box`:  elements painted outside of the bounds of the node are clipped. This matches the behaviour from Bevy 0.14.

There is also a `with_margin` method that increases the size of the visible area by the given number in logical pixels, negative margin values are clamped to zero.

`OverflowClipMargin` is ignored unless overflow-clip, -hidden or -scroll is also set on at least one axis of the UI node.
