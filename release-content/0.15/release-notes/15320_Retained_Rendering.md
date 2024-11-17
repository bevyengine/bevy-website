<!-- Retained Rendering -->
<!-- https://github.com/bevyengine/bevy/pull/14449 Original PR -->
<!-- https://github.com/bevyengine/bevy/pull/15320 Adopted PR -->
<!-- https://github.com/bevyengine/bevy/pull/15582 Synchronized Removed Components -->
<!-- https://github.com/bevyengine/bevy/pull/15756 Type Safe Retained Render World -->
For awhile now, Bevy [has had a "parallel pipelined renderer"](/news/bevy-0-6/#pipelined-rendering-extract-prepare-queue-render). To enable this, we added a Render World, in addition to the Main World (a `World` holds ECS data like Entities, Components, and Resources). The Main World is the source of truth for app logic. While the Render World is rendering the current frame, the Main World can be simulating the next frame. There is a brief "extract step", where we synchronize the two and copy relevant data from the Main World to the Render World.

In previous versions of Bevy, we employed an "immediate mode" approach to Main World -> Render World synchronization: we fully cleared the Render World entities every frame. This accomplished a couple of things:

1. It allowed us to ensure entity IDs "lined up", allowing us to reuse entities in both places.
2. It prevented us from needing to solve the "desync problem". By clearing every frame and re-syncing, we ensure the two Worlds are always perfectly in sync.

There was also precedent for the "immediate mode" pipelined rendering approach: Bungie's Destiny renderer uses it to great effect!

However we learned pretty quickly that clearing every frame had major downsides:

1. The clearing process itself had overhead.
2. "Table" ECS storage could be expensive to rebuild every frame relative to alternatives, due to "archetype moves". As a result, we employed many workarounds such as moving storage outside of the ECS.
3. Full resyncs every frame meant re-doing work that didn't need redoing. ECS gives us a nice global view of how our data is changing. We should take advantage of that!

In **Bevy 0.15** we switched to a "retained Render World". We no longer clear each frame. We no longer rely on a shared entity ID space. Instead:

1. Each world has its own entities
2. For entities that are related, we store that relationship as components (ex: Render World entities have a `MainEntity` component and Main World entities have a `RenderEntity` component). If a Main World entity with `SyncToRenderWorld` is spawned, we spawn an equivalent in the Render World. If a Main World entity is despawned, we despawn the relevant entity in the Render World.

Ensuring synchronization is perfect is _not_ an easy problem. Plugging all of the holes took a lot of time this cycle and we will likely continue to evolve our synchronization strategy in the future. But we think "retained" is fundamentally better for Bevy, and we're excited to have this foundation laid!
