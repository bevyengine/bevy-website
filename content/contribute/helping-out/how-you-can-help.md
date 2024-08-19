+++
title = "How You Can Help"
insert_anchor_links = "right"
[extra]
weight = 1
+++

If you've made it to this page, you're probably already convinced that Bevy is a project you'd like to see thrive.
But how can *you* help?

No matter how experienced you are with Bevy and Rust, or what your level of commitment is, there is a way for you to contribute! Take a look at the sections below to find a route that appeals to you (or many!).

## Battle-testing Bevy

Ultimately, Bevy is a tool that's designed to help people make cool games. Just by *using* Bevy you're helping us catch bugs, prioritize new features, polish off rough edges, and promote the project.

If you're using Bevy, we want to hear from you! Don't hesitate to ask for support on [GitHub Discussions], [Discord], or [Reddit]. Once you've made something that you're proud of, feel free to drop a link, video, or screenshot in `#showcase` on [Discord]! If you release a game on [itch.io] we'd be thrilled if you tagged it with `bevy`.

{% callout() %}
Think you've found a bug, missing documentation, or a feature that would help you make better games? [File an issue](https://github.com/bevyengine/bevy/issues/new/choose) on the main `bevy` repo! Take a look at the chapter on [Reporting Issues](@/contribute/helping-out/reporting-issues.md) to learn more.
{% end %}

[GitHub Discussions]: https://github.com/bevyengine/bevy/discussions
[Discord]: https://discord.gg/bevy
[Reddit]: https://www.reddit.com/r/bevy
[itch.io]: https://itch.io/games/tag-bevy

## Reviewing Others' Work

**One of the most valuable things you can do is review other people's work.** Bevy relies on community code review to keep the code quality high and the maintenance burden low. Absolutely anyone is welcome to leave reviews, and all reviews are treated with equal weight.

You don't need to be an Elder Rustacean to be useful here: anyone can catch missing tests, unclear docs, logic errors, and so on. If you're new to Rust or Bevy, submitting reviews is great way to learn things and acquaint yourself with the internals of the engine.

If you do have specific skills (e.g. advanced familiarity with `unsafe` code, rendering knowledge, or web development experience) or experience with a specific problem, that's fantastic! We suggest trying to prioritize those areas to ensure we get appropriate expertise where we need it.

Not even our Project Leads and Maintainers are exempt from reviews! By giving feedback on this work (and related supporting work), you can help us make sure our releases are both high-quality and timely.

{% callout() %}
Interested in reviewing but don't know where to start? Check out [Reviewing Pull Requests](TODO)!
{% end %}

## Joining a Working Group

Bevy's active initiatives are organized into *temporary working groups*: public, open-membership teams where people work together to tackle a sizeable, well-scoped goal. Each working group coordinates through a dedicated forum-channel on [Discord], but they may also create issues or use project boards to organize and track their progress.

You should consider joining a working group if you're interested in contributing but don't know where to start or what to work on. Choosing one and asking how to help can be a fantastic way to get up to speed and be immediately useful.

There are no special requirements to participate in a working group — no applications, formal membership list, or even leadership. Anyone can help, and you should expect to compromise and work together with others to bring your shared vision to life. Working groups are *spaces*, not clubs.

{% callout() %}
Anyone can start a working group! Check out [Working Groups](TODO) for more information about the process and requirements.
{% end %}

## Contributing Code

We love getting code contributions from the community, and there's plenty of work to go around! If you want to code but don't know what to work on, you should take a look at the open issues. The page on [Triage](@/contribute/reference/triage.md) details several tags that you can use to filter issues by difficulty and area.

If you already know what you want to work on, all you have to do is make your changes and submit a pull request! The page on [Your First Pull Request](TODO) is there to help if you get stuck.

{% callout() %}
When working on the engine's code, it's a good idea to introduce yourself in the `#engine-dev` channel on [Discord](https://discord.gg/bevy) and tell people about your plans. Communicating your progress early and often can help you avoid avoid headaches and disagreements during code review.
{% end %}

## Writing docs and examples

Bevy relies heavily on Rust's inline documentation and a collection of up-to-date examples, but both are in constant need of revision and improvement. If you'd like to help us improve our learning materials, take a look at the dedicated sections on [Writing Docs](TODO) and [Creating Examples](TODO).

## Expanding the Ecosystem

You can improve Bevy's ecosystem by building your own Bevy plugins and crates, or by helping to maintain existing third-party libraries.

Non-trivial, reusable functionality that works well by itself is a good candidate for a plugin. If it's closer to a snippet or design pattern, you may want to share it with the community on [Discord], [Reddit], or [GitHub Discussions] instead.

{% callout() %}
Check out our [plugin guidelines](@/learn/quick-start/plugin-development.md) for helpful tips and patterns!
{% end %}

## Organizing Issues and Pull Requests

If nothing brings you more satisfaction than seeing every last issue labeled and all resolved issues closed, feel free to hop on [Discord] and ask `@Maintainer` to be added to the GitHub organization. Anyone interested in helping us keep things neat and tidy is welcome to join. As will be discussed later, this role only requires good faith, a basic understanding of our development process, and a few merged pull requests.

## Teaching Others

Bevy is still very young, and it's light on documentation, tutorials, and accumulated expertise. By helping others with their issues and teaching them about Bevy, you will naturally learn the engine and codebase in greater depth (while also making our community better)!

Some of the best ways to do this are:

- Answering questions on [GitHub Discussions], [Discord], and [Reddit](https://www.reddit.com/r/bevy).
- Writing tutorials, guides, and other informal documentation and sharing them on [Bevy Assets](https://github.com/bevyengine/bevy-assets).
- Streaming, writing blog posts, and creating videos. Share these in the `#devlogs` channel on [Discord]!
