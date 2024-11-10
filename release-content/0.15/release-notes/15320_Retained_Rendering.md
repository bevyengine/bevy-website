<!-- Retained Rendering -->
<!-- https://github.com/bevyengine/bevy/pull/14449 Original PR -->
<!-- https://github.com/bevyengine/bevy/pull/15320 Adopted PR -->
<!-- https://github.com/bevyengine/bevy/pull/15582 Synchronized Removed Components -->
<!-- https://github.com/bevyengine/bevy/pull/15756 Type Safe Retained Render World -->

We here at Bevy ECS Incorporated are big supporters of Big Entity. 'Entifying' the internals (queries as entities, components as entities, assets as entities) grows the ~~economy~~ ECS and we can use all sorts of neat things like observers on the things we just transmorphed into entities. More entities is more better, but we here at Bevy ECS Enterprise, have a problem.

The folks over at Bevy Render Corp have been doing some wacky things with our beloved ECS. Mainly, spefically, in this instance, we are concerned with this rather odd thing called a 'render world'. Everything in an ECS (entities, components and even systems) lives in a world. It's a big box we keep all our toys in. This box is as large as your main memory and should be good enough for just about everything. Yet, Bevy Render Associates have decided long ago that they wanted a second box. This second world, henceforth named the render world, is meant to do rendering stuff, so the first word (main world) can do simulation stuff. In the main world you do things like enemy AI, while the render world does things like create rendering passes (whatever those are). These worlds are seperate for concurrency reasons. It's something called pipelined rendering and you can look that up if you want to.

Anyway, in order to make sure that the render world had the correct data it needed to extract said data from the main world, once at the beginning of every frame. During this extraction step, both worlds are only touched by a single thread in order to make sure nothing weird happens. Afterwards they can run of independently again, untill the next frame comes.

What does this have to do with entities, you may ask? I'll get to it.

This extraction was handled in a multitude of ways (assets are stored in some hashmap I believe and don't use the ECS), but we are concerned with entities. During the extraction step, relevant entities were copied over from the main world to the render world. In order to make sure that removed entities from the main world didn't linger in the render world, we did something extremely clever.

We remove every entity from the render world at the end of the frame and to just re-extract everything next frame (coincidentally, this also takes care of change detection).

Does this sound wastefull?

Yes, yes it does. But given the circumstances this was actually mostly fine in terms of performance.

There are two main problems:
1. The old implementation had a 1-to-1 mapping for entity ids between the main and render worlds. Entity 5 in the main world was also entity 5 in the render world.
2. We nuke all entities at the end of every frame.

The first problem is annoying on both a practical and philosophical level and it's the main one a retained rendering world tries to solve. We at Bevy ECS Regional want to keep entity ids opaque, i.e. you shouldn't have to worry about what the numbers behind them mean. We don't want to guarentee that ids stay stable across frames (and we didn't), but for the rendering world we had to. The second problem, on the other hand, is more concrete. If we keep entifying ECS internals, nuking all entities means that we would practically dissassemble the render world at runtime.

So this had to be fixed. And fix it we did. In the two Retained Rendering PRs (leave links) we stopped nuking the world and instituted a mapping between the main and rendering world. Entity 5 in the main world could correspond to entity 7 in the render world and this will just work. More-over we added a sync step (before the extraction step) that does the bookkeeping needed to keep this correspondance correct accross frames and changes.

This PR (merged by yours truly) created a lot of bugs, because this is one of those fun cross-cutting concerns where lots of different parts of the codebase need to extract stuff from the main world. The most major fixes include synchronized removed components and type safe retained render world (link both of these).

The most straight-forward of these is the syncrhonized removed components PR. While we have been talking about entities for the last many paragraphs, when extracting data, you are really mostly only extracting components (which is data). Now imagine if you were to remove a component of a synced entity?

Are you done? Did you imagine it?

That's right, under the initial implementation this would not remove the component from the render world. Which is wrong. This PR fixes it by implementing a SyncComponentPlugin, which I'll probably get to later.

The other PR was a bunch'o bugfixes related to the fact that a lot of stuff in the render world held an 'Entity' and that used to be a fairly straightforward idea. The one-to-one mapping made it so that it didn't matter what you were referring to: it's all the same anyway. In the new world, a lot more clarification was needed in order to keep straight what exactly an 'Entity' is referring to: the main one or the rendering one?

Man, I've made an enormous mess haven't I?

In any case. What does this mean for you, the consumer. The answer is either a lot or very little. If you didn't bother with rendering internals before don't know what a rendering pass is, this has quite certainly no effect on you. If you however have touched these things, I apologize. There are migration guides somewhere, I presume.

But let's not linger on the past, but look to the future. Mainly, what does this allow us to do? Well, to start of with, we were able to deprecate `batch_insert_or_spawn` and `get_or_spawn` which has allowed us to shrink our API surface by 0.1%! Moreover, we can now proceed with queries as entities, which is the next step on the (very long) road to relations.

Exciting stuff. Truly legendary.
