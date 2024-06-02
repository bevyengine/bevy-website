[`taffy`](https://crates.io/crates/taffy), the UI layout crate that we rely on, has had a major update.
In addition to various bug fixes, this brings two new pieces of functionality to users of `bevy_ui`:

1. `Display::Block`: use the [CSS block layout model](https://www.w3schools.com/css/css_display_visibility.asp) to layout UI nodes. Elements will start on a new line, and take up the whole width. Great for in-line images!
2. `Overflow::Hidden`: if elements overflow their container, they will be completely hidden, rather than shown or clipped.
