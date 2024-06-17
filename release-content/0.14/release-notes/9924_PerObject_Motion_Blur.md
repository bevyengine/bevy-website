We've added a post-processing effect that blurs fast-moving objects in the direction of motion.
Our implementation uses motion vectors, which means it works with Bevy's built in PBR materials, skinned meshes, or anything else that writes motion vectors and depth.
The effect is used to convey high speed motion, which can otherwise look like flickering or teleporting when the image is perfectly sharp.

Blur scales with the motion of objects relative to the camera.
If the camera is tracking a fast moving object, like a vehicle, the vehicle will remain sharp, while stationary objects will be blurred.
Conversely, if the camera is pointing at a stationary object, and a fast moving vehicle moves through the frame, only the fast moving object will be blurred.

The implementation is configured with [camera shutter angle](https://en.wikipedia.org/wiki/Rotary_disc_shutter), which corresponds to how long the virtual shutter is open during a frame.
In practice, this means the effect scales with framerate, so users running at high refresh rates aren't subjected to over-blurring.

TODO: add image.

You can enable motion blur by adding [`MotionBlurBundle`](https://dev-docs.bevyengine.org/bevy/core_pipeline/motion_blur/struct.MotionBlurBundle.html) to your camera entity, as shown in our [`motion blur` example](https://github.com/bevyengine/bevy/blob/main/examples/3d/motion_blur.rs).
