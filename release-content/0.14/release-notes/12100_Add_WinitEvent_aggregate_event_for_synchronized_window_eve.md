When handling inputs, the exact ordering of the events received is often very significant, even when the events are not the same type!
Consider a simple drag-and-drop operation. When, exactly, did the user release the mouse button relative to the many tiny movements that they performed?
Getting these details right goes a long way to a responsive, precise user experience.

We now expose the blanket [`WinitEvent`](https://docs.rs/bevy/0.14/bevy/winit/enum.WinitEvent.html) event stream, in addition to the existing separated event streams, which can be read and matched on directly whenever these problems arise.
