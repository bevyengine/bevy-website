<!-- Implement additive blending for animation graphs. -->
<!-- https://github.com/bevyengine/bevy/pull/15631 -->

Bevy's animation graphs (`AnimationGraph`), which are used to combine
simultaneously playing animations, now support *additive blending*.

Additive blending is a technique which allows separately authored animations
to be applied on top of an arbitrary base animation. For instance, an animation
in which a character swings a weapon may be applied additively on top of a 
walking or running animation.

Within an animation graph itself, this is accomplished by using `Add` nodes.
The situation above might be described with an animation graph that looks
something like this (weights omitted):

```
┌─────┐                              
│Walk ┼─┐                            
└─────┘ │ ┌─────┐                    
        ┼─┼Blend┼─┐                  
┌─────┐ │ └─────┘ │ ┌─────┐   ┌─────┐
│Run  ┼─┘         ┼─┤Add  ┼───┼Root │
└─────┘   ┌─────┐ │ └─────┘   └─────┘
          │Swing┼─┘                  
          └─────┘                    
```

The `Add` node functions by taking its first input (here, a blend of the 'Walk'
and 'Run' clips) as-is and then applying the subsequent inputs additively on
top of it. In code, the graph might be constructed as follows:

```rust
let mut animation_graph = AnimationGraph::new();

// Attach an `Add` node to the root.
let add_node = animation_graph.add_additive_blend(1.0, animation_graph.root);

// Add the `Blend` node and the additive clip as children; the `Blend` result
// will be used as the base because it is listed first.
let blend_node = animation_graph.add_blend(1.0, add_node);
animation_graph.add_clip(swing_clip_handle, 1.0, add_node);

// Finally, blend the 'Walk' and 'Run' clips to use as a base.
animation_graph.add_clip(walk_clip_handle, 0.5, blend_node);
animation_graph.add_clip(run_clip_handle, 0.5, blend_node);
```
