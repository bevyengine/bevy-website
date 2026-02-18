+++
title = "Project Goals"
insert_anchor_links = "right"
[extra]
weight = 4
+++

The [Bevy Organization](/learn/contribute/project-information/bevy-organization/) uses **Goals** to track and plan "big" efforts.

## The Project Goals Board

Goals are tracked on the [Project Goals Board](https://github.com/orgs/bevyengine/projects/23/views/1) on GitHub, which fills a similar role to a "roadmap". It is a great place to see what we are currently working on, what we would _like_ to work on in the future, and what we _don't_ plan on working on. It is also a great place to find efforts to contribute to!

## What is a Goal?

- A Goal is work that is impactful, high-effort, controversial, or complicated. In general, nobody should be making large or controversial changes to the engine without an active, approved Goal.
- Goals define a high level feature or need, not low level implementation details. Implementation details are determined by the [Working Group](/learn/contribute/project-information/working-groups/) and approved by [SMEs](/learn/contribute/project-information/bevy-organization).
- Goals are expressed as [Issues with the `C-Goal` label](https://github.com/bevyengine/bevy/issues?q=is%3Aissue%20label%3AC-Goal) in the main [Bevy GitHub Repo](https://github.com/bevyengine/bevy/).
- Goals can be large, but they need a defined completion criteria.
- Goals can have "sub-Goals", which are expressed as "sub-issues" on GitHub
  - For something large like a "Bevy Editor" Goal, consider breaking it into smaller pieces (ex: Editor UI Widgets, Core Editor App Framework, Scene Editor, Editor Inspector, etc). The pieces should _still_ be Goal-like ... avoid dictating implementation details.
  - Avoid trying to plan out large "sub-goal trees" in advance. "Expand" the tree lazily, as the need arises and you understand more. Generally only do this _after_ a **Goal** has been Postponed or Approved.
  - It is critical to define completion criteria for larger Goals with sub-Goals, to avoid Goals that live forever. A Goal is _not_ an **Area**.
- Goals can organically change over time. If we learn something new and it makes sense to break up a Goal into multiple Goals, add sub-Goals, or reframe the Goal ... do it!
- Goals inherently require _collaboration_ to bring them to completion. They are never the work of a single individual. At the very least, there is someone coming up with a design / implementation, and one or more **SME** verifying the design / implementation.
- Goals are inherently `X-Needs-SME`, so they should be given that label.
- Anything that needs a Goal, but doesn't currently have one should be labeled with `C-Needs-Goal`.
- Goals are typically created by **SMEs** or **Maintainers**.
  - However, trusted contributors may create Goals after discussion with at least one **SME** or **Maintainer**.
  - Goals created without following this process will be deleted without consideration, and a warning will be issued.
  - The intent behind this restriction is that Goals are a "public facing" / marketing thing, so phrasing and framing is important. Every new Goal gets added to the Proposed goal column on the public Project Goals board. Going through SMEs + Maintainers ensures redundancies are avoided, noise is kept to a minimum, consistency is enforced, templates are used, framing / public image / marketing is taken into account, etc. This is just as true for closed Goals, which exist to be a nice consolidated list of things the project definitely doesn't want to do.

## When does an Issue or PR need a Goal?

- In general, `C-Needs-Goal` is a measure of investment, risk, and publicity. If SMEs cannot resolve the work with a very short time investment and low risk, or the work is an ongoing thing, it almost certainly needs a Goal. If it has significant implications for public reception or has significant public interest, it needs a Goal.
- When in doubt, it probably needs a Goal
- Is the work a part of a larger whole? If yes, that "whole" probably needs a Goal
- Does the work require a long iterative development process? If yes, it probably needs a Goal
- Would the work benefit from collaboration and consensus building? If yes, it probably needs a Goal
- Does the work have significant implications for Bevy's user experience, public reception, or internals? If so, it probably needs a Goal
- Note that `C-Goal` is inherently `X-Needs-SME`, but not all `X-Needs-SME` work is `C-Goal`. Smaller, easily resolved, less controversial, less impactful `X-Needs-SME` things can sometimes get away with not having a Goal.

## Why do we use Goals?

- Goals allow the Bevy Organization to define, discuss, and share its priorities with the community
- Goals let us give quick yes/no/later feedback for feature proposals as early as possible
- Goals allow big efforts move forward _when_ they are staffed by SMEs
- Goals help ensure that big efforts do _not_ move forward when they are _not_ staffed by SMEs. This helps protect contributors from building things we aren't ready for (Bevy's technical leadership has limited capacity) or don't want.
- Goals make it easier for interested developers to find "project aligned" work

## Goal States

Goals can have a variety of "states", which are expressed as "statuses" on the [Project Goals Board](https://github.com/orgs/bevyengine/projects/23/views/1).

- **Proposed**: One or more **SME** or **Maintainer** thinks this Goal is worth considering
  - Goals enter this state when they are created by an SME, Maintainer, or Project Lead
  - The SMEs and Project Lead discuss what state to transition the Goal to. Ideally Goals do not stay in the Proposed state for long.
- **Postponed**: We might want to do this later, but we don't have the bandwidth or inclination to invest in it now.
- **Blocked (Approved)**: We want to do this, one or more SME has agreed to "staff" it once unblocked, and the Project Lead has approved it, but other work is required first
  - Goals that are blocked on other Goals or issues should include a "blocked by" relationship on GitHub.
- **Inactive (Approved)**: We want to do this, one or more **SME** has agreed to staff it, the **Project Lead** has approved it, but there is no active **Working Group**
  - This is essentially the list of Goals Bevy Leadership really wants to work on now / is willing to support, but without anyone actively working on them. Interested contributors are highly encouraged to help form a Working Group for these Goals.
- **Active (Approved)**:  A **Working Group** is actively working toward this, with one or more active **SME** staffing it.
- **Done**: The goal's issue has been closed. This actually has two "sub states" (which are _not_ expressed as Project Board Statuses, as GitHub issues already encode the relevant information):
  - **Declined**: Closed as "not planned"
  - **Completed**: Closed as "completed"

### Staffing / Approving a Goal

Goals can be "staffed" by one or more [SME](/learn/contribute/project-information/bevy-organization). Staffing a Goal means:

1. The SME believes the Goal should be completed
2. The SME is willing to spend time guiding a Working Group as they design and implement the Goal
3. The SME believes they have the competency required to be a good technical shepherd of the Goal

A "staffing" SME's job is to shepherd the Working Group in the right direction. They generally have the final say on design and implementation proposals, prior to the final reviews by the other SMEs / Project Lead. Staffing SMEs are welcome (and encouraged) to help draft designs and implementations, but this is not _required_. In _all_ cases they are responsible for ensuring that the Working Group moves in the correct direction.

If a "staffing SME" can no longer dedicate the time necessary to vet the outputs of a Working Group, they should inform the relevant members of Bevy Leadership, who will then decide the best way to move forward (ex: Keeping moving forward if there are still enough staffing SMEs, find a new SME to staff the Goal, or Postpone the Goal).

If an appropriate **SME** is unavailable, **Maintainers** can also choose to staff a Goal, provided relevant **SMEs** and the **Project Lead** are on board.

The SMEs staffing a Goal should be listed in the Goal's description.

## The Life-cycle of a Goal

1. Someone proposes a feature in the form of an issue
    - By default this issue is not a Goal, but **SMEs**, **Maintainers**, and the **Project Lead** can skip to step (5) and create proposed Goal issues directly.
2. The Triage Team assigns it to an **Area**
3. A [Github Project Workflow](https://docs.github.com/en/issues/planning-and-tracking-with-projects/automating-your-project/using-the-built-in-automations) adds it to the **Area Project**
4. **SMEs** identify that the issue needs a goal and apply the `C-Needs-Goal` label
5. **SMEs** discuss the framing of the Goal and create a new issue with the `C-Goal` label.
6. A Github Project Workflow picks up the `C-Goal` issue and adds it to the **Project Goals** board in the Proposed state.
7. The **Project Lead** and **SMEs** discuss how to handle the Goal: Deny, Inactive (Approved), or Postpone. Approval is contingent on an **SME** agreeing to staff it, and put energy into helping the Working Group bring it to completion.
8. If the Goal is Inactive (Approved), and there is enough commitment and support from the community to form a **Working Group**, it moves into the Active (Approved) state.
9. The **Project Lead** adjusts the priorities of the Goals (expressed via the [Project Goals Board](https://github.com/orgs/bevyengine/projects/23/views/1) order), based on the needs of the project.
10. When a Goal is completed, the issue is closed out. A GitHub Project Workflow moves it into the Done status.

## Goal Issue Template

Typically, only SMEs and Maintainers should create new Goals (see rationale in the "What is a Goal?" section above)!
However, trusted contributors may create a goal after discussion with relevant SMEs and Maintainers.

If that is you, copy this template when creating a new Goal. We put this template here, rather than adding it as a GitHub issue template, because we don't want normal users creating Goals.

1. Name the Goal. This should be a short, functional name, as it would be communicated to the public. This is a market-able "feature name", like "PBR Renderer", "Relationships", etc. Avoid using things like "initial" or "MVP" in the name, as this is implied.
2. Add the `C-Goal` and `X-Needs-SME` labels.
3. Paste in the markdown template below.
4. Fill in the template.

```md
## Goal Description

In roughly one paragraph, describe from a high level _what_ this Goal is. Provide just enough to describe to the public and contributors _what_ the bounds of the Goal are. Leave the "why" and "how" to the future **Design Documents**. Avoid dictating implementation or design details whenever possible.

## Goal Status

This is a potential [Bevy Project Goal](https://bevy.org/learn/contribute/project-information/project-goals/). Its state on the [Project Goals Board](https://github.com/orgs/bevyengine/projects/23/views/1) determines if it is Proposed, Postponed, Blocked (Approved), Inactive (Approved), Active (Approved), Completed, or Declined.

- **Staffing SMEs**: Not yet staffed
- **Working Group Link**: Working Group not yet formed
- **Design Documents**: Design documents not yet written
```

- If / when a Goal is approved and SMEs agree to "staff" the Goal, add them to the **Staffing SMEs** list.
- If / when a Goal becomes "active" and a Working Group is formed, fill in the **Working Group Link** with the Working Group thread on the Bevy Discord.
- If / when design documents are authored by the Working Group, link to them in the **Design Documents**.
