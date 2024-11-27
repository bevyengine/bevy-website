<!-- Implement animation masks, allowing fine control of the targets that animations affect. -->
<!-- https://github.com/bevyengine/bevy/pull/15013 -->

Animations now support masking out animation targets (joints).
This is implemented at the level of animation blend graphs (`AnimationGraph`)
and can be used to play different animations on separate parts of the
same model without interfering with one another. For example, you can play separate animations on a character's upper and lower body.

In this video, the fox's head and legs are playing two separate animations, thanks to animation masks:
<video controls><source src="masked-animation.mp4" type="video/mp4"/></video>
