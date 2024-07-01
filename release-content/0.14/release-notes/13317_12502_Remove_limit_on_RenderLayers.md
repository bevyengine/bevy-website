<!-- #12502 Remove limit on RenderLayers. -->
<!-- https://github.com/bevyengine/bevy/pull/13317 -->

Render layers are used to quickly toggle the visibility of sets of objects, and control which objects can be seen by which cameras.
This can be useful for things like debug views, gear preview screens, toggleable diegetic UI and so on.


Before Bevy 0.14 the membership was defined by a bitmask which had limited slots available.
Now, there is no longer any practical limit to how many layers you can define, which is particularly helpful for creative coding applications like [nannou](https://nannou.cc/)! 
We've made sure to keep the common case fast, but now use a growable mask that will allocate space for additional layers as needed. Remember, there's still a cost to check visibility per layer, but this allows for more dynamic uses where layers can be created on demand without worrying about going over a limit.

