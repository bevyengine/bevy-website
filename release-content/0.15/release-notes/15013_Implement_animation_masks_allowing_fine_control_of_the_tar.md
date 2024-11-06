<!-- Implement animation masks, allowing fine control of the targets that animations affect. -->
<!-- https://github.com/bevyengine/bevy/pull/15013 -->

Animations now support masking out animation targets (joints).
This is implemented at the level of animation blend graphs (`AnimationGraph`)
and can be used to play different animations on separate parts of the
same model without interfering with one another. This may come up, for example, 
if you need a character to play separate animations on its upper and lower body.

