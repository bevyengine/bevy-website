+++
title = "Project Goals"
insert_anchor_links = "right"
[extra]
weight = 4
+++

The [Bevy Organization](/learn/contribute/project-information/bevy-organization/) uses **Goals** to track and plan "big" efforts.

## The Project Goals Board

**Goals** are tracked on the [Project Goals Board](https://github.com/orgs/bevyengine/projects/23/views/1) on GitHub, which fills a similar role to a "roadmap". It is a great place to see what we are currently working on, what we would _like_ to work on in the future, and what we _don't_ plan on working on. It is also a great place to find efforts to contribute to!

## What is a Goal?

- A **Goal** is work that is impactful, high-effort, controversial, or complicated. In general, nobody should be making large or controversial changes to the engine without an active, approved **Goal**.
- **Goals** define a high level feature or need, not low level implementation details. Implementation details are determined by the [Working Group](/learn/contribute/project-information/working-groups/) and approved by [SMEs](learn/contribute/project-information/bevy-organization).
- **Goals** are expressed as [Issues with the `C-Goal` label](https://github.com/bevyengine/bevy/issues?q=is%3Aissue%20label%3AC-Goal) in the main [Bevy GitHub Repo](https://github.com/bevyengine/bevy/).
- **Goals** can be large, but they need a defined completion criteria.
- **Goals** can have "sub-**Goals**", which are expressed as "sub-issues" on GitHub
  - For something large like a "Bevy Editor" Goal, consider breaking it into smaller pieces (ex: Editor UI Widgets, Core Editor App Framework, Scene Editor, Editor Inspector, etc). The pieces should _still_ be **Goal**-like ... avoid dictating implementaton details. Also avoid trying to plan out large "goal trees" in advance. "Expand" the tree lazily, as the need arises and you understand more.
  - It is critical to define completion criteria for larger **Goals** with sub-**Goals**, to avoid **Goals** that live forever. A **Goal** is _not_ an **Area**.
- **Goals** can organically change over time. If we learn something new and it makes sense to break up a **Goal** into multiple **Goals**, add sub-**Goals**, or reframe the **Goal** ... do it!
- **Goals** inherently require _collaboration_ to bring them to completion. They are never the work of a single individual. At the very least, there is someone coming up with a design / implementation, and one or more **SME** verifying the design / implementation.
- **Goals** have a short, functional name, as it would be communicated to the public. This is a market-able "feature name", like "PBR Renderer", "Relationships", etc. Avoid using things like "initial" or "MVP" in the name, as this is implied.
- **Goals** are inherently `X-Needs-SME`, so they should be given that label.
- Anything that needs a **Goal**, but doesn't currently have one should be labeled with `C-Needs-Goal`.

## When does an Issue or PR need a Goal?

- In general, `C-Needs-Goal` is a measure of investment, risk, and publicity. If SMEs cannot resolve the work with a very short time investment and low risk, or the work is an ongoing thing, it almost certainly needs a **Goal**. If it has significant implications for public reception or has significant public interest, it needs a **Goal**.
- When in doubt, it probably needs a **Goal**
- Is the work a part of a larger whole? If yes, that "whole" probably needs a **Goal**
- Does the work require a long iterative development process? If yes, it probably needs a **Goal**
- Would the work benefit from collaboration and consensus building? If yes, it probably needs a **Goal**
- Does the work have significant implications for Bevy's user experience, public reception, or internals? If so, it probably needs a **Goal**
- Note that `C-Goal` is inherently `X-Needs-SME`, but not all `X-Needs-SME` work is `C-Goal`. Smaller, easily resolved, less controversial, less impactful `X-Needs-SME` things can sometimes get away with not having a **Goal**.

## Why do we use Goals?

- **Goals** allow the Bevy Organization to define, discuss, and share its priorities with the community
- **Goals** let us give quick yes/no/later feedback for feature proposals as early as possible
- **Goals** allow big efforts move forward _when_ they are staffed by SMEs
- **Goals** help ensure that big efforts do _not_ move forward when they are _not_ staffed by SMEs. This helps protect contributors from building things we aren't ready for (Bevy's technical leadership has limited capacity) or don't want.
- **Goals** make it easier for interested developers to find "project aligned" work

## Goal States

Goals can have a variety of "states", which are expressed as "statuses" on the [Project Goals Board](https://github.com/orgs/bevyengine/projects/23/views/1).

- **Proposed**: One or more **SME** or **Maintainer** thinks this **Goal** is worth considering
  - **Goals** enter this state when they are created by an SME, Maintainer, or Project Lead
  - The SMEs and Project Lead discuss what state to transition the Goal to. Ideally **Goals** do not stay in the Proposed state for long.
- **Postponed**: We might want to do this later, but we don't have the bandwidth or inclination to invest in it now.
- **Blocked (Approved)**: We want to do this, one or more SME has agreed to "staff" it, and the Project Lead has approved it, but other work is blocking it
  - **Goals** that are blocked on other **Goals** or issues should include a "blocked by" relationship on GitHub.
- **Inactive (Approved)**: We want to do this, one or more **SME** has agreed to staff it, the **Project Lead** has approved it, but there is no active **Working Group**.
- **Active (Approved)**:  A **Working Group** is actively working toward this, with one or more active **SME** staffing it.
- **Done**: The goal's issue has been closed. This actually has two "sub states" (which are _not_ expressed as Project Board Statuses, as GitHub issues already encode the relevant information):
  - **Declined**: Closed as "not planned"
  - **Completed**: Closed as "completed"

### Staffing / Approving a Goal

**Goals** can be "staffed" by one or more [SME](learn/contribute/project-information/bevy-organization). Staffing a **Goal** means:

1. The SME believes the **Goal** should completed
2. The SME is willing to spend time guiding the vision and implementation
3. The SME believes they have the competency required to be a good technical shepherd of the **Goal**

If an appropriate **SME** is unavailable, **Maintainers** can also choose to staff a **Goal**, provided relevant **SMEs** and the **Project Lead** are on board.

The SMEs staffing a **Goal** should be listed in the **Goal**'s description.

## The Life-cycle of a Goal

1. Someone proposes a feature in the form of an issue
    - By default this issue is not a **Goal**, but **SMEs**, **Maintainers**, and the **Project Lead** can skip to step (5) and create proposed **Goal** issues directly.
2. The Triage Team assigns it to an **Area**
3. A [Github Project Workflow](https://docs.github.com/en/issues/planning-and-tracking-with-projects/automating-your-project/using-the-built-in-automations) adds it to the **Area Project**
4. **SMEs** identify that the issue needs a goal and apply the `C-Needs-Goal` label
5. **SMEs** discuss the framing of the Goal and create a new issue with the `C-Goal` label.
6. A Github Project Workflow picks up the `C-Goal` issue and adds it to the **Project Goals** board in the Proposed state.
7. The **Project Lead** and **SMEs** discuss how to handle the **Goal**: Deny, Inactive (Approved), or Postpone. Approval is contingent on an **SME** agreeing to staff it, and put energy into helping the Working Group bring it to completion.
8. If the **Goal** is Inactive (Approved), and there is enough commitment and support from the community to form a **Working Group**, it moves into the Active (Approved) state.
9. The **Project Lead** adjusts the priorities of the **Goals** (expressed via Project Board order), based on the needs of the project.
10. When a **Goal** is completed, the issue is closed out. A GitHub Project Workflow moves it into the Done status.
