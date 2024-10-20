<!-- Add custom cursors -->
<!-- https://github.com/bevyengine/bevy/pull/14284 -->

Previously Bevy's native window cursors supported only a fixed set of built-in OS cursors. Now they also support arbitrary images as "custom cursors". Custom cursors still use native facilities of the OS, which allows them to stay perfectly responsive even when the frame rate of the application drops.
