+++
title = "Opening Pull Requests"
insert_anchor_links = "right"
[extra]
weight = 3
+++

## Making changes to Bevy

Most changes don't require much "process". If your change is relatively straightforward, just do the following:

1. A community member (that's you!) creates one of the following:
    * [GitHub Discussions]: An informal discussion with the community. This is the place to start if you want to propose a feature or specific implementation.
    * [Issue](https://github.com/bevyengine/bevy/issues): A formal way for us to track a bug or feature. Please look for duplicates before opening a new issue and consider starting with a Discussion.
    * [Pull Request](https://github.com/bevyengine/bevy/pulls) (or PR for short): A request to merge code changes. This starts our "review process". You are welcome to start with a pull request, but consider starting with an Issue or Discussion for larger changes (or if you aren't certain about a design). We don't want anyone to waste their time on code that didn't have a chance to be merged! But conversely, sometimes PRs are the most efficient way to propose a change. Use your own judgment here.
2. Other community members review and comment in an ad-hoc fashion. Active Subject Matter Experts may be pulled into a thread using `@mentions`. If your PR has been quiet for a while and is ready for review, feel free to leave a message to "bump" the thread, or bring it up on [Discord] in an appropriate engine development channel.
3. Once they're content with the pull request (design, code quality, documentation, tests), individual reviewers leave "Approved" reviews.
4. After consensus has been reached (see [Reviewing Pull Requests](@/learn/contribute/helping-out/reviewing-pull-requests.md#how-pull-requests-are-merged) for more on that) and CI passes, the [S-Ready-For-Final-Review](https://github.com/bevyengine/bevy/issues?q=is%3Aopen+is%3Aissue+label%3AS-Ready-For-Final-Review) label is added.
5. When they find time, the Project Lead or a Maintainer performs a final code review and queues the PR for merging.

[Discord]: https://discord.com/invite/bevy
[GitHub Discussions]: https://github.com/bevyengine/bevy/discussions

### Complex changes

Individual contributors often lead major new features and reworks. However these changes require more design work and scrutiny. Complex changes like this tend to go through the following life-cycle:

1. A need or opportunity is identified and an issue is made, laying out the general problem.
2. As needed, this is discussed further on that issue thread, in cross-linked [GitHub Discussions] threads, or on [Discord] in the Engine Development channels.
3. Either a Draft Pull Request or an RFC is made. As discussed in the [RFC repo](https://github.com/bevyengine/rfcs), complex features need RFCs, but these can be submitted before or after prototyping work has been started.
4. If feasible, parts that work on their own (even if they're only useful once the full complex change is merged) get split out into individual PRs to make them easier to review.
5. The community as a whole helps improve the Draft PR and/or RFC, leaving comments, making suggestions, and submitting pull requests to the original branch.
6. Once the RFC is merged and/or the Draft Pull Request is transitioned out of draft mode, the [normal change process outlined in the previous section](#making-changes-to-bevy) can begin.

## Contributing Code

Bevy is actively open to code contributions from community members.
If you're new to Bevy, here's the workflow we use:

1. Fork the `bevyengine/bevy` repository on GitHub. You'll need to create a GitHub account if you don't have one already.
    1. Copy `.cargo/config_fast_builds.toml` to `.cargo/config.toml`. Then update the file and follow the general
  recommendations to [compile with performance optimizations](https://bevyengine.org/learn/quick-start/getting-started/setup/#compile-with-performance-optimizations).
2. Make your changes in a local clone of your fork, typically in its own new branch.
   1. Try to split your work into separate commits, each with a distinct purpose. Be particularly mindful of this when responding to reviews so it's easy to see what's changed.
   2. {% callout() %}
    Tip: [You can set up a global `.gitignore` file](https://docs.github.com/en/get-started/getting-started-with-git/ignoring-files#configuring-ignored-files-for-all-repositories-on-your-computer) to exclude your operating system/text editor's special/temporary files. (e.g. `.DS_Store`, `thumbs.db`, `*~`, `*.swp` or `*.swo`) This allows us to keep the `.gitignore` file in the repository uncluttered.
    {% end %}

3. To test CI validations locally, run the `cargo run -p ci` command. This will run most checks that happen in CI, but can take some time. You can also run sub-commands to iterate faster depending on what you're contributing:
    * `cargo run -p ci -- lints` - to run formatting and Clippy.
    * `cargo run -p ci -- test` - to run tests.
    * `cargo run -p ci -- doc` - to run doc tests and doc checks.
    * `cargo run -p ci -- compile` - to check that everything that must compile still does (examples and benches), and that some that we want to be sure *don't* compile ([`crates/bevy_ecs/compile_fail`](https://github.com/bevyengine/bevy/tree/main/crates/bevy_ecs/compile_fail)).
    * to get more information on commands available and what is run, check the [tools/ci crate](https://github.com/bevyengine/bevy/tree/main/tools/ci).
4. When working with Markdown (`.md`) files, Bevy's CI will check markdown files (like this one) using [markdownlint](https://github.com/DavidAnson/markdownlint).
To locally lint your files using the same workflow as our CI:
   1. Install [markdownlint-cli](https://github.com/igorshubovych/markdownlint-cli).
   2. Run `markdownlint -f -c .github/linters/.markdown-lint.yml .` in the root directory of the Bevy project.
5. When working with TOML (`.toml`) files, Bevy's CI will check them for style and correctness using [taplo](https://taplo.tamasfe.dev/): `taplo fmt --check --diff`
   1. If you use VSCode, install [Even Better TOML](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml) and format your files.
   2. If you want to use the CLI tool, install [taplo-cli](https://taplo.tamasfe.dev/cli/installation/cargo.html) and run `taplo fmt --check --diff` to check for the formatting. Fix any issues by running `taplo fmt` in the root directory of the Bevy project.
6. Check for typos. Bevy's CI will check for them using [typos](https://github.com/crate-ci/typos).
   1. If you use VSCode, install [Typos Spell Checker](https://marketplace.visualstudio.com/items?itemName=tekumara.typos-vscode).
   2. You can also use the CLI tool by installing [typos-cli](https://github.com/crate-ci/typos?tab=readme-ov-file#install) and running `typos` to check for typos, and fix them by running `typos -w`.
7. Push your changes to your fork on GitHub and open a Pull Request.
8. Respond to any CI failures or review feedback. While CI failures must be fixed before we can merge your PR, you do not need to *agree* with all feedback from your reviews, merely acknowledge that it was given. If you cannot come to an agreement, leave the thread open and defer to a Maintainer or the Project Lead's final judgement.
9. When your PR is ready to merge, a Maintainer or the Project Lead will review it and suggest final changes. If those changes are minimal they may even apply them directly to speed up merging.

If you end up adding a new official Bevy crate to the `bevy` repository:

1. Add the new crate to the [tools/publish.sh](https://github.com/bevyengine/bevy/blob/main/tools/publish.sh) file.
2. Check if a new cargo feature was added, update [docs/cargo_features.md](https://github.com/bevyengine/bevy/blob/main/docs/cargo_features.md) as needed.

When contributing, please:

* Prefer small PRs that incrementally improve things.
* Explain what you're doing and why.
* Try to loosely follow the workflow in [*Making changes to Bevy*](#making-changes-to-bevy).
* Consult the [style guide](@/learn/contribute/helping-out/opening-pull-requests.md#style-guide) to help keep our code base tidy.
* Document new code with doc comments.
* Include clear, simple tests.
* Add or improve the examples when adding new user-facing functionality.
* Break work into digestible chunks.
* Ask for any help that you need!

Your first PR will be merged in no time!

No matter how you're helping, thanks for contributing to Bevy!

## Style Guide

### General guidelines

1. Prefer granular imports over glob imports like `bevy_ecs::prelude::*`.
2. Use a consistent comment style:
   1. `///` doc comments belong above `#[derive(Trait)]` invocations.
   2. `//` comments should generally go above the line in question, rather than in-line.
   3. Avoid `/* */` block comments, even when writing long comments.
   4. Use \`variable_name\` code blocks in comments to signify that you're referring to specific types and variables.
   5. Start comments with capital letters. End them with a period if they are sentence-like.
3. Use comments to organize long and complex stretches of code that can't sensibly be refactored into separate functions.
4. When using [Bevy error codes](https://bevyengine.org/learn/errors/) include a link to the relevant error on the Bevy website in the returned error message `... See: https://bevyengine.org/learn/errors/b0003`.

### Rust API guidelines

As a reference for our API development we are using the [Rust API guidelines][Rust API guidelines]. Generally, these should be followed, except for the following areas of disagreement:

#### Areas of disagreements

Some areas mentioned in the [Rust API guidelines][Rust API guidelines] we do not agree with. These areas will be expanded whenever we find something else we do not agree with, so be sure to check these from time to time.

> All items have a rustdoc example

- This guideline is too strong and not applicable for everything inside of the Bevy game engine. For functionality that requires more context or needs a more interactive demonstration (such as rendering or input features), make use of the `examples` folder instead.

> Examples use ?, not try!, not unwrap

- This guideline is usually reasonable, but not always required.

> Only smart pointers implement Deref and DerefMut

- Generally a good rule of thumb, but we're probably going to deliberately violate this for single-element wrapper types like `Life(u32)`. The behavior is still predictable and it significantly improves ergonomics / new user comprehension.

[Rust API guidelines]: https://rust-lang.github.io/api-guidelines/about.html

## Receiving and Responding To Reviews

Once you have opened your PR, your next task will be to shepherd it through community review so that it can be merged. Ideally it shouldn't be too long before a member of the community reviews your work, but sometimes PRs fall through the cracks. If your PR ends up sitting around for a few weeks with no interest, then don't be afraid to solicit reviews on the [Discord].

You may find that your reviewers sometimes misunderstand your work, ask for changes you disagree with, or request additional changes you aren't interested in making. If you find yourself disagreeing with a reviewer, it's fine to politely say "no" or indicate that their suggestion would be better left to a follow-up PR.

## Adopting pull requests

Occasionally authors of pull requests get busy or become unresponsive, or project members fail to reply in a timely manner.
This is a natural part of any open source project.
To avoid blocking these efforts, these pull requests may be *adopted*, where another contributor creates a new pull request with the same content.
If there is an old pull request that is without updates, comment to the organization whether it is appropriate to add the
*[S-Adopt-Me](https://github.com/bevyengine/bevy/labels/S-Adopt-Me)* label, to indicate that it can be *adopted*.
*S-Adopt-Me* PRs should be closed, and a tracking issue opened with the same labels to track their adoptions.

If you plan on adopting a PR yourself, mention so in the tracking issue.
For a PR that hasn't been yet marked as open for adoption, you can also leave a comment on the PR asking the author if they plan on returning.
If the author gives permission or simply doesn't respond after a few days, then it can be adopted.
This may sometimes even skip the labeling process since at that point the PR has been adopted by you.

With this label added, it's best practice to fork the original author's branch.
This ensures that they still get credit for working on it and that the commit history is retained.
When the new pull request is ready, it should reference the original PR in the description.
Then, notify org members to close the original.

* For example, you can reference the original PR by adding the following to your PR description: `Adopted #number-original-pull-request`

## Helping a PR get ready

Without going to the complete adoption of a PR, sometimes the author needs help to get it approved or passing CI.
Those PRs can be labeled as *[S-Needs-Help](https://github.com/bevyengine/bevy/labels/S-Needs-Help)*, and opening PRs on them is welcomed to fix the last few points, resolve conflicts to pass CI.
You will need to work closely with the original author or one of the maintainer to add your commits to the original PR.
