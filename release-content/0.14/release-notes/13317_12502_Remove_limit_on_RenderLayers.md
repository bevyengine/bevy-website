<!-- #12502 Remove limit on RenderLayers. -->
<!-- https://github.com/bevyengine/bevy/pull/13317 -->

In Bevy you are able to assign entities spawned to different layers.

You can make 3D models, meshes, lights, etc. all have their own if you wish.

You can tell cameras which layers to render, and only objects which are part of those layers get rendered to that camera's target.

There is no longer any practical limit to how many layers you can define!
Before Bevy 0.14 the membership was defined by a bitmask which had limited slots available. We've made sure to keep the common case fast, but now use a growable mask that will allocate space for additional layers as needed. Remember, there's still a cost to check visibility per layer, but this allows for more dynamic uses where layers can be created on demand without worrying about going over a limit.

