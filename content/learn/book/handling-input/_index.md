+++
title = "Handling Input"
template = "docs.html"
insert_anchor_links = "right"
[extra]
weight = 14
status = 'hidden'
+++

By definition, games are interactive.
Players will perform some action or reaction, and the game will react and change in return.
When it comes to video games specifically, we need to receive input from the player through some kind of device.
In Bevy, we can use input from gamepads, keyboards, mice, or even touch screens in our games and applications.
The process of accessing and using this data is detailed on the [Using Input page](/learn/book/handling-input/using-input), which is where we'd recommend you start.

While input data from every device will follow the same process, you should also read the page of each device relevant to your project to see how their data is uniquely handled:

- [Keyboard Input](/learn/book/handling-input/keyboard-input)
- [Mouse Input](/learn/book/handling-input/mouse-input)
- [Gamepad Input](/learn/book/handling-input/gamepad-input)
- [Touch Input](/learn/book/handling-input/touch-input)

When you use larger groupings of features (like profiles and collections), all of these devices are enabled in your game by default.
Unfortunately, due to the way that Cargo features work, we can't specifically disable a device.
We do have the option to manually enable the [feature flag] for a specific device, but then you will be required to manually enable the feature flags of every other part of the engine that you want to use.
If you believe that this tradeoff is worth it for your project, please see the [Selective Feature Use section] on the Compiling Less Code page for more details.

[Selective Feature Use section]: /learn/book/releasing-projects/compiling-less-code/#more-selective-feature-use

[feature flag]: https://docs.rs/bevy/latest/bevy/index.html#feature-list
