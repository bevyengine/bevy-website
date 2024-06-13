External types are no longer registered automatically unless they are used by
other Bevy types. If you were depending on `std`, `glam` or similar types being
in the type registry you need to manually register them with `.register_type()`.
