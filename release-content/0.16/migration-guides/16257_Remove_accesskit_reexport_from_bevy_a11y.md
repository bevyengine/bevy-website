```diff
# main.rs
--    use bevy_a11y::{
--        accesskit::{Node, Rect, Role},
--        AccessibilityNode,
--    };
++    use bevy_a11y::AccessibilityNode;
++    use accesskit::{Node, Rect, Role};

# Cargo.toml
++    accesskit = "0.17"
```

- Users will need to add `accesskit = "0.17"` to the dependencies section of their `Cargo.toml` file and update their `accesskit` use statements to come directly from the external crate instead of `bevy_a11y`.
- Make sure to keep the versions of `accesskit` aligned with the versions Bevy uses.
