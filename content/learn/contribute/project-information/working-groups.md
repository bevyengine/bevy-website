+++
title = "Working Groups"
insert_anchor_links = "right"
[extra]
weight = 5
+++

Working Groups are temporary community initiatives devoted to accomplishing a [**Goal**](/learn/contribute/project-information/project-goals/) (a complex, large, or important task). This might include:

- Adding an unusually complex new feature
- Undertaking a major refactor
- Upstreaming an ecosystem crate
- Bootstrapping new major documentation efforts
- Preparing for a new release

As you can see, these aren't purely for programming tasks!

{% callout(type="info") %}
You can read more about how to find, and join, active Working Groups in the [How You Can Help](@/learn/contribute/helping-out/how-you-can-help.md) section.
{% end %}

## Ground Rules

When participating in a Working Group, here are a few things to keep in mind:

- Generally, no one is "in charge" of the Working Group; the founders of a Working Group do not own it. The SMEs (Subject Matter Experts) that have agreed to "staff" the **Goal** _can_ describe what they would like, and are responsible for providing prescriptive technical direction.
- These are social spaces where anyone is invited and can drop in to chat.
- Working groups are "hop-in hop-out". No commitments, minimal crunch, everyone contributes what they can when they can.
- The decisions made in Working Groups should be uncontroversial among the group, commonly agreed upon by all major stakeholders, before going to the Subject Matter Experts.
- Communication among the group is great, since a group of people who are clued-in to the design process will let you sail through our review process easier than you would otherwise.

## Life-Cycle of a Working Group

Working groups move through the following phases:

1. A [**Goal**](/learn/contribute/project-information/project-goals/) is proposed.
2. Project Leadership (SMEs and the Project Lead) approves the [**Goal**](/learn/contribute/project-information/project-goals/), and one or more SME agrees to "staff" it.
3. A Working Group is formed and the [**Goal**](/learn/contribute/project-information/project-goals/) becomes "active".
4. The Working Group comes up with design proposal(s) / prototypes, in collaboration with the ["staffing SME(s)"](/learn/contribute/project-information/project-goals/)
5. The design is approved by Project Leadership.
6. The design is implemented by the Working Group, in collaboration with the "staffing SME(s)".
7. The implementation is reviewed and approved.

## Make A Proposal

Anyone is welcome to propose a [**Goal**](/learn/contribute/project-information/project-goals/) / Working Group, all you need to do is get some friends together and submit a proposal:

1. Decide what the **Goal** of the Working Group will be. This should be tightly focused and achievable!
2. Gather at least 3 people including yourself who are willing to be in the Working Group.
3. Ping the appropriate `@SME-AREA` (ex: `@SME-Rendering`) role on Discord (and/or the `@Maintainer` role if you can't find the right SME role) in [#engine-dev](https://discord.com/channels/691052431525675048/692572690833473578) announcing your mutual intent and a one or two sentence description of your plans.

The SMEs, Maintainers, and Project Lead will briefly evaluate the proposal and give you a thumbs up or down on whether this is something Bevy can and wants to explore right now.
You don't need a concrete plan at this stage, just a sensible argument for both "why is this something that could be useful to Bevy" and "why there aren't any serious barriers in implementing this in the near future".
If they're in favor, the **SMEs** will create a **Goal** on GitHub, a Maintainer will create a forum channel for you, and you're off to the races.

## Write A Design Doc

Your initial task is writing up a design doc: laying out the scope of work and general implementation strategy.
Here's a [solid example of a design doc](https://github.com/Bevyengine/Bevy/issues/12365), although feel free to use whatever format works best for your team.

Once that's ready, get a sign-off on the broad vision and goals from the appropriate SMEs and Maintainers.
This is the primary review step: Maintainers and SMEs should be broadly patient and supportive even if they're skeptical until a proper design doc is in hand to evaluate.

## Implement The Design Doc

With a sign-off in hand, post the design doc to [GitHub Discussions](https://github.com/Bevyengine/Bevy/discussions) with the [`C-Design-Doc` label](https://github.com/Bevyengine/Bevy/discussions?discussions_q=is%3Aopen+label%3A%22C-Design+Doc%22) for archival purposes and begin work on implementation.
Post PRs that you need reviews on in your group's forum thread, ask for advice, and share the load.
Controversial PRs are still `X-Controversial`, but with a sign-off-in-principle, things should go more smoothly.

If work peters out and the initiative dies, Maintainers can wind down Working Groups (in consultation with SMEs and the Working Group itself).
This is normal and expected— projects fail for all sorts of reasons!
However, it's important to both keep the number of Working Groups relatively small and ensure they're active, since
they serve a vital role in onboarding new contributors.

Once your implementation work laid out in your initial design doc is complete, it's time to wind down the Working Group.
Feel free to make another one though to tackle the next step in your grand vision!
