+++
title = "Release Process"
insert_anchor_links = "right"
[extra]
weight = 3
+++

Bevy uses three-months-long development cycles, delimited by a weeks-long rolling release process.

This means that Bevy releases a new version roughly once every three to four months. In an effort to avoid crunch and encourage sustainable development, we do not pick specific dates or deadlines.

## Development Cycles

During a three-month development cycle, Maintainers collect important PRs and issues into a GitHub Milestone. Issues included in the milestone are given priority, and the community generally makes a best-effort attempt to merge as many of these as possible over the duration of the cycle.

In the lead-up to a release (generally about about two and a half months after the previous) we begin to aggressively purge the milestone of things that seem unlikely to be ready in time. The main things that tend to remain are:

- Quick and easy features under active development.
- Fixes for important regressions or crashes.
- Large or impressive features that only need minor polish.

When the milestone is complete, we open a new working group to handle the release preparations, publish a release candidate (ex. `0.14.0-rc.1`) and begin the release process.

## Release Candidates

In the run-up to a release, Bevy publishes a new release candidate about once a week. Publishing release candidates allows users to identify bugs and regressions, gives ecosystem crates time to update so they can be ready closer to release day, and provides Bevy contributors extra time to write release notes and migration guides.

When publishing a release candidate, we also announce a general target release date. This is generally about one week out. If critical bugs or regressions are discovered during this testing window, we fix them and publish a new release candidate. Then we either

- Announce a new date, if we believe more testing time is needed than the current window, or
- Keep the same target release date if we believe there is enough time in the current window.

When we successfully hit a release candidate's target date, we do the final release.

## Release Checklist

When making a release, the Maintainers follow these checklists:

### Minor Version

#### Minor Pre-release

1. Check regressions tag.
2. Check appropriate milestone and close it.
3. Check GitHub Projects page for staleness.
4. Update change log.
5. Create migration guide.
6. Write blog post.
7. Update book.
8. Bump version number for all crates, using the "Release" workflow.
   1. Change the commit message to be nicer.
9. Create tag on GitHub.
10. Edit GitHub Release. Add links to the `Release announcement` and `Migration Guide`.
11. Bump `latest` tag to most recent release.
12. Run the [`update-screenshots` workflow] to update screenshots. *This will block blog post releases (and take ~40 minutes) so do it early*.
13. Run the [`build-wasm-examples` workflow] to update Wasm examples.

#### Minor Release

1. Release on crates.io using `bash tools/publish.sh`
2. Announce on:
    1. HackerNews
    2. Twitter
    3. Reddit: /r/bevy, /r/rust, /r/rust_gamedev
    4. Discord: Bevy, Game Development in Rust, Rust Programming Language Community
    5. This Month in Rust Game Development newsletter
    6. This Week in Rust newsletter

#### Minor Post-release

1. Bump version number for all crates to next versions, as `0.X-dev`, using the "Post-release version bump" workflow, to ensure properly displayed version for [Dev Docs](https://dev-docs.bevyengine.org/bevy/index.html).
2. Update Bevy version used for the Bevy Website's `learning-code-examples` tool (code example validation and formatting for the learning materials) to latest release.

### Patch

#### Patch Pre-release

1. Check appropriate milestone.
2. Close the milestone, open the next one if anything remains and transfer them.
3. Bump version number for all crates, using [the command from the "Release" workflow] locally, with `patch` for the new version.
    - Change the commit message to be nicer.
4. Create tag on GitHub.
5. Edit GitHub Release. Add link to the comparison between this patch and the previous version.
6. Bump `latest` tag to most recent release.
7. Run the [`update-screenshots` workflow] to update screenshots.
8. Run this [`build-wasm-examples` workflow] to update Wasm examples.

#### Patch Release

1. Release on crates.io
    - `bash tools/publish.sh`
2. Announce on:
    1. Discord: Bevy

#### Patch Post-Release

### Release Candidate

#### RC Pre-Release

1. Check appropriate milestone.
2. Create a branch for the release.
3. Bump version number for all crates, using [the command from the "Release" workflow] locally, with `rc` for the new version.
    - Change the commit message to be nicer.
4. Create tag on GitHub.
5. Edit GitHub Release. Add link to the comparison between this rc and the previous version.

#### RC Release

1. Release on crates.io
    - `bash tools/publish.sh`
2. Announce on:
    1. Discord: Bevy, #dev-announcements

#### RC Post-Release

1. Update Bevy version used for the Bevy Website's `learning-code-examples` tool (code example validation and formatting for the learning materials) to latest release.

[`update-screenshots` workflow]: https://github.com/bevyengine/bevy-website/actions/workflows/update-screenshots.yml
[`build-wasm-examples` workflow]: https://github.com/bevyengine/bevy-website/actions/workflows/build-wasm-examples.yml
[the command from the "Release" workflow]: https://github.com/bevyengine/bevy/blob/main/.github/workflows/release.yml
