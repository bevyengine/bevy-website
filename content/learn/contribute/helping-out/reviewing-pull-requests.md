+++
title = "Reviewing Pull Requests"
insert_anchor_links = "right"
[extra]
weight = 4
+++

With the sheer volume of activity in Bevy's community, reviewing others' work with the aim of improving it is one of the most valuable things you can do.

## Who can review

Anyone!

You don't need to be an Elder Rustacean to be useful here: anyone can catch missing tests, unclear docs, logic errors, and so on. If you have specific skills (e.g., advanced familiarity with unsafe code, rendering knowledge or web development experience) or personal experience with a problem, try to prioritize those areas to ensure we can get appropriate expertise where we need it.

If you find a PR that you don't feel comfortable reviewing, but you can think of someone who does, consider using GitHub's "Request review" functionality (in the top-right of the PR screen) to bring the work to their attention. If they're not a Bevy Org member, you'll need to ping them in the thread directly: that's fine too! Almost everyone working on Bevy is a volunteer; this should be treated as a gentle nudge, rather than an assignment of work. Consider checking the Git history for appropriate reviewers, or ask on Discord for suggestions.

## How to review a Pull Request

If you're new to GitHub, there's a lot of great information on the official [Pull Request Review documentation](https://docs.github.com/en/github/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/about-pull-request-reviews). If you're happy with the work and feel you're reasonably qualified to assess quality in this particular area, leave your Approved review on the *PR*. Again, anyone can and should leave review— no special permissions are required!

## Giving feedback

Focus on giving constructive, actionable feedback that results in real improvements to code quality or end-user experience. If you don't understand why an approach was taken, please ask!

Provide actual code suggestions when that is helpful. Small changes work well as comments or in-line suggestions on specific lines of codes. Larger changes deserve a comment in the main thread, or a pull request to the original author's branch (but please mention that you've made one). When in doubt about a matter of architectural philosophy, please refer back to our page on [What We're Trying to Build](@/learn/contribute/introduction.md#what-we-re-trying-to-build) for guidance.

It's okay to leave an approval even if you aren't one-hundred percent confident on all areas of the PR, but just be sure to note your limitations. When Maintainers are evaluating the PR to be merged, they'll make sure that there's good coverage on all of the critical areas. If you can only check that the math is correct, and another reviewer can check everything but the math, we're in good shape!

Similarly, if there are areas that would be good to fix, but aren't severe, please consider leaving an approval. The author can address them immediately, or spin it out into follow-up issues or PRs. Large PRs are much more draining for both reviewers and authors, so try to push for a smaller scope with clearly tracked follow-ups.

## Requesting changes

It is quite common for reviews to ask for additional changes, but as a reviewer it's important to be respectful of the author's time.

It is always acceptable to ask an author to fix an issue or bug in their code, or request they do something differently. It is also fine to request additional documentation or tests, but when you do it's best to outline what exactly is lacking in the current version.

Otherwise, please try to avoid asking for additional work unless absolutely necessary, especially if the PR is in an otherwise acceptable state. The purpose of an open source code review is to assess the code as it is, not to tell the author what to do.

## Dealing with disagreements

With the large number of contributors and reviewers collectively building a complex system, disagreements will inevitably occur. Regardless of whether the disagreement is about naming, performance, or code coverage, the goal should be to reach a compromise amicably.

- **Keep scope in mind**. Trivial cleanup of surrounding area is okay to recommend, but don't expect a contributor to overhaul a portion of Bevy on their own.
- **Don't be afraid to escalate**. Regardless of what side of the review process you're on, if you feel someone is being unreasonable don't hesitate to tag one of the Maintainers or post in a related dev channel in the Discord.
- **Not all review comments will necessarily be addressed**, and that's okay. Feel free to open an issue for further discussion.

## What to review

There are three main places you can check for things to review:

1. Pull requests which are ready and in need of more reviews on the [`bevy` repository](https://github.com/bevyengine/bevy/pulls?q=is%3Aopen+is%3Apr+-label%3AS-Ready-For-Final-Review+-draft%3A%3Atrue+-label%3AS-Needs-RFC+-reviewed-by%3A%40me+-author%3A%40me).
2. Pull requests on the [`bevy`](https://github.com/bevyengine/bevy/pulls) and the [`bevy-website`](https://github.com/bevyengine/bevy-website/pulls) repositories.
3. [RFCs](https://github.com/bevyengine/rfcs), which need extensive thoughtful community input on their design.

Not even our Project Lead and Maintainers are exempt from reviews and RFCs! By giving feedback on this work (and related supporting work), you can help us make sure our releases are both high-quality and timely.

Finally, if nothing brings you more satisfaction than seeing every last issue labeled and all resolved issues closed, feel free to message @alice-i-cecile or @cart for a Bevy Org role to help us keep things tidy.

As discussed in our page on [The Bevy Organization](@/learn/contribute/project-information/bevy-organization.md), this role only requires good faith and a basic understanding of our development process.

## How Pull Requests are merged


Maintainers abide by the following rules when merging pull requests:

1. Trivial PRs can be merged with approval from one community member (including Maintainers).
2. Relatively uncontroversial PRs can be merged following approval from at least two community members (including Maintainers) with appropriate expertise.
3. Controversial PRs cannot be merged unless they have the approval of the Project Lead or two Subject Matter Experts (in the "area" of the PR).
4. If two Maintainers have approved a controversial PR they can "start the clock" on a PR by adding it to [this queue](https://github.com/orgs/bevyengine/projects/6). If 45 days elapse without SME or Project Lead action (approval, feedback or an explicit request to defer), the PR can be merged by maintainers.

{% callout() %}
To read more on Maintainers, check out our section on them in our [The Bevy Organization](@/learn/contribute/project-information/bevy-organization.md#maintainer) page.
{% end %}

