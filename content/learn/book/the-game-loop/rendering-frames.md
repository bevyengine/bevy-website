+++
title = "Rendering frames"
insert_anchor_links = "right"
[extra]
weight = 4
+++

Bevy employs a technique called "Pipelined Rendering" to increase graphics throughput.
This means, essentially, while rendering frame n we simulate frame n+1.
Bevy achieves this by moving all the rendering to a second [world] after the simulation for the frame is done.
Rendering can then progress in parallel on the render world while simulation for the next frame starts on the main world.

This process is complex enough to deserve it's own chapter.
For now, know that the state of the ecs is "frozen" after the `Last` schedule, and used to render a frame.
