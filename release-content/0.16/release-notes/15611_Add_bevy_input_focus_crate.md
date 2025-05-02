The concept of "input focus" is important for accessibility. Visually or motor-impaired users may have trouble using mice; with focus navigation they can control the UI using only the keyboard. This not only helps disabled users, but power users as well.

The same general idea can be applied to game controllers and other input devices. This is often seen in games where the number of controls on the screen exceeds the number of buttons on the gamepad, or for complex "inventory" pages with a grid of items.

The `bevy_a11y` crate has had a `Focus` resource for a long time, but it was situated in a way that made it hard to use. This has been replaced by an `InputFocus` resource which lives in a new crate, `bevy_input_focus`, and now includes a bunch of helper functions that make it easier to implement widgets that are focus-aware.

This new crate also supports "pluggable focus navigation strategies", of which there are currently two: there's a "tab navigation" strategy, which implements the traditional desktop sequential navigation using the TAB key, and a more "console-like" 2D navigation strategy that uses a hybrid of spatial searching and explicit navigation links.
