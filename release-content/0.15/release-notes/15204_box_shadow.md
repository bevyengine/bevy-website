<!-- box shadow -->
<!-- https://github.com/bevyengine/bevy/pull/15204 -->

![A demonstration of Bevy's box shadows. There are 12 shapes on a light blue background, and the border radius, aspect ratio and softness of the shadows is varied for each of them. The shadows cause the buttons to appear to hover above the page.](box_shadow.png)

Bevy UI now supports box shadows! Box shadows can be used to achieve particular artistic effects, such as creating a sense of depth to cue to users that an element is interactable.

By adding the new [`BoxShadow`] component to any [`Node`] entity, you can draw a pretty shadow behind your widgets.

```rust
#[derive(Component)]
pub struct BoxShadow {
    /// The shadow's color
    pub color: Color,
    /// Horizontal offset
    pub x_offset: Val,
    /// Vertical offset
    pub y_offset: Val,
    /// How much the shadow should spread outward.
    ///
    /// Negative values will make the shadow shrink inwards.
    /// Percentage values are based on the width of the UI node.
    pub spread_radius: Val,
    /// Blurriness of the shadow
    pub blur_radius: Val,
}
```

We have plans for future improvements: enable using shadows to create an inset / sunken look, and adding shadow support for images and text. If those possibilities excite you, get involved! We love helping [new contributors] land the features they care about.

[`BoxShadow`]: https://docs.rs/bevy/0.15/bevy/prelude/struct.BoxShadow.html
[`Node`]: https://docs.rs/bevy/0.15/bevy/prelude/struct.Node.html
[new contributors]: https://bevy.org/learn/contribute/introduction/