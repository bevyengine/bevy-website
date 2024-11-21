<!-- box shadow -->
<!-- https://github.com/bevyengine/bevy/pull/15204 -->

![A demonstration of Bevy's box shadows. There are 12 white shapes on a salmon pink background, and the border radius, aspect ratio and softness of the shadows is varied for each of them. The shadows cause the buttons to appear to hover above the page.](box_shadow.png)

Bevy's UI now supports box shadows!
When designing user interfaces, creating a sense of depth can provide cues to users about which elements are interactable.
One of the key features of elements with depth (that are not just flat on the metaphorical page) is that they cast shadows.

UI shadows aren't intended to be realistic: no raytracing here!
Instead, we care about offering a pretty effect with maximum artistic control at a minimum runtime cost.

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

For now, these shadows can't be used to create an inset, sunken look and shadows are only rendered for plain nodes (aka divs or containers), not for images or text.
If those possibilities excite you, get involved!
We love helping [new contributors] land the features they care about.

[`BoxShadow`]: https://docs.rs/bevy/0.15/bevy/prelude/struct.BoxShadow.html
[`Node`]: https://docs.rs/bevy/0.15/bevy/prelude/struct.Node.html
[new contributors]: https://bevyengine.org/learn/contribute/introduction/