+++
title = "Areas"
insert_anchor_links = "right"
[extra]
weight = 3
+++

An **Area** is a "design space", such as Rendering, Assets, and Scenes. Technical work being done in Bevy almost always belongs to one or more **Area**.

An **Area** is [governed](/learn/contribute/project-information/bevy-organization/) by **SMEs** (Subject Matter Experts) and the **Project Lead**. Some **Areas** are not foundational enough or active enough to merit **SMEs**. In these cases, the **Project Lead** governs the **Area**.

On GitHub, Issues and Pull Requests in a given **Area** are labeled with the `A-AREA-HERE` label (`A-Rendering`, `A-Assets`, etc).

## Area Projects

Each major **Area** has a "GitHub Project", where work in the **Area** is organized, prioritized, and blessed by **SMEs** and the **Project Lead**. "Area Projects" are a great way to see at a glance what is happening in an **Area**. They are also a great entry point for Contributors to find work in an **Area**.

Each **Area Project** is instantiated from the standardized [Area Project Template](https://github.com/orgs/bevyengine/projects/24/views/1).

As much as possible, we drive the "views" in these projects with query-able data such as GitHub labels, rather than defining new Area Project-specific states (which we reserve for SME triaging).

These are the current Area Projects:

- [Animation](https://github.com/orgs/bevyengine/projects/28)
- [Assets](https://github.com/orgs/bevyengine/projects/34)
- [Audio](https://github.com/orgs/bevyengine/projects/33)
- [ECS](https://github.com/orgs/bevyengine/projects/27)
- [Input](https://github.com/orgs/bevyengine/projects/29)
- [Reflection](https://github.com/orgs/bevyengine/projects/31)
- [Rendering](https://github.com/orgs/bevyengine/projects/26)
- [UI](https://github.com/orgs/bevyengine/projects/32)

### SME Triaging

Issues and PRs added to the Area Project have the following states:

- **Needs SME Triage**: An **SME** has not triaged this yet
  - When an Issue or PR is assigned an **Area** label by the [Triage Team](/learn/contribute/project-information/bevy-organization/), it is automatically added to the Area Project in the "Needs SME Triage" state.
- **SME Triaged**: An **SME** has triaged this. To triage, an SME should do the following:
  - Try to understand the issue / PR, from a surface level
  - If clarity is needed (or a minimal repro, in the case of a bug), leave a comment asking for it
  - If you have any quick useful context that might help (ex: related issues or PRs, complete or partial solutions, threads to pull on, people to ping, etc), leave a comment.
  - Is this a duplicate? If so, close it as such and link to the existing issue.
  - Is this an immediate, pressing concern? If so, label it with `P-High`.
  - Is this "controversial"? Does it need an SME vote to resolve it? If so, add the `X-Needs-SME` label.
  - Does this need a [**Goal**](/learn/contribute/project-information/project-goals)? If so, either create the **Goal** or add the `S-Needs-Goal` label. For existing / new **Goals**, comment with a link to the **Goal** to make a bi-directional connection between the two. If the issue is redundant with the **Goal**, close it.
  - Switch the state to **SME Triaged**, either by clicking the state and changing it via the dropdown, or by dragging the issue into the **SME Triaged** group.
  - Do a quick "vibes based" priority ordering by dragging the triaged item relative to other triaged items. This isn't an exact science, just make sure that important things aren't buried deep.
- **Done**: The Issue or PR was closed. This state is set automatically ... don't do it manually!

Triaging should be a quick process. SMEs ... don't be overwhelmed! You don't need to solve, or completely understand, the issue to triage it.

Note that the SME role is generally occupied by volunteers. We do not set any expectations about when they triage, or how much they triage. Activity is a data point, not an expectation.

We use this "triage" system for the following reasons:

- We want new Issues and PRs to be seen by at least one **SME** or **Project Lead**, and these states encode that information.
- SMEs must know what is happening within their **Area** if they are going to effectively govern it.
- If SMEs are not looking at incoming Issues and PRs, that is a sign that the **Area** is understaffed and likely needs more SMEs. It also makes it easy for the **Project Lead** and/or **Maintainers** to identify where they need to step in to pick up the slack.
