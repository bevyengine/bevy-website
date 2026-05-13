+++
title = "The Bevy Organization"
insert_anchor_links = "right"
[extra]
weight = 2
+++

The Bevy Organization is the group of people responsible for stewarding the Bevy Project. It handles things like merging pull requests, choosing project direction, managing bugs / issues / feature requests, running Bevy's website, controlling access to secrets, defining, and enforcing best practices, etc.

{% callout(type="info") %}
Everyone is welcome, and encouraged, to contribute to Bevy, whether or not they're a Bevy Organization member. Community contributors (this means you) are encouraged to freely open issues, submit pull requests, and review pull requests.
{% end %}

The Bevy Organization is currently broken up into the following roles:

## Project Lead

The Project Lead has the final call on all design and code changes within Bevy. This is to ensure a coherent vision and consistent quality of code. They are responsible for representing the project publicly and interacting with other entities (companies, organizations, etc) on behalf of the project. They choose how the project is organized, which includes how responsibility is delegated. The role of Project Lead implicitly confers power of other roles (Maintainer, Subject Matter Expert, etc).

[Carter Anderson] (`@cart`) is, for now, our singular project lead. He tries to be accountable, open to new ideas, and to changing his mind in the face of compelling arguments or community consensus.

[Carter Anderson]: https://github.com/cart

## Maintainer

Maintainers have merge rights in Bevy repositories. They assess the scope of pull requests and whether they fit into the Bevy Project's vision. They also serve as representatives of the Bevy Project and are often the interface between the Bevy community and the Bevy Project. They assist the Project Lead in moderating the community, handling administrative tasks, defining best practices, choosing project direction, and deciding how the project is organized.

Maintainers abide by the rules described in the section on [Reviewing Pull Requests](@/learn/contribute/helping-out/reviewing-pull-requests.md#how-pull-requests-are-merged) when merging pull requests.

We choose new Maintainers carefully, and only after they have proven themselves in the Bevy community. Maintainers must have a proven track record of the following:

1. **A strong understanding of the Bevy Project as a whole**: our vision, our development process, and our community
2. **Solid technical skills and code contributions across most engine areas**: Maintainers must be able to evaluate the scope of pull requests, provide complete code reviews, ensure the appropriate people have signed off on a PR, and decide if changes align with our vision for Bevy. This can only be done if Maintainers are technical experts, both generically across engine [Areas](/learn/contribute/project-information/areas/), and more specifically in the Bevy codebase.
3. **Great social skills**: Maintainers regularly deal with, and resolve, "community issues", and serve as moderators on Discord. They must always present a professional and friendly face. They are representatives of the project and their actions directly reflect our goals and values. Working with them should not be painful.
4. **Thorough reviews of other peoples' PRs**: Maintainers are the last line of defense when protecting project vision and code quality. They are also often the first people new contributors interact with. They must have a history of leaving thorough and helpful code reviews.
5. **Ethical and trustworthy behavior**: Maintainers are granted significant administrative permissions and serve as the board members of the Bevy Foundation. They must be trustworthy and professional.

To make it easy to reach consensus, hold a high quality bar, and synchronize vision, we intentionally keep the Maintainer team small.

If you are interested in a Maintainer role and believe you meet these criteria, reach out to the Project Lead or one of our Maintainers. One month after every Bevy release Maintainers and the Project Lead will evaluate the need for new roles, review candidates, and vote. Bringing in a new Maintainer requires unanimous support from the Project Lead and all Maintainers.

{% callout(type="info") %}
Check out the [Bevy People](/community/people/#the-bevy-organization) page for the current list of Maintainers.
{% end %}

## Subject Matter Expert (SME)

Subject Matter Experts are members of the Bevy Organization that have proven themselves to be experts in a given development [Area](/learn/contribute/project-information/areas/) (Rendering, Assets, ECS, UI, etc), and have a solid understanding of the Bevy Organization's vision for that area. They are great people to reach out to if you have questions about a given area of Bevy.

SMEs are responsible for:

- Reviewing / approving / voting on controversial PRs and designs
- Proposing and voting on [Bevy Project Goals](/learn/contribute/project-information/goals)
- Managing their [Area Project Board](/learn/contribute/project-information/areas)
- Being a point of contact for contributors interested in an Area

SME approvals count as "votes" on controversial PRs (provided the PR is in their "subject area"). This includes [RFCs](https://github.com/bevyengine/rfcs) and working groups design documents. If a controversial PR has two votes from Subject Matter Experts in that PR's area, it can be merged without Project Lead approval. If a SME creates a PR in their subject area, this does count as a vote.

However, the Project Lead has the right to revert changes merged this way, so it is each SME's responsibility to ensure they have synced up with the Project Lead's vision. Additionally, when approving a design, consensus between SMEs and the Project Lead (and ideally most of the wider Bevy community) is heavily encouraged. Merging without consensus risks fractured project vision and/or ping-ponging between designs. The "larger" the impact of a design, the more critical it is to establish consensus.

We choose new SMEs carefully, and only after they have proven themselves in the Bevy community. SMEs must have a proven track record of the following:

1. **Designing and contributing to foundational pieces in their subject area**: SMEs are responsible for building and extending the foundations of a given subject area. They must have a history of doing this before becoming an SME.
2. **Thorough reviews of other peoples' PRs in their subject area**: Within a subject area, SMEs are responsible for guiding people in the correct technical direction and only approving things aligned with that vision. They must have a history of doing this before becoming an SME.
3. **Great social skills**: Within a subject area, SMEs are responsible for reviewing peoples' code, communicating project vision, and establishing consensus. They are representatives of the project and their actions directly reflect our goals and values. Working with them should not be painful.

To make it easy to reach consensus, hold a high quality bar, and synchronize vision, we intentionally keep the number of SMEs in a given area small: 2 is the absolute minimum (to allow voting to occur), 3 is preferred, and 4 will be allowed in some cases. Bevy Organization members can be SMEs in more than one area, and Maintainers can also be SMEs.

If you are interested in a SME role and believe you meet these criteria, reach out to our Project Lead or one of our Maintainers. One month after every Bevy release Maintainers and the Project Lead will evaluate the need for new roles, review candidates, and vote. Bringing in a new SME requires the support of the Project Lead and half of the Maintainers (however unanimous support is preferred).

{% callout(type="info") %}
Check out the [Bevy People](https://bevy.org/community/people/#the-bevy-organization) page for the current list of SMEs.
{% end %}

## Bevy Org Member / Triage Team

[Bevy Org members](https://github.com/orgs/bevyengine/people) are contributors who:

1. Have actively engaged with Bevy development.
2. Have demonstrated themselves to be polite and welcoming representatives of the project with an understanding of our goals and direction.
3. Have asked to join the Bevy Org. Reach out to the Maintainers on [Discord](https://discord.gg/bevy) or email us at <support@bevy.org> if you are interested.

If you've been around the community for a while, just ask! There are no formal responsibilities and this is intended to cover the community of people making Bevy quite broadly.

All Bevy Org members are also Triage Team members. The Triage Team can label and close issues and PRs, but do not have merge rights or any special authority within the community.

## Role Rotation

All Bevy Organization roles, excluding the Triage Team, have the potential for "role rotation".

Roles like Project Lead, Maintainer, and SME are intentionally kept in limited supply to ensure a cohesive project vision. However these roles can be taxing, and qualified motivated people deserve a chance to lead.

To resolve these issues, we plan on building in "role rotation". What this looks like hasn't yet been determined (as this issue hasn't come up yet and we are still in the process of scaling out our team), but we will try to appropriately balance the needs and desires of both current and future leaders, while also ensuring consistent vision and continuity for Bevy.

Additionally, if you are currently holding a role that you can no longer "meaningfully engage with", please reach out to the Project Lead and Maintainers about rotating out. If you hold a role but don't engage with it, you are preventing other qualified people from driving the project forward.

{% callout() %}
Leaving a role doesn't need to be permanent. If you need to rotate out because your life is currently busy with work / life / school / etc, but later you find more time, we can discuss rotating back in!
{% end %}
