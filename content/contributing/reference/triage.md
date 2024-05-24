+++
title = "Triage"
insert_anchor_links = "right"
[extra]
weight = 1
+++

Bevy's scope and user base means that it gets a large volume of issues and pull requests. This work all needs to be organized to make it easier to find related issues, figure out where you can help, and to quickly communicate the status of a task at a glance. The continual process of organizing work is called **triage**.

## Classifying issues and PRs

Labels are our primary tool for organizing work. You can find a complete list with descriptions [here](https://github.com/bevyengine/bevy/labels). These are a few of the most common, organized by category:

- **A**: Area (e.g. `A-Animation`, `A-ECS`, `A-Rendering`, ...).
- **C**: Category. The most common include:
    - `C-Bug`: unexpected or incorrect behavior.
    - `C-Enhancement`: a new feature or meaningful extension of an existing one.
    - `C-Docs`: an addition or correction to the documentation.
    - `C-Code-Quality`: a section of code that is hard to understand or change.
    - `C-Performance`: a change motivated by speed, memory usage, or compile times.
    - `C-Tracking-Issue`: collects information on a broad development initiative.
    - `C-Needs-Release-Notes`: work that should be called out in the blog post due to impact. This decision is usually made by maintainers, but feel free to nominate a change in the comments if you think it deserves the spotlight!
    - `C-Breaking-Change`: a breaking change to Bevy's public API, and should be noted in the migration guide.
- **D**: Difficulty. This can either be the estimated level of expertise (not time) to solve an issue or review a pull request. In order, these are:
    - `D-Trivial`: typos, obviously incorrect one-line bug fixes, code reorganization, and renames.
    - `D-Straightforward`: simple bug fixes, API improvements, docs, tests, and examples.
    - `D-Modest`: new features, refactors, and challenging bug fixes.
    - `D-Complex`: rewrites and unusually complex features.
    - The `D-Domain-Expert` and `D-Domain-Agnostic` labels are modifiers, which describe if unusually high or low degrees of domain-specific knowledge are required.
    - The `D-Unsafe` label is applied to any code that touches `unsafe` Rust, which requires special skills and scrutiny.
- **O**: Operating System (e.g. O-Linux, O-Web, O-Windows, ...).
- **P**: Priority (e.g. P-Critical, P-High, ...).
    - Most work is not explicitly categorized by priority; volunteer work mostly occurs on an ad hoc basis depending on contributor interests.
- **S**: Status. The most common include:
    - `S-Needs-Triage`: this issue needs to be labeled.
    - `S-Adopt-Me`: the original PR author has no intent to complete the PR, and it should be adopted by another contributor.
    - `S-Blocked`: cannot move forward until something else changes.
    - `S-Needs-Review`: this PR needs reviewer attention to move forward.
    - `S-Waiting-On-Author`: the author needs to make changes to this PR before it can be approved.
    - `S-Ready-For-Final-Review`: this PR has been approved by the community and is ready for a maintainer to consider merging it.
- **X**: Controversiality. In order, these are: 
    - `X-Uncontroversial`: everyone should agree that this is a good idea.
    - `X-Contentious`: there's real design thought needed to ensure that this is the right path forward.
    - `X-Controversial`: there's active disagreement and / or large-scale architectural implications involved.
    - `X-Blessed`: work that was previously controversial, but whose controversial (but perhaps not technical) elements have been endorsed by the relevant decision makers.

You can learn more about labels on [Github's documentation](https://docs.github.com/en/issues/using-labels-and-milestones-to-track-work/managing-labels).

The rules for how PRs get merged depend on their classification by controversy and difficulty. More difficult PRs will require more careful review from experts, while more controversial PRs will require rewrites to reduce the costs involved and / or sign-off from Subject Matter Experts and maintainers.

When making PRs, try to split out more controversial changes from less controversial ones, in order to make your work easier to review and merge. Also consider splitting simple changes from complex ones, since the simple changes can be reviewed and merged much quicker.

### Examples

Some reasons to apply `X-Controversial` or `X-Contentious` include:

1. Changes to a project-wide workflow or style.
2. New architecture for a large feature.
3. Serious tradeoffs were made.
4. Heavy user impact.
5. New ways for users to make mistakes (footguns).
6. Adding a new dependency of unknown repute.
7. Touching licensing information (due to the level of precision required).
8. Adding root-level files and folders (due to the high level of visibility).

Keep in mind that `X-Contentious` is a lesser form of `X-Controversial`. If you create a PR that _might_ be controversial but no one has objected to yet, consider adding `X-Contentious` so reviewers take extra care to consider the consequences.

Some reasons to apply `X-Uncontroversial` include:

- Fixing typos, grammar, and dead links.
- Removing dead code or unused dependencies.
- Fixing [Clippy] lints and warnings.

[Clippy]: https://doc.rust-lang.org/clippy/

Some reasons to apply `D-Complex` include:

1. High levels of technical complexity.
2. Introduction or modification of soundness-relevant code. (If it touches `unsafe`, be sure to add `D-Unsafe` as well!)
3. Large-scale code reoganization, where it is easy to miss small changes.

Some reasons to apply `D-Trivial` include:

1. Single-line changes.
2. Moving a file from one location to another.
3. Small changes to documentation or error messages.

Remember that difficulty labels are for expertise required to either solve an issue or review a pull request. By labeling an issue as `D-Trivial`, you are marking it as a good first issue for new contributors to the Bevy project.

## Closing PRs and Issues

From time to time, PRs are unsuitable to be merged in a way that cannot be readily fixed. Rather than leaving these PRs open in limbo indefinitely, they should simply be closed.

This might happen if:

1. The PR is spam or malicious.
2. The work has already been done elsewhere or is otherwise fully obsolete.
3. The PR was successfully adopted.
4. The work is particularly low quality, and the author is resistant to coaching.
5. The work adds features or abstraction of limited value, especially in a way that could easily be recreated outside of the engine.
6. The work has been sitting in review for so long and accumulated so many conflicts that it would be simpler to redo it from scratch.
7. The PR is pointlessly large, and should be broken into multiple smaller PRs for easier review.

PRs that are `S-Adopt-Me` should be left open, but only if they're genuinely more useful to rebase rather than simply use as a reference.

There are several paths for PRs to be closed:

1. Authors may close their own PRs for any reason at any time.
2. If a PR is clearly spam or malicious, anyone with triage rights is encouraged to close out the PR and report it to Github.
3. If the work has already been done elsewhere, adopted or otherwise obsoleted, anyone with triage rights is encouraged to close out the PR with an explanatory comment.
4. Anyone may nominate a PR for closure, by bringing it to the attention of the author and / or one of the SMEs / maintainers. Let them press the button, but this is generally well-received and helpful.
5. SMEs or maintainers may and are encouraged to unilaterally close PRs that fall into one or more of the remaining categories.
6. In the case of PRs where some members of the community (other than the author) are in favor and some are opposed, any two relevant SMEs or maintainers may act in concert to close the PR.

When closing a PR, check if it has an issue linked. If it does not, you should strongly consider creating an issue and linking the now-closed PR to help make sure the previous work can be discovered and credited.

## Triage Team

Members of the Triage Team within the Bevy organization have permissions to label and close issues, though they do not have merge rights or special authority. Anyone is free to join as long as:

1. They have actively engaged with Bevy development in the past.
2. They have demonstrated themselves to be polite and welcoming representatives of the project with an understanding of its goals and direction.

If that applies to you, then feel free to ask a maintainer on [Discord] or email <bevyengine@gmail.com>. Everyone is welcome to do this. We generally accept membership requests, so don't hesitate if you are interested!

[Discord]: https://discord.gg/bevy
