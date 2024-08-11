+++
title = "Working Groups"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Working groups are temporary community initiatives devoted to particularly, complex, large, or important tasks. Some examples include:

- Revamping bevy's Color apis
- Replacing bevy's render graph
- Implementing relations
- And drafting this Contributing Guide!

:::info
You can read more about how to find and join active working groups in the [How You Can Help](todo) section.
:::

## Ground Rules

When participating in a working group, here are a few things to keep in mind:

- No one is in charge. The founders of a working group do not own it.
- These are social spaces. Everyone is invited, anyone can drop in to chat.
- Working groups are "hop-in hop-out". No commitments, minimal crunch, everyone contributes what they can when they can.
- The goal is consensus. Before going to the SMEs, try to come to a decision all the major stakeholders are content with.
- Communication is key. Having a group of people who are clued-in to the design process will let you sail through our review process.

## Life-Cycle of a Working Group

Working groups move through three loose phases: An initial proposal, a period of design and approval, and finally the bulk of the implementation work. These phases are intended to be lightweight, and are sometimes little more than a formality.

## Make A Proposal

Anyone is welcome to start a working group, all you need to do is get some friends together and submit a proposal:

1. Decide what the working group is going to focus on. This should be tightly focused and achievable!
2. Gather at least 3 people including yourself who are willing to be in the working group.
3. Ping the `@Maintainer` role on Discord in [#engine-dev](https://discord.com/channels/691052431525675048/692572690833473578) announcing your mutual intent and a one or two sentence description of your plans.

The maintainers will briefly evaluate the proposal in consultation with the relevant SMEs and give you a thumbs up or down on whether this is something Bevy can and wants to explore right now.
You don't need a concrete plan at this stage, just a sensible argument for both "why is this something that could be useful to Bevy" and "why there aren't any serious barriers in implementing this in the near future".
If they're in favor, a maintainer will create a forum channel for you and you're off to the races.

## Write A Design Doc

Your initial task is writing up a design doc: laying out the scope of work and general implementation strategy.
Here's a [solid example of a design doc](https://github.com/bevyengine/bevy/issues/12365), although feel free to use whatever format works best for your team.

Once that's ready, get a sign-off on the broad vision and goals from the appropriate SMEs and maintainers.
This is the primary review step: maintainers and SMEs should be broadly patient and supportive even if they're skeptical until a proper design doc is in hand to evaluate.

## Implement The Design Doc

With a sign-off in hand, post the design doc to [Github Discussions](https://github.com/bevyengine/bevy/discussions) with the [`C-Design-Doc` label](https://github.com/bevyengine/bevy/discussions?discussions_q=is%3Aopen+label%3A%22C-Design+Doc%22) for archival purposes and begin work on implementation.
Post PRs that you need review on in your group's forum thread, ask for advice, and share the load.
Controversial PRs are still `S-Controversial`, but with a sign-off-in-priniciple, things should go more smoothly.

If work peters out and the initiative dies, maintainers can wind down working groups (in consultation with SMEs and the working group itself).
This is normal and expected: projects fail for all sorts of reasons!
However, it's important to both keep the number of working groups relatively small and ensure they're active:
they serve a vital role in onboarding new contributors.

Once your implementation work laid out in your initial design doc is complete, it's time to wind down the working group.
Feel free to make another one though to tackle the next step in your grand vision!
