The `Debug` and `Display` impls for `Entity` now return `PLACEHOLDER` for the `Entity::PLACEHOLDER` constant. If you had any code relying on these values, you may need to account for this change.
