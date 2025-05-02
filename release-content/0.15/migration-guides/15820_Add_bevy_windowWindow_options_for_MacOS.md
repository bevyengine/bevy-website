`bevy_window::Window` now has extra fields for configuring MacOS window settings:

```rs
    pub movable_by_window_background: bool,
    pub fullsize_content_view: bool,
    pub has_shadow: bool,
    pub titlebar_shown: bool,
    pub titlebar_transparent: bool,
    pub titlebar_show_title: bool,
    pub titlebar_show_buttons: bool,
```

Using `Window::default` keeps the same behaviour as before.
